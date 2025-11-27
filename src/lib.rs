pub mod metrics;
mod note_memory_only;

use crate::note_memory_only::NoteMemoryOnly;
use metrics::{Metrics, NoteMetrics};
use obsidian_parser::prelude::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

pub type Properties = HashMap<String, serde_yml::Value>;

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

    //let ungraph = vault.get_ungraph().unwrap();
    let digraph = vault.get_digraph().unwrap();

    let metrics = Metrics {
        note_info: vault
            .notes()
            .iter()
            .map(|note| NoteMetrics::new(note, &path_to_vault))
            .collect(),
        //ungraph: ungraph.map_owned(|_, note| NoteMetrics::new(note, &path_to_vault), |_, _| {}),
        digraph: digraph.map_owned(|_, note| NoteMetrics::new(note, &path_to_vault), |_, _| {}),
        count_duplicated_notes_by_name: vault.get_duplicates_notes_by_name().len(),
    };

    serde_json::to_string(&metrics).unwrap_or("ERROR serde".to_string())
}
