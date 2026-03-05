//! Commandes du pipeline — execution et tracabilite

use std::path::PathBuf;

use serde::Serialize;
use tauri::{AppHandle, Emitter, State};

use spec_forge::domain::traceability::build_traceability_matrix;

use crate::presentation::events::{EVENT_PIPELINE_PROGRESS, PipelineProgressPayload};
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

/// Emet un evenement de progression du pipeline
fn emit_progress(app: &AppHandle, stage: &str, message: &str, pct: f32) {
    let _ = app.emit(
        EVENT_PIPELINE_PROGRESS,
        PipelineProgressPayload {
            stage: stage.to_string(),
            message: message.to_string(),
            progress_pct: Some(pct),
        },
    );
}

/// Execute le pipeline complet : US -> Spec -> Tests -> Tracabilite
/// Chaque etape emet des evenements de progression vers le frontend.
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
    let specs_dir = output.join("specs");
    let features_dir = output.join("features");

    let pipeline_guard = state.pipeline.read().await;
    let pipeline = pipeline_guard.as_ref().ok_or_else(|| {
        "LLM non initialise — installez le modele depuis le tableau de bord".to_string()
    })?;

    // Etape 1 : Lecture des fichiers
    emit_progress(
        &app,
        "ReadingInput",
        &format!("Demarrage du pipeline avec {} fichier(s)...", input_paths.len()),
        0.0,
    );

    for (i, path) in input_paths.iter().enumerate() {
        let filename = path.file_name().map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());
        emit_progress(
            &app,
            "ReadingInput",
            &format!("Lecture du fichier {}/{} : {}", i + 1, input_paths.len(), filename),
            (i as f32 / input_paths.len() as f32) * 10.0,
        );
    }

    let story_set = pipeline
        .read_stories_multi(&input_paths)
        .await
        .map_err(|e| e.to_string())?;

    emit_progress(
        &app,
        "ReadingInput",
        &format!(
            "{} user stories chargees depuis {} fichier(s)",
            story_set.stories.len(),
            input_paths.len()
        ),
        10.0,
    );

    // Etape 2 : Raffinement des specifications (etape la plus longue)
    emit_progress(
        &app,
        "RefiningSpec",
        "Preparation du prompt LLM pour le raffinement...",
        12.0,
    );

    emit_progress(
        &app,
        "RefiningSpec",
        &format!(
            "Envoi de {} user stories au LLM pour raffinement...",
            story_set.stories.len()
        ),
        15.0,
    );

    let spec = pipeline
        .refine_stories(&story_set, &specs_dir, constitution.as_deref())
        .await
        .map_err(|e| e.to_string())?;

    emit_progress(
        &app,
        "RefiningSpec",
        &format!(
            "Reponse LLM recue : {} scenarios identifies",
            spec.user_scenarios.len()
        ),
        42.0,
    );

    emit_progress(
        &app,
        "RefiningSpec",
        &format!(
            "Specification generee : {} scenarios, {} exigences fonctionnelles",
            spec.user_scenarios.len(),
            spec.functional_requirements.len()
        ),
        48.0,
    );

    emit_progress(
        &app,
        "RefiningSpec",
        &format!("Ecriture de la specification dans {}", specs_dir.display()),
        50.0,
    );

    // Etape 3 : Generation des tests
    emit_progress(
        &app,
        "GeneratingTests",
        "Preparation du prompt LLM pour les tests Gherkin...",
        52.0,
    );

    emit_progress(
        &app,
        "GeneratingTests",
        &format!(
            "Generation des tests pour {} exigences...",
            spec.functional_requirements.len()
        ),
        55.0,
    );

    let suite = pipeline
        .generate_tests(&spec, &features_dir)
        .await
        .map_err(|e| e.to_string())?;

    emit_progress(
        &app,
        "GeneratingTests",
        &format!(
            "Reponse LLM recue : {} features, {} scenarios de test",
            suite.features.len(),
            suite.total_scenarios
        ),
        75.0,
    );

    emit_progress(
        &app,
        "GeneratingTests",
        &format!("Ecriture des fichiers .feature dans {}", features_dir.display()),
        80.0,
    );

    // Etape 4 : Tracabilite
    emit_progress(
        &app,
        "WritingOutput",
        "Construction de la matrice de tracabilite...",
        85.0,
    );

    let traceability = build_traceability_matrix(&spec, &suite);

    emit_progress(
        &app,
        "WritingOutput",
        &format!(
            "Tracabilite : {} exigences tracees, couverture calculee",
            traceability.entries.len()
        ),
        95.0,
    );

    emit_progress(
        &app,
        "Completed",
        &format!(
            "Pipeline termine avec succes : {} scenarios, {} exigences, {} features, {} tests",
            spec.user_scenarios.len(),
            spec.functional_requirements.len(),
            suite.features.len(),
            suite.total_scenarios
        ),
        100.0,
    );

    let spec_json = serde_json::to_value(&spec).map_err(|e| e.to_string())?;
    let suite_json = serde_json::to_value(&suite).map_err(|e| e.to_string())?;
    let trace_json = serde_json::to_value(&traceability).map_err(|e| e.to_string())?;

    // Recuperer les chemins generes depuis les repertoires de sortie
    let spec_path = specs_dir.to_string_lossy().to_string();
    let feature_paths: Vec<String> = suite
        .features
        .iter()
        .map(|f| features_dir.join(format!("{}.feature", f.name)).to_string_lossy().to_string())
        .collect();

    Ok(PipelineResultResponse {
        specification: spec_json,
        test_suite: suite_json,
        traceability: trace_json,
        spec_path,
        feature_paths,
        traceability_path: None,
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
