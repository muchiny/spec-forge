//! Port OutputWriter - Interface pour l'ecriture des sorties

use async_trait::async_trait;
use std::path::{Path, PathBuf};

use crate::domain::specification::Specification;
use crate::domain::test_case::TestSuite;

/// Trait pour l'ecriture des specifications et tests
#[async_trait]
pub trait OutputWriter: Send + Sync {
    /// Ecrit une specification raffinee en Markdown
    async fn write_specification(
        &self,
        spec: &Specification,
        output_dir: &Path,
    ) -> Result<PathBuf, anyhow::Error>;

    /// Ecrit les fichiers .feature Gherkin
    async fn write_features(
        &self,
        test_suite: &TestSuite,
        output_dir: &Path,
    ) -> Result<Vec<PathBuf>, anyhow::Error>;

    /// Ecrit le rapport de tracabilite
    async fn write_traceability(
        &self,
        spec: &Specification,
        test_suite: &TestSuite,
        output_dir: &Path,
    ) -> Result<PathBuf, anyhow::Error>;
}
