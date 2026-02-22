#![no_main]
use libfuzzer_sys::{fuzz_target, Corpus};
use spec_forge::application::refine_service::clean_json_response;

fuzz_target!(|data: &[u8]| -> Corpus {
    let Ok(input) = std::str::from_utf8(data) else {
        return Corpus::Reject;
    };
    // Must never panic
    let _ = clean_json_response(input);
    Corpus::Keep
});
