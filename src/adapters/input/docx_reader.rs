//! Adapter DocxReader - Extrait les User Stories depuis un fichier Word (.docx)
//!
//! Un fichier .docx est une archive ZIP contenant du XML.
//! Le texte se trouve dans `word/document.xml` dans les elements `<w:t>`.

use async_trait::async_trait;
use quick_xml::Reader;
use quick_xml::events::Event;
use std::io::Read;
use std::path::Path;

use crate::domain::errors::InputError;
use crate::domain::user_story::{Language, UserStorySet};
use crate::ports::input_reader::InputReader;

use super::story_parser;

/// Lit les User Stories depuis un fichier Word (.docx)
pub struct DocxReader;

impl DocxReader {
    pub fn new() -> Self {
        Self
    }

    /// Extrait le texte brut depuis un fichier .docx
    fn extract_text(path: &Path) -> Result<String, InputError> {
        let file = std::fs::File::open(path).map_err(|e| InputError::ReadError(e.to_string()))?;

        let mut archive = zip::ZipArchive::new(file)
            .map_err(|e| InputError::ParseError(format!("Erreur ouverture ZIP (docx): {e}")))?;

        // Lire word/document.xml
        let mut document_xml = String::new();
        {
            let mut doc_file = archive.by_name("word/document.xml").map_err(|e| {
                InputError::ParseError(format!("word/document.xml introuvable dans le .docx: {e}"))
            })?;
            doc_file
                .read_to_string(&mut document_xml)
                .map_err(|e| InputError::ReadError(e.to_string()))?;
        }

        // Parser le XML et extraire le texte des elements <w:t>
        Self::extract_text_from_xml(&document_xml)
    }

    /// Parse le XML du document Word et extrait le texte
    pub fn extract_text_from_xml(xml: &str) -> Result<String, InputError> {
        let mut reader = Reader::from_str(xml);
        let mut text = String::new();
        let mut in_text_element = false;
        let mut in_paragraph = false;
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                    let local_name = e.local_name();
                    match local_name.as_ref() {
                        b"p" => {
                            // Nouveau paragraphe : ajouter un saut de ligne
                            if !text.is_empty() && !text.ends_with('\n') {
                                text.push('\n');
                            }
                            in_paragraph = true;
                        }
                        b"t" if in_paragraph => {
                            in_text_element = true;
                        }
                        b"tab" if in_paragraph => {
                            text.push('\t');
                        }
                        b"br" if in_paragraph => {
                            text.push('\n');
                        }
                        _ => {}
                    }
                }
                Ok(Event::Text(ref e)) if in_text_element => {
                    let decoded = e.decode().map_err(|err| {
                        InputError::ParseError(format!("Erreur decodage XML: {err}"))
                    })?;
                    text.push_str(&decoded);
                }
                Ok(Event::GeneralRef(ref e)) if in_text_element => {
                    let name: &[u8] = e.as_ref();
                    match name {
                        b"amp" => text.push('&'),
                        b"lt" => text.push('<'),
                        b"gt" => text.push('>'),
                        b"apos" => text.push('\''),
                        b"quot" => text.push('"'),
                        _ => {}
                    }
                }
                Ok(Event::End(ref e)) => {
                    let local_name = e.local_name();
                    match local_name.as_ref() {
                        b"t" => in_text_element = false,
                        b"p" => in_paragraph = false,
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    return Err(InputError::ParseError(format!(
                        "Erreur parsing XML du .docx: {e}"
                    )));
                }
                _ => {}
            }
            buf.clear();
        }

        Ok(text)
    }
}

impl Default for DocxReader {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl InputReader for DocxReader {
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
        &["docx"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_text_from_xml() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p>
      <w:r><w:t>Recherche ISBN</w:t></w:r>
    </w:p>
    <w:p>
      <w:r><w:t>En tant que bibliothécaire, je veux rechercher un livre par ISBN afin de trouver rapidement un ouvrage.</w:t></w:r>
    </w:p>
    <w:p>
      <w:r><w:t>- Le champ accepte ISBN-10 et ISBN-13</w:t></w:r>
    </w:p>
  </w:body>
</w:document>"#;

        let text = DocxReader::extract_text_from_xml(xml).unwrap();
        assert!(text.contains("Recherche ISBN"));
        assert!(text.contains("En tant que"));
        assert!(text.contains("ISBN-10"));
    }

    #[test]
    fn test_extract_xml_with_tabs_and_breaks() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p>
      <w:r><w:t>Hello</w:t></w:r>
      <w:r><w:tab/></w:r>
      <w:r><w:t>World</w:t></w:r>
      <w:r><w:br/></w:r>
      <w:r><w:t>Next line</w:t></w:r>
    </w:p>
  </w:body>
</w:document>"#;

        let text = DocxReader::extract_text_from_xml(xml).unwrap();
        assert!(text.contains("Hello"));
        assert!(text.contains('\t'));
        assert!(text.contains("World"));
        assert!(text.contains("Next line"));
    }

    #[test]
    fn test_extract_xml_empty() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body></w:body>
</w:document>"#;

        let text = DocxReader::extract_text_from_xml(xml).unwrap();
        assert!(text.trim().is_empty());
    }

    #[test]
    fn test_extract_xml_with_escaped_entities() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p>
      <w:r><w:t>Test &amp; validation &lt;important&gt;</w:t></w:r>
    </w:p>
  </w:body>
</w:document>"#;

        let text = DocxReader::extract_text_from_xml(xml).unwrap();
        assert!(text.contains("Test & validation <important>"));
    }

    #[test]
    fn test_extract_xml_multiple_paragraphs() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p><w:r><w:t>Paragraph 1</w:t></w:r></w:p>
    <w:p><w:r><w:t>Paragraph 2</w:t></w:r></w:p>
    <w:p><w:r><w:t>Paragraph 3</w:t></w:r></w:p>
  </w:body>
</w:document>"#;

        let text = DocxReader::extract_text_from_xml(xml).unwrap();
        let paragraphs: Vec<&str> = text.lines().filter(|l| !l.is_empty()).collect();
        assert_eq!(paragraphs.len(), 3);
    }

    mod proptest_suite {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn extract_xml_never_panics(input in "\\PC*") {
                let _ = DocxReader::extract_text_from_xml(&input);
            }
        }
    }
}
