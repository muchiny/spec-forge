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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_pipeline_stage_labels() {
        assert_eq!(PipelineStage::ReadingInput.label(), "Lecture du fichier");
        assert_eq!(PipelineStage::RefiningSpec.label(), "Raffinement LLM");
        assert_eq!(
            PipelineStage::GeneratingTests.label(),
            "Generation des tests"
        );
        assert_eq!(PipelineStage::WritingOutput.label(), "Ecriture des sorties");
    }

    #[test]
    fn test_pipeline_stage_equality() {
        assert_eq!(PipelineStage::ReadingInput, PipelineStage::ReadingInput);
        assert_ne!(PipelineStage::ReadingInput, PipelineStage::WritingOutput);
    }

    #[test]
    fn test_pipeline_event_stage_started() {
        let event = PipelineEvent::StageStarted(PipelineStage::RefiningSpec);
        assert!(matches!(
            event,
            PipelineEvent::StageStarted(PipelineStage::RefiningSpec)
        ));
    }

    #[test]
    fn test_pipeline_event_progress() {
        let event = PipelineEvent::Progress {
            stage: PipelineStage::GeneratingTests,
            message: "Batch 1/3".into(),
        };
        if let PipelineEvent::Progress { stage, message } = event {
            assert_eq!(stage, PipelineStage::GeneratingTests);
            assert_eq!(message, "Batch 1/3");
        } else {
            panic!("Pattern matching echoue");
        }
    }

    #[test]
    fn test_pipeline_event_llm_call() {
        let started = PipelineEvent::LlmCallStarted { prompt_tokens: 500 };
        assert!(matches!(
            started,
            PipelineEvent::LlmCallStarted { prompt_tokens: 500 }
        ));

        let completed = PipelineEvent::LlmCallCompleted {
            response_tokens: 1200,
            elapsed_ms: 3500,
        };
        if let PipelineEvent::LlmCallCompleted {
            response_tokens,
            elapsed_ms,
        } = completed
        {
            assert_eq!(response_tokens, 1200);
            assert_eq!(elapsed_ms, 3500);
        } else {
            panic!("Pattern matching echoue");
        }
    }

    #[test]
    fn test_pipeline_event_file_written() {
        let event = PipelineEvent::FileWritten {
            path: PathBuf::from("/tmp/output/spec.md"),
        };
        if let PipelineEvent::FileWritten { path } = event {
            assert_eq!(path, PathBuf::from("/tmp/output/spec.md"));
        } else {
            panic!("Pattern matching echoue");
        }
    }

    #[test]
    fn test_pipeline_event_error() {
        let event = PipelineEvent::Error("Connexion LLM echouee".into());
        assert!(matches!(event, PipelineEvent::Error(_)));
    }
}
