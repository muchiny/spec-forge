//! Adapter MarkdownReader - Parse les User Stories depuis un fichier Markdown

use async_trait::async_trait;
use std::path::Path;

use crate::domain::errors::InputError;
use crate::domain::user_story::{Language, UserStorySet};
use crate::ports::input_reader::InputReader;

use super::story_parser;

/// Lit les User Stories depuis un fichier Markdown
pub struct MarkdownReader;

impl MarkdownReader {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MarkdownReader {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl InputReader for MarkdownReader {
    async fn read_stories(&self, path: &Path) -> Result<UserStorySet, InputError> {
        if !path.exists() {
            return Err(InputError::FileNotFound {
                path: path.display().to_string(),
            });
        }

        super::check_file_size(path)?;

        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| InputError::ReadError(e.to_string()))?;

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
        &["md", "markdown"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_language_french() {
        let reader = MarkdownReader::new();
        let content = "En tant que bibliothécaire, je veux rechercher un livre afin de le prêter.";
        assert_eq!(reader.detect_language(content), Language::French);
    }

    #[test]
    fn test_detect_language_english() {
        let reader = MarkdownReader::new();
        let content = "As a user, I want to search for a book so that I can borrow it.";
        assert_eq!(reader.detect_language(content), Language::English);
    }
}
