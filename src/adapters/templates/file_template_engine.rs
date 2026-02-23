//! Adapter FileTemplateEngine - Charge et rend les templates Handlebars

use handlebars::Handlebars;
use std::path::{Path, PathBuf};
use tracing::debug;

use crate::ports::template_engine::TemplateEngine;

/// Moteur de templates base sur les fichiers Handlebars
pub struct FileTemplateEngine {
    handlebars: Handlebars<'static>,
    template_dir: PathBuf,
}

impl FileTemplateEngine {
    /// Cree un moteur de templates depuis un repertoire
    pub fn new(template_dir: &Path) -> Result<Self, anyhow::Error> {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(false);

        // Charger tous les templates .md du repertoire
        if template_dir.exists() {
            for entry in std::fs::read_dir(template_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().is_some_and(|ext| ext == "md")
                    && path
                        .file_stem()
                        .is_some_and(|name| !name.eq_ignore_ascii_case("readme"))
                {
                    let name = path
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    let content = std::fs::read_to_string(&path)?;
                    handlebars.register_template_string(&name, &content)?;
                    debug!(template = %name, "Template charge");
                }
            }
        }

        Ok(Self {
            handlebars,
            template_dir: template_dir.to_path_buf(),
        })
    }
}

impl TemplateEngine for FileTemplateEngine {
    fn load_template(&self, name: &str) -> Result<String, anyhow::Error> {
        let path = self.template_dir.join(format!("{}.md", name));

        // Securite : verifier que le chemin reste dans le repertoire de templates
        let canonical_dir = self
            .template_dir
            .canonicalize()
            .map_err(|e| anyhow::anyhow!("Repertoire templates invalide: {}", e))?;
        let canonical_path = path
            .canonicalize()
            .map_err(|e| anyhow::anyhow!("Template '{}' non trouve: {}", name, e))?;
        if !canonical_path.starts_with(&canonical_dir) {
            anyhow::bail!(
                "Acces interdit: le template '{}' sort du repertoire autorise",
                name
            );
        }

        let content = std::fs::read_to_string(&canonical_path)
            .map_err(|e| anyhow::anyhow!("Template '{}' non trouve: {}", name, e))?;
        Ok(content)
    }

    fn render(
        &self,
        template_name: &str,
        context: &serde_json::Value,
    ) -> Result<String, anyhow::Error> {
        self.handlebars
            .render(template_name, context)
            .map_err(|e| anyhow::anyhow!("Erreur de rendu du template '{}': {}", template_name, e))
    }

    fn list_templates(&self) -> Vec<String> {
        self.handlebars.get_templates().keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_load_templates() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("test_template.md"), "Hello {{name}}!").unwrap();

        let engine = FileTemplateEngine::new(dir.path()).unwrap();
        let templates = engine.list_templates();
        assert!(templates.contains(&"test_template".to_string()));
    }

    #[test]
    fn test_render_template() {
        let dir = TempDir::new().unwrap();
        fs::write(
            dir.path().join("greeting.md"),
            "Bonjour {{name}}, bienvenue dans {{project}}!",
        )
        .unwrap();

        let engine = FileTemplateEngine::new(dir.path()).unwrap();
        let context = serde_json::json!({
            "name": "Alice",
            "project": "spec-forge"
        });
        let result = engine.render("greeting", &context).unwrap();
        assert_eq!(result, "Bonjour Alice, bienvenue dans spec-forge!");
    }

    #[test]
    fn test_path_traversal_blocked() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("legit.md"), "Contenu ok").unwrap();

        let engine = FileTemplateEngine::new(dir.path()).unwrap();

        // Tentative de path traversal
        let result = engine.load_template("../../etc/passwd");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("Acces interdit") || err_msg.contains("non trouve"),
            "Message d'erreur inattendu: {err_msg}"
        );
    }

    #[test]
    fn test_render_with_array() {
        let dir = TempDir::new().unwrap();
        fs::write(
            dir.path().join("list.md"),
            "Items:\n{{#each items}}\n- {{this}}\n{{/each}}",
        )
        .unwrap();

        let engine = FileTemplateEngine::new(dir.path()).unwrap();
        let context = serde_json::json!({
            "items": ["un", "deux", "trois"]
        });
        let result = engine.render("list", &context).unwrap();
        assert!(result.contains("- un"));
        assert!(result.contains("- trois"));
    }
}
