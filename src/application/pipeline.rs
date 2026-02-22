//! Orchestrateur du pipeline complet US -> Spec -> Tests

use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::info;

use crate::adapters::input::docx_reader::DocxReader;
use crate::adapters::input::markdown_reader::MarkdownReader;
use crate::adapters::input::pdf_reader::PdfReader;
use crate::adapters::input::yaml_reader::YamlReader;
use crate::adapters::output::gherkin_writer::GherkinWriter;
use crate::adapters::output::markdown_writer::MarkdownWriter;
use crate::adapters::output::traceability_writer::TraceabilityWriter;
use crate::application::generate_tests_service::GenerateTestsService;
use crate::application::refine_service::RefineService;
use crate::domain::errors::DomainError;
use crate::domain::specification::Specification;
use crate::domain::test_case::TestSuite;
use crate::domain::user_story::{Language, UserStorySet};
use crate::infrastructure::config::Config;
use crate::ports::input_reader::InputReader;
use crate::ports::llm_service::LlmService;
use crate::ports::template_engine::TemplateEngine;

/// Resultat du pipeline complet
pub struct PipelineResult {
    pub specification: Specification,
    pub test_suite: TestSuite,
    pub spec_path: std::path::PathBuf,
    pub feature_paths: Vec<std::path::PathBuf>,
    pub traceability_path: Option<std::path::PathBuf>,
}

/// Orchestrateur du pipeline
pub struct Pipeline {
    refine_service: RefineService,
    generate_service: GenerateTestsService,
    config: Config,
}

impl Pipeline {
    pub fn new(
        llm: Arc<dyn LlmService>,
        templates: Arc<dyn TemplateEngine>,
        config: Config,
    ) -> Self {
        let language = Language::from_code(&config.output.gherkin_language);

        let refine_service = RefineService::new(
            Arc::clone(&llm),
            Arc::clone(&templates),
            config.pipeline.max_retries,
        )
        .with_token_budget(config.pipeline.token_budget);

        let generate_service = GenerateTestsService::new(
            Arc::clone(&llm),
            Arc::clone(&templates),
            language,
            config.pipeline.max_retries,
        )
        .with_token_budget(config.pipeline.token_budget);

        Self {
            refine_service,
            generate_service,
            config,
        }
    }

    /// Retourne la configuration du pipeline
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Lit les user stories depuis un fichier (detecte le format)
    pub async fn read_stories(&self, input_path: &Path) -> Result<UserStorySet, DomainError> {
        let extension = input_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        let story_set = match extension {
            "yaml" | "yml" => {
                let reader = YamlReader::new();
                reader.read_stories(input_path).await?
            }
            "pdf" => {
                let reader = PdfReader::new();
                reader.read_stories(input_path).await?
            }
            "docx" => {
                let reader = DocxReader::new();
                reader.read_stories(input_path).await?
            }
            _ => {
                let reader = MarkdownReader::new();
                reader.read_stories(input_path).await?
            }
        };

        info!(
            count = story_set.stories.len(),
            file = %input_path.display(),
            "User stories chargees"
        );

        Ok(story_set)
    }

