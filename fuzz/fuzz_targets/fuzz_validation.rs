#![no_main]
use libfuzzer_sys::{fuzz_target, Corpus};
use spec_forge::domain::specification::Specification;
use spec_forge::domain::validation::validate_specification;

fuzz_target!(|data: &[u8]| -> Corpus {
    let Ok(input) = std::str::from_utf8(data) else {
        return Corpus::Reject;
    };
    // Tenter de deserialiser un JSON arbitraire en Specification
    let Ok(spec) = serde_json::from_str::<Specification>(input) else {
        return Corpus::Reject;
    };
    // La validation ne doit jamais paniquer, quelle que soit la specification
    let _ = validate_specification(&spec);
    Corpus::Keep
});
