#![no_main]
use libfuzzer_sys::{fuzz_target, Corpus};
use spec_forge::adapters::input::story_parser;
use spec_forge::domain::user_story::Language;

fuzz_target!(|data: &[u8]| -> Corpus {
    let Ok(input) = std::str::from_utf8(data) else {
        return Corpus::Reject;
    };
    // Must never panic — only return Ok/Err
    let _ = story_parser::parse_stories(input, Language::French);
    let _ = story_parser::parse_stories(input, Language::English);
    let _ = story_parser::detect_language(input);
    Corpus::Keep
});
