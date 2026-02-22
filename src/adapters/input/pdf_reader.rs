//! Adapter PdfReader - Extrait les User Stories depuis un fichier PDF

use async_trait::async_trait;
use std::path::Path;

use crate::domain::errors::InputError;
use crate::domain::user_story::{Language, UserStorySet};
use crate::ports::input_reader::InputReader;

use super::story_parser;

/// Lit les User Stories depuis un fichier PDF
pub struct PdfReader;

impl PdfReader {
    pub fn new() -> Self {
        Self
    }

    /// Extrait le texte brut depuis un fichier PDF
    fn extract_text(path: &Path) -> Result<String, InputError> {
        let bytes = std::fs::read(path).map_err(|e| InputError::ReadError(e.to_string()))?;

        pdf_extract::extract_text_from_mem(&bytes)
            .map_err(|e| InputError::ParseError(format!("Erreur extraction PDF: {e}")))
    }
}

impl Default for PdfReader {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl InputReader for PdfReader {
    async fn read_stories(&self, path: &Path) -> Result<UserStorySet, InputError> {
        if !path.exists() {
            return Err(InputError::FileNotFound {
                path: path.display().to_string(),
            });
        }

        super::check_file_size(path)?;

        let content = Self::extract_text(path)?;
        let language = self.detect_language(&content);
        let stories = story_parser::parse_stories(&content, language)?;

        Ok(UserStorySet {
            stories,
            source_files: vec![path.display().to_string()],
            language,
        })
    }

    fn detect_language(&self, content: &str) -> Language {
        story_parser::detect_language(content)
    }

    fn supported_extensions(&self) -> &[&str] {
        &["pdf"]
    }
}
