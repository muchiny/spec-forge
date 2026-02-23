#![no_main]
use libfuzzer_sys::{fuzz_target, Corpus};

/// Fuzz la compilation et le rendu de templates Handlebars
fuzz_target!(|data: &[u8]| -> Corpus {
    let Ok(input) = std::str::from_utf8(data) else {
        return Corpus::Reject;
    };

    // Fuzzer le parsing Handlebars — ne doit jamais panic
    let mut hbs = handlebars::Handlebars::new();
    hbs.set_strict_mode(false);
    let _ = hbs.register_template_string("fuzz", input);

    // Si le template compile, tester le rendu avec un contexte vide
    if hbs.register_template_string("fuzz2", input).is_ok() {
        let ctx = serde_json::json!({
            "title": "Test",
            "actor": "user",
            "action": "test",
            "benefit": "value",
            "language": "fr",
        });
        let _ = hbs.render("fuzz2", &ctx);
    }

    Corpus::Keep
});
