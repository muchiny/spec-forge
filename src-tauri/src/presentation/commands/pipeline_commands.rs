//! Commandes du pipeline — execution et tracabilite

use std::path::PathBuf;

use serde::Serialize;
use tauri::{AppHandle, Emitter, State};

use spec_forge::domain::traceability::build_traceability_matrix;

use crate::presentation::events::{PipelineProgressPayload, EVENT_PIPELINE_PROGRESS};
use crate::presentation::state::AppState;

/// Reponse du pipeline complet
#[derive(Serialize)]
pub struct PipelineResultResponse {
    pub specification: serde_json::Value,
    pub test_suite: serde_json::Value,
    pub traceability: serde_json::Value,
    pub spec_path: String,
    pub feature_paths: Vec<String>,
    pub traceability_path: Option<String>,
}

/// Execute le pipeline complet : US -> Spec -> Tests -> Tracabilite
#[tauri::command]
pub async fn run_full_pipeline(
    paths: Vec<String>,
    output_dir: String,
    constitution: Option<String>,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<PipelineResultResponse, String> {
    let input_paths: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();
    let output = PathBuf::from(&output_dir);

    // Progression : lecture
    let _ = app.emit(
        EVENT_PIPELINE_PROGRESS,
        PipelineProgressPayload {
            stage: "ReadingInput".to_string(),
            message: format!("Lecture de {} fichier(s)...", input_paths.len()),
            progress_pct: Some(0.0),
        },
    );

    let result = state
        .pipeline
        .run_full(
            &input_paths,
            &output,
            constitution.as_deref(),
        )
        .await
        .map_err(|e| e.to_string())?;

    // Progression : termine
    let _ = app.emit(
        EVENT_PIPELINE_PROGRESS,
        PipelineProgressPayload {
            stage: "Completed".to_string(),
            message: format!(
                "Pipeline termine : {} scenarios, {} exigences",
                result.specification.user_scenarios.len(),
                result.specification.functional_requirements.len()
            ),
            progress_pct: Some(100.0),
        },
    );

    // Construire la tracabilite
    let traceability = build_traceability_matrix(&result.specification, &result.test_suite);

    let spec_json =
        serde_json::to_value(&result.specification).map_err(|e| e.to_string())?;
    let suite_json =
        serde_json::to_value(&result.test_suite).map_err(|e| e.to_string())?;
    let trace_json =
        serde_json::to_value(&traceability).map_err(|e| e.to_string())?;

    Ok(PipelineResultResponse {
        specification: spec_json,
        test_suite: suite_json,
        traceability: trace_json,
        spec_path: result.spec_path.to_string_lossy().to_string(),
        feature_paths: result
            .feature_paths
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect(),
        traceability_path: result
            .traceability_path
            .map(|p| p.to_string_lossy().to_string()),
    })
}

/// Construit la matrice de tracabilite depuis spec + suite JSON
#[tauri::command]
pub async fn build_traceability(
    spec_json: String,
    suite_json: String,
) -> Result<serde_json::Value, String> {
    let spec = serde_json::from_str(&spec_json).map_err(|e| e.to_string())?;
    let suite = serde_json::from_str(&suite_json).map_err(|e| e.to_string())?;
    let matrix = build_traceability_matrix(&spec, &suite);
    serde_json::to_value(&matrix).map_err(|e| e.to_string())
}
