//! Port TemplateEngine - Interface pour le moteur de templates

/// Trait pour le chargement et le rendu de templates
pub trait TemplateEngine: Send + Sync {
    /// Charge un template par nom
    fn load_template(&self, name: &str) -> Result<String, anyhow::Error>;

    /// Rend un template avec des donnees de contexte
    fn render(
        &self,
        template_name: &str,
        context: &serde_json::Value,
    ) -> Result<String, anyhow::Error>;

    /// Liste les templates disponibles
    fn list_templates(&self) -> Vec<String>;
}
