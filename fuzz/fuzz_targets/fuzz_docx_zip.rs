#![no_main]
use libfuzzer_sys::fuzz_target;
use spec_forge::adapters::input::docx_reader::DocxReader;
use std::io::{Cursor, Read};

fuzz_target!(|data: &[u8]| {
    let cursor = Cursor::new(data);
    let Ok(mut archive) = zip::ZipArchive::new(cursor) else {
        return;
    };

    for i in 0..archive.len().min(20) {
        let Ok(file) = archive.by_index(i) else {
            continue;
        };
        let is_xml = file.name().ends_with(".xml");
        let mut contents = Vec::new();
        // Limit read to 5MB to avoid OOM
        let _ = file.take(5 * 1024 * 1024).read_to_end(&mut contents);

        if is_xml {
            if let Ok(xml_str) = std::str::from_utf8(&contents) {
                let _ = DocxReader::extract_text_from_xml(xml_str);
            }
        }
    }
});
