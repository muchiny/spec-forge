#![no_main]
use libfuzzer_sys::{fuzz_target, Corpus};
use spec_forge::adapters::input::docx_reader::DocxReader;

fuzz_target!(|data: &[u8]| -> Corpus {
    let Ok(xml) = std::str::from_utf8(data) else {
        return Corpus::Reject;
    };
    // extract_text_from_xml must never panic
    let _ = DocxReader::extract_text_from_xml(xml);
    Corpus::Keep
});
