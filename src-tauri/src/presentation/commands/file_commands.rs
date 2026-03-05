//! Commandes de lecture de fichiers

use std::path::PathBuf;

use tauri::State;

use spec_forge::application::pipeline::Pipeline;

use crate::presentation::state::AppState;

/// Lit les User Stories depuis les chemins fournis
#[tauri::command]
pub async fn read_stories(
    paths: Vec<String>,
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let input_paths: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();
    let pipeline_guard = state.pipeline.read().await;
    let pipeline = pipeline_guard
        .as_ref()
        .ok_or_else(|| "LLM non initialise".to_string())?;
    let story_set = pipeline
        .read_stories_multi(&input_paths)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_value(&story_set).map_err(|e| e.to_string())
}

/// Expanse les chemins d'entree (dossiers -> fichiers)
#[tauri::command]
pub async fn expand_input_paths(paths: Vec<String>) -> Result<Vec<String>, String> {
    let input_paths: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();
    let expanded = Pipeline::expand_paths(&input_paths).map_err(|e| e.to_string())?;
    Ok(expanded
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect())
}
