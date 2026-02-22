//! Evenements de progression du pipeline pour la TUI

use std::path::PathBuf;

use crate::domain::specification::Specification;
use crate::domain::test_case::TestSuite;

/// Etape du pipeline
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineStage {
    ReadingInput,
    RefiningSpec,
    GeneratingTests,
    WritingOutput,
}

impl PipelineStage {
    pub fn label(&self) -> &'static str {
        match self {
            PipelineStage::ReadingInput => "Lecture du fichier",
            PipelineStage::RefiningSpec => "Raffinement LLM",
            PipelineStage::GeneratingTests => "Generation des tests",
            PipelineStage::WritingOutput => "Ecriture des sorties",
        }
    }
}

/// Evenement emis par le pipeline pendant l'execution
#[derive(Debug, Clone)]
pub enum PipelineEvent {
    /// Une etape demarre
    StageStarted(PipelineStage),

    /// Une etape est terminee
    StageCompleted(PipelineStage),

    /// Message de progression
    Progress {
        stage: PipelineStage,
        message: String,
    },

    /// Appel LLM commence
    LlmCallStarted { prompt_tokens: usize },

    /// Appel LLM termine
    LlmCallCompleted {
        response_tokens: usize,
        elapsed_ms: u64,
    },

    /// Fichier ecrit sur disque
    FileWritten { path: PathBuf },

    /// Erreur
    Error(String),

    /// Pipeline termine avec succes — contient les resultats
    Completed {
        spec: Box<Specification>,
        test_suite: Box<TestSuite>,
        feature_contents: Vec<String>,
        traceability_content: Option<String>,
    },
}
