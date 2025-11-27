use crate::Properties;
use crate::note_memory_only::NoteMemoryOnly;
use obsidian_parser::prelude::*;
use petgraph::graph::DiGraph;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct NoteMetrics {
    /// Уникальный id
    id: usize,

    /// Количество слов в содержарнии заметки
    count_word_in_content: usize,

    /// Количество символов в содержарнии заметки
    count_symbols_in_content: usize,

    /// Количество yaml полей в свойствах заметки
    count_yaml_field: usize,

    /// Количество слов в названии заметки
    count_word_in_note_name: usize,

    /// Количество символов в названии заметки
    count_symbols_in_note_name: usize,

    /// Глубина заметки
    path_depth: usize,

    /// Размер путя до заметки
    path_len: usize,

    /// Количество `aliases`
    aliases: usize,

    /// Количество `todo`
    todos: usize,
}

impl NoteMetrics {
    pub fn new(note: &NoteMemoryOnly<Properties>, path_to_vault: &str) -> Self {
        let content = note.content().unwrap();
        let properties = note.properties().unwrap_or_default();
        let note_name = note.note_name().unwrap();
        let absolute_path = note.path().unwrap();
        let path = absolute_path.strip_prefix(&path_to_vault).unwrap();

        let aliases = {
            properties.as_ref().map(|properties| {
                let aliases = properties
                    .get("aliases")
                    .map(|todo| todo.as_sequence().into_iter().count());

                aliases.unwrap_or(0)
            })
        }
        .unwrap_or(0);

        let todos = {
            let todos_from_content = content.match_indices("#todo").count();

            let todos_from_yaml = properties
                .as_ref()
                .map(|properties| {
                    let todo = properties.get("tags").map(|todo| {
                        todo.as_sequence()
                            .into_iter()
                            .map(|v| v.iter().any(|x| x.as_str().is_some_and(|s| s == "todo")))
                            .count()
                    });

                    todo.unwrap_or(0)
                })
                .unwrap_or(0);

            todos_from_content + todos_from_yaml
        };

        Self {
            id: note.id(),
            count_word_in_content: content.split_whitespace().count(),
            count_symbols_in_content: content.len(),
            count_yaml_field: properties.unwrap_or_default().len(),
            count_word_in_note_name: note_name.split_whitespace().count(),
            count_symbols_in_note_name: note_name.len(),
            path_depth: path.components().count(),
            path_len: path.as_os_str().len(),
            aliases,
            todos,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Metrics {
    pub note_info: Vec<NoteMetrics>,
    //pub ungraph: UnGraph<NoteMetrics, ()>,
    pub digraph: DiGraph<NoteMetrics, ()>,
    pub count_duplicated_notes_by_name: usize,
}
