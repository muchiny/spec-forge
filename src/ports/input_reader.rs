//! Port InputReader - Interface pour la lecture des User Stories

use async_trait::async_trait;
use std::path::Path;

use crate::domain::errors::InputError;
use crate::domain::user_story::{Language, UserStorySet};

/// Trait pour la lecture des user stories depuis differents formats
#[async_trait]
pub trait InputReader: Send + Sync {
    /// Lit les user stories depuis un fichier
    async fn read_stories(&self, path: &Path) -> Result<UserStorySet, InputError>;

    /// Detecte la langue du contenu
    fn detect_language(&self, content: &str) -> Language;

    /// Extensions de fichier supportees
    fn supported_extensions(&self) -> &[&str];
}
