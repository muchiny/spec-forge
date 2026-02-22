use std::path::{Path, PathBuf};
use std::sync::Arc;

use spec_forge::adapters::llm::mock_adapter::MockLlmAdapter;
use spec_forge::adapters::templates::file_template_engine::FileTemplateEngine;
use spec_forge::application::pipeline::Pipeline;
use spec_forge::infrastructure::config::Config;

fn mock_refine_response() -> String {
    std::fs::read_to_string("tests/fixtures/mock_refine_response.json").unwrap()
}

fn mock_generate_response() -> String {
    std::fs::read_to_string("tests/fixtures/mock_generate_response.json").unwrap()
}

fn create_test_pipeline(responses: Vec<String>) -> Pipeline {
    let llm = Arc::new(MockLlmAdapter::new(responses));
    let templates = Arc::new(FileTemplateEngine::new(Path::new("templates")).unwrap());
    let config = Config::default();
    Pipeline::new(llm, templates, config)
}

#[tokio::test]
async fn test_pipeline_read_stories_markdown() {
    let pipeline = create_test_pipeline(vec![]);
    let result = pipeline
        .read_stories(Path::new("tests/fixtures/sample_us_fr.md"))
        .await;

    assert!(result.is_ok());
    let story_set = result.unwrap();
    assert_eq!(story_set.stories.len(), 2);
}

#[tokio::test]
async fn test_pipeline_read_stories_yaml() {
    let pipeline = create_test_pipeline(vec![]);
    let result = pipeline
        .read_stories(Path::new("tests/fixtures/sample_us.yaml"))
        .await;

    assert!(result.is_ok());
    let story_set = result.unwrap();
    assert_eq!(story_set.stories.len(), 2);
}

#[tokio::test]
async fn test_pipeline_refine() {
    let pipeline = create_test_pipeline(vec![mock_refine_response()]);
    let dir = tempfile::TempDir::new().unwrap();

    let result = pipeline
        .refine(
            &[PathBuf::from("tests/fixtures/sample_us_fr.md")],
            dir.path(),
            None,
        )
        .await;

    assert!(result.is_ok(), "Refine failed: {:?}", result.err());
    let spec = result.unwrap();
    assert_eq!(spec.user_scenarios.len(), 2);
    assert_eq!(spec.functional_requirements.len(), 3);
}

#[tokio::test]
async fn test_pipeline_full_run() {
    let pipeline = create_test_pipeline(vec![mock_refine_response(), mock_generate_response()]);
    let dir = tempfile::TempDir::new().unwrap();

    let result = pipeline
        .run_full(
            &[PathBuf::from("tests/fixtures/sample_us_fr.md")],
            dir.path(),
            None,
        )
        .await;

    assert!(result.is_ok(), "Pipeline failed: {:?}", result.err());
    let pipeline_result = result.unwrap();

    // Spec was generated
    assert!(pipeline_result.spec_path.exists());
    assert_eq!(pipeline_result.specification.user_scenarios.len(), 2);
    assert_eq!(
        pipeline_result.specification.functional_requirements.len(),
        3
    );

    // Features were generated
    assert!(!pipeline_result.feature_paths.is_empty());
    for path in &pipeline_result.feature_paths {
        assert!(path.exists());
    }

    // Traceability was generated
    assert!(pipeline_result.traceability_path.is_some());
    assert!(pipeline_result.traceability_path.unwrap().exists());
}