    /// Extensions de fichiers supportees
    const SUPPORTED_EXTENSIONS: &'static [&'static str] =
        &["md", "markdown", "yaml", "yml", "pdf", "docx"];

    /// Expande les chemins : dossiers → fichiers supportes, fichiers → tels quels
    pub fn expand_paths(paths: &[PathBuf]) -> Result<Vec<PathBuf>, DomainError> {
        let mut result = Vec::new();
        for path in paths {
            if path.is_dir() {
                let entries = std::fs::read_dir(path).map_err(|e| {
                    DomainError::Input(crate::domain::errors::InputError::ReadError(format!(
                        "Impossible de lire le dossier {}: {}",
                        path.display(),
                        e
                    )))
                })?;
                for entry in entries {
                    let entry = entry.map_err(|e| {
                        DomainError::Input(crate::domain::errors::InputError::ReadError(
                            e.to_string(),
                        ))
                    })?;
                    let entry_path = entry.path();
                    if entry_path.is_file()
                        && let Some(ext) = entry_path.extension().and_then(|e| e.to_str())
                        && Self::SUPPORTED_EXTENSIONS.contains(&ext)
                    {
                        result.push(entry_path);
                    }
                }
            } else {
                result.push(path.clone());
            }
        }
        result.sort();
        Ok(result)
    }

    /// Lit les user stories depuis plusieurs fichiers et les fusionne
    pub async fn read_stories_multi(&self, paths: &[PathBuf]) -> Result<UserStorySet, DomainError> {
        let expanded = Self::expand_paths(paths)?;
        if expanded.is_empty() {
            return Err(DomainError::Input(
                crate::domain::errors::InputError::NoStoriesFound,
            ));
        }

        let mut sets = Vec::new();
        for path in &expanded {
            let set = self.read_stories(path).await?;
            sets.push(set);
        }

        let merged = UserStorySet::merge(sets);
        info!(
            count = merged.stories.len(),
            files = expanded.len(),
            "User stories fusionnees depuis {} fichier(s)",
            expanded.len()
        );
        Ok(merged)
    }

    /// Etape 1 : Raffiner les US en specification (lit les fichiers puis raffine)
    pub async fn refine(
        &self,
        input_paths: &[PathBuf],
        output_dir: &Path,
        constitution: Option<&str>,
    ) -> Result<Specification, DomainError> {
        let story_set = self.read_stories_multi(input_paths).await?;
        self.refine_stories(&story_set, output_dir, constitution)
            .await
    }

    /// Raffine un UserStorySet deja charge en specification (sans re-lecture)
    pub async fn refine_stories(
        &self,
        story_set: &UserStorySet,
        output_dir: &Path,
        constitution: Option<&str>,
    ) -> Result<Specification, DomainError> {
        let spec = self.refine_service.refine(story_set, constitution).await?;

        // Ecrire la spec
        let writer = MarkdownWriter::new();
        let spec_path = writer.write(&spec, output_dir).await.map_err(|e| {
            DomainError::Refinement(crate::domain::errors::RefinementError::LlmFailed {
                details: e.to_string(),
            })
        })?;

        info!(path = %spec_path.display(), "Specification ecrite");

        Ok(spec)
    }

    /// Etape 2 : Generer les tests depuis une specification
    pub async fn generate_tests(
        &self,
        spec: &Specification,
        output_dir: &Path,
    ) -> Result<TestSuite, DomainError> {
        let suite = self.generate_service.generate(spec).await?;

        // Ecrire les fichiers .feature
        let language = match self.config.output.gherkin_language.as_str() {
            "en" => Language::English,
            _ => Language::French,
        };
        let writer = GherkinWriter::new(language);

        for feature in &suite.features {
            let path = writer
                .write_feature(feature, output_dir)
                .await
                .map_err(|e| {
                    DomainError::Generation(crate::domain::errors::GenerationError::GherkinFailed {
                        details: e.to_string(),
                    })
                })?;
            info!(path = %path.display(), "Feature ecrite");
        }

        Ok(suite)
    }

    /// Pipeline complet : US -> Spec -> Tests
    pub async fn run_full(
        &self,
        input_paths: &[PathBuf],
        output_dir: &Path,
        constitution: Option<&str>,
    ) -> Result<PipelineResult, DomainError> {
        info!("Demarrage du pipeline complet");

        let specs_dir = output_dir.join("specs");
        let features_dir = output_dir.join("features");

        // Etape 1: Raffinement
        let story_set = self.read_stories_multi(input_paths).await?;
        let spec = self.refine_service.refine(&story_set, constitution).await?;

        let md_writer = MarkdownWriter::new();
        let spec_path = md_writer.write(&spec, &specs_dir).await.map_err(|e| {
            DomainError::Refinement(crate::domain::errors::RefinementError::LlmFailed {
                details: e.to_string(),
            })
        })?;
        info!(path = %spec_path.display(), "Specification ecrite");

        // Etape 2: Generation de tests
        let suite = self.generate_service.generate(&spec).await?;

        let language = match self.config.output.gherkin_language.as_str() {
            "en" => Language::English,
            _ => Language::French,
        };
        let gherkin_writer = GherkinWriter::new(language);
        let mut feature_paths = Vec::new();
        for feature in &suite.features {
            let path = gherkin_writer
                .write_feature(feature, &features_dir)
                .await
                .map_err(|e| {
                    DomainError::Generation(crate::domain::errors::GenerationError::GherkinFailed {
                        details: e.to_string(),
                    })
                })?;
            feature_paths.push(path);
        }

        // Tracabilite
        let traceability_path = if self.config.output.traceability {
            let trace_writer = TraceabilityWriter::new();
            let path = trace_writer
                .write(&spec, &suite, output_dir)
                .await
                .map_err(|e| {
                    DomainError::Generation(crate::domain::errors::GenerationError::GherkinFailed {
                        details: e.to_string(),
                    })
                })?;
            info!(path = %path.display(), "Matrice de tracabilite ecrite");
            Some(path)
        } else {
            None
        };

        info!(
            specs = spec.user_scenarios.len(),
            requirements = spec.functional_requirements.len(),
            features = suite.features.len(),
            scenarios = suite.total_scenarios,
            "Pipeline termine avec succes"
        );

        Ok(PipelineResult {
            specification: spec,
            test_suite: suite,
            spec_path,
            feature_paths,
            traceability_path,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_expand_paths_single_file() {
        let dir = TempDir::new().unwrap();
        let file = dir.path().join("story.md");
        fs::write(&file, "# US").unwrap();

        let result = Pipeline::expand_paths(&[file.clone()]).unwrap();
        assert_eq!(result, vec![file]);
    }

    #[test]
    fn test_expand_paths_directory_mixed_files() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("a.md"), "# US A").unwrap();
        fs::write(dir.path().join("b.yaml"), "stories: []").unwrap();
        fs::write(dir.path().join("c.txt"), "pas supporte").unwrap(); // ignore
        fs::write(dir.path().join("d.pdf"), "fake pdf").unwrap();

        let result = Pipeline::expand_paths(&[dir.path().to_path_buf()]).unwrap();
        // .md, .yaml, .pdf sont supportes, .txt non. Resultat trie.
        assert_eq!(result.len(), 3);
        let names: Vec<String> = result
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();
        assert_eq!(names, vec!["a.md", "b.yaml", "d.pdf"]);
    }

    #[test]
    fn test_expand_paths_empty_directory() {
        let dir = TempDir::new().unwrap();
        let result = Pipeline::expand_paths(&[dir.path().to_path_buf()]).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_expand_paths_nested_not_recursive() {
        let dir = TempDir::new().unwrap();
        let sub = dir.path().join("subdir");
        fs::create_dir(&sub).unwrap();
        fs::write(sub.join("nested.md"), "# US").unwrap();
        fs::write(dir.path().join("top.md"), "# US").unwrap();

        let result = Pipeline::expand_paths(&[dir.path().to_path_buf()]).unwrap();
        // Seulement top.md, pas nested.md (pas de recursion)
        assert_eq!(result.len(), 1);
        assert!(result[0].file_name().unwrap() == "top.md");
    }
}
