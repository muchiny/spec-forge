//! Evenements Tauri emis vers le frontend

/// Evenement de progression du pipeline
pub const EVENT_PIPELINE_PROGRESS: &str = "pipeline-progress";

/// Evenement de changement du statut LLM
pub const EVENT_LLM_STATUS: &str = "llm-status";

/// Evenement de progression du pull de modele
pub const EVENT_MODEL_PULL_PROGRESS: &str = "model-pull-progress";

/// Payload de progression du pipeline
#[derive(serde::Serialize, Clone)]
pub struct PipelineProgressPayload {
    pub stage: String,
    pub message: String,
    pub progress_pct: Option<f32>,
}

/// Payload de progression du pull de modele
#[derive(serde::Serialize, Clone)]
pub struct ModelPullProgressPayload {
    pub status: String,
    pub completed: Option<u64>,
    pub total: Option<u64>,
    pub digest: Option<String>,
}
