#![no_main]
use libfuzzer_sys::{fuzz_target, Corpus};
use spec_forge::infrastructure::config::Config;

fuzz_target!(|data: &[u8]| -> Corpus {
    let Ok(input) = std::str::from_utf8(data) else {
        return Corpus::Reject;
    };
    // Le parsing YAML ne doit jamais paniquer, meme avec des entrees invalides
    let _ = serde_yaml::from_str::<Config>(input);
    Corpus::Keep
});
