use obsidian_parser::{
    note::parser::{self, ResultParse, parse_note},
    prelude::*,
};
use serde::de::DeserializeOwned;
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct NoteMemoryOnly<T>
where
    T: DeserializeOwned + Clone,
{
    id: usize,
    content: String,
    properties: Option<T>,
    path: PathBuf,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid frontmatter format")]
    InvalidFormat(#[from] parser::Error),

    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yml::Error),
}

impl<T> Note for NoteMemoryOnly<T>
where
    T: DeserializeOwned + Clone,
{
    type Properties = T;
    type Error = self::Error;

    fn content(&self) -> Result<std::borrow::Cow<'_, str>, Self::Error> {
        Ok(Cow::Borrowed(&self.content))
    }

    fn properties(&self) -> Result<Option<Cow<'_, Self::Properties>>, Self::Error> {
        Ok(self.properties.as_ref().map(|p| Cow::Borrowed(p)))
    }

    fn path(&self) -> Option<Cow<'_, std::path::Path>> {
        Some(Cow::Borrowed(&self.path))
    }
}

impl<T> NoteMemoryOnly<T>
where
    T: DeserializeOwned + Clone,
{
    pub fn from_string(
        raw_text: impl AsRef<str>,
        path: impl AsRef<Path>,
        id: usize,
    ) -> Result<Self, self::Error> {
        let raw_text = raw_text.as_ref();
        let path = path.as_ref().to_path_buf();

        match parse_note(raw_text)? {
            ResultParse::WithProperties {
                content,
                properties,
            } => Ok(Self {
                content: content.to_string(),
                properties: Some(serde_yml::from_str(properties)?),
                path,
                id,
            }),
            ResultParse::WithoutProperties => Ok(Self {
                content: raw_text.to_string(),
                properties: None,
                path,
                id,
            }),
        }
    }

    #[inline]
    pub fn id(&self) -> usize {
        self.id
    }
}
