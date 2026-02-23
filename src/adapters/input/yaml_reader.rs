//! Adapter YamlReader - Parse les User Stories depuis un fichier YAML

use async_trait::async_trait;
use std::path::Path;

use crate::domain::errors::InputError;
use crate::domain::user_story::{Language, UserStorySet};
use crate::ports::input_reader::InputReader;

/// Lit les User Stories depuis un fichier YAML structure
pub struct YamlReader;

impl YamlReader {
    pub fn new() -> Self {
        Self
    }
}

impl Default for YamlReader {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl InputReader for YamlReader {
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

        let story_set: UserStorySet =
            serde_yaml::from_str(&content).map_err(|e| InputError::InvalidFormat {
                details: format!("Erreur YAML: {}", e),
            })?;

        if story_set.stories.is_empty() {
            return Err(InputError::NoStoriesFound);
        }

        Ok(story_set)
    }

    fn detect_language(&self, content: &str) -> Language {
        if content.contains("language: en") || content.contains("language: EN") {
            Language::English
        } else {
            Language::French
        }
    }

    fn supported_extensions(&self) -> &[&str] {
        &["yaml", "yml"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_read_yaml_stories() {
        let yaml_content = r#"
language: fr
stories:
  - id: "550e8400-e29b-41d4-a716-446655440000"
    title: "Recherche ISBN"
    actor: "bibliothécaire"
    action: "rechercher un livre par ISBN"
    benefit: "trouver rapidement un ouvrage"
    priority: P1
    acceptance_criteria:
      - "Le champ accepte ISBN-10 et ISBN-13"
      - "Résultats en moins de 2 secondes"
"#;
        let mut file = NamedTempFile::with_suffix(".yaml").unwrap();
        write!(file, "{}", yaml_content).unwrap();

        let reader = YamlReader::new();
        let result = reader.read_stories(file.path()).await;
        assert!(result.is_ok());
        let set = result.unwrap();
        assert_eq!(set.stories.len(), 1);
        assert_eq!(set.stories[0].title, "Recherche ISBN");
        assert_eq!(set.stories[0].acceptance_criteria.len(), 2);
    }

    #[tokio::test]
    async fn test_read_yaml_file_not_found() {
        let reader = YamlReader::new();
        let result = reader.read_stories(Path::new("/nonexistent.yaml")).await;
        assert!(matches!(result, Err(InputError::FileNotFound { .. })));
    }

    #[tokio::test]
    async fn test_read_yaml_invalid_syntax() {
        let mut file = NamedTempFile::with_suffix(".yaml").unwrap();
        write!(file, "ceci: n'est: pas: du: yaml: [valide").unwrap();

        let reader = YamlReader::new();
        let result = reader.read_stories(file.path()).await;
        assert!(result.is_err(), "YAML invalide DOIT echouer");
        assert!(
            matches!(result.unwrap_err(), InputError::InvalidFormat { .. }),
            "Attendu InvalidFormat"
        );
    }

    #[tokio::test]
    async fn test_read_yaml_empty_stories() {
        let yaml_content = r#"
language: fr
stories: []
"#;
        let mut file = NamedTempFile::with_suffix(".yaml").unwrap();
        write!(file, "{}", yaml_content).unwrap();

        let reader = YamlReader::new();
        let result = reader.read_stories(file.path()).await;
        assert!(result.is_err(), "Stories vides DOIT echouer");
        assert!(matches!(result.unwrap_err(), InputError::NoStoriesFound));
    }

    #[test]
    fn test_yaml_reader_detect_language() {
        let reader = YamlReader::new();
        assert_eq!(reader.detect_language("language: en"), Language::English);
        assert_eq!(reader.detect_language("language: fr"), Language::French);
        assert_eq!(reader.detect_language("no language tag"), Language::French);
    }

    #[test]
    fn test_yaml_reader_supported_extensions() {
        let reader = YamlReader::new();
        assert_eq!(reader.supported_extensions(), &["yaml", "yml"]);
    }

    #[test]
    fn test_yaml_reader_default() {
        let _reader = YamlReader;
    }
}
