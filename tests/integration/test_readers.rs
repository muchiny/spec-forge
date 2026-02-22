use std::path::Path;

use spec_forge::adapters::input::markdown_reader::MarkdownReader;
use spec_forge::adapters::input::yaml_reader::YamlReader;
use spec_forge::domain::user_story::Language;
use spec_forge::ports::input_reader::InputReader;

#[tokio::test]
async fn test_read_french_markdown() {
    let reader = MarkdownReader::new();
    let path = Path::new("tests/fixtures/sample_us_fr.md");
    let result = reader.read_stories(path).await;

    assert!(
        result.is_ok(),
        "Failed to read FR markdown: {:?}",
        result.err()
    );
    let story_set = result.unwrap();
    assert_eq!(story_set.stories.len(), 2);
    assert_eq!(story_set.language, Language::French);
    assert_eq!(story_set.stories[0].actor, "bibliothecaire");
    assert_eq!(story_set.stories[1].actor, "futur adherent");
    assert_eq!(story_set.stories[0].acceptance_criteria.len(), 2);
}

#[tokio::test]
async fn test_read_english_markdown() {
    let reader = MarkdownReader::new();
    let path = Path::new("tests/fixtures/sample_us_en.md");
    let result = reader.read_stories(path).await;

    assert!(
        result.is_ok(),
        "Failed to read EN markdown: {:?}",
        result.err()
    );
    let story_set = result.unwrap();
    assert_eq!(story_set.stories.len(), 2);
    assert_eq!(story_set.language, Language::English);
    assert_eq!(story_set.stories[0].actor, "librarian");
}

#[tokio::test]
async fn test_read_yaml() {
    let reader = YamlReader::new();
    let path = Path::new("tests/fixtures/sample_us.yaml");
    let result = reader.read_stories(path).await;

    assert!(result.is_ok(), "Failed to read YAML: {:?}", result.err());
    let story_set = result.unwrap();
    assert_eq!(story_set.stories.len(), 2);
    assert_eq!(story_set.stories[0].actor, "bibliothecaire");
    assert_eq!(story_set.stories[0].acceptance_criteria.len(), 2);
}

#[tokio::test]
async fn test_read_nonexistent_file() {
    let reader = MarkdownReader::new();
    let path = Path::new("tests/fixtures/does_not_exist.md");
    let result = reader.read_stories(path).await;
    assert!(result.is_err());
}
