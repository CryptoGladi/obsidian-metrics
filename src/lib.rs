mod note_memory_only;

use crate::note_memory_only::NoteMemoryOnly;
use obsidian_parser::prelude::*;
use petgraph::graph::{DiGraph, UnGraph};
use serde::Serialize;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize)]
struct NoteMetrics {
    id: usize,
    count_word_in_content: usize,
    count_symbols_in_content: usize,
    count_yaml_field: usize,
    count_word_in_note_name: usize,
    count_symbols_in_note_name: usize,
    path_depth: usize,
    path_len: usize,
}

impl NoteMetrics {
    pub fn new(
        note: NoteMemoryOnly<HashMap<String, serde_yml::Value>>,
        path_to_vault: &str,
    ) -> Self {
        let content = note.content().unwrap();
        let properties = note.properties().unwrap_or_default();
        let note_name = note.note_name().unwrap();
        let absolute_path = note.path().unwrap();
        let path = absolute_path.strip_prefix(&path_to_vault).unwrap();

        Self {
            id: note.id(),
            count_word_in_content: content.split_whitespace().count(),
            count_symbols_in_content: content.len(),
            count_yaml_field: properties.unwrap_or_default().len(),
            count_word_in_note_name: note_name.split_whitespace().count(),
            count_symbols_in_note_name: note_name.len(),
            path_depth: path.components().count(),
            path_len: path.as_os_str().len(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct Metrics {
    note_info: Vec<NoteMetrics>,
    ungraph: UnGraph<NoteMetrics, ()>,
    digraph: DiGraph<NoteMetrics, ()>,
    count_duplicated_notes: usize,
}

#[wasm_bindgen]
pub struct NoteInfo {
    full_text: String,
    path: String,
}

#[wasm_bindgen]
impl NoteInfo {
    #[wasm_bindgen(constructor)]
    pub fn new(full_text: String, path: String) -> Self {
        Self { full_text, path }
    }
}

#[wasm_bindgen]
pub fn get_json_metrics(notes: Vec<NoteInfo>, path_to_vault: String) -> String {
    //wasm_log::init(Config::default());
    let vault = notes
        .into_iter()
        .enumerate()
        .map(|(id, note)| NoteMemoryOnly::from_string(note.full_text, note.path, id))
        .filter_map(Result::ok)
        .build_vault(&VaultOptions::new(&path_to_vault));

    let ungraph = vault.get_ungraph().unwrap();
    let digraph = vault.get_digraph().unwrap();

    let metrics = Metrics {
        note_info: vault
            .notes()
            .clone()
            .into_iter()
            .map(|note| NoteMetrics::new(note, &path_to_vault))
            .collect(),
        ungraph: ungraph.map_owned(
            |_, note| NoteMetrics::new(note.clone(), &path_to_vault),
            |_, _| {},
        ),
        digraph: digraph.map_owned(
            |_, note| NoteMetrics::new(note.clone(), &path_to_vault),
            |_, _| {},
        ),
        count_duplicated_notes: vault.get_duplicates_notes_by_name().len(),
    };

    serde_json::to_string(&metrics).unwrap_or("ERROR serde".to_string())
}
