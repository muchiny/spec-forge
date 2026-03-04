//! Evenements Tauri emis vers le frontend

/// Evenement de progression du pipeline
pub const EVENT_PIPELINE_PROGRESS: &str = "pipeline-progress";

/// Evenement de changement du statut LLM
pub const EVENT_LLM_STATUS: &str = "llm-status";

/// Payload de progression du pipeline
#[derive(serde::Serialize, Clone)]
pub struct PipelineProgressPayload {
    pub stage: String,
    pub message: String,
    pub progress_pct: Option<f32>,
}
