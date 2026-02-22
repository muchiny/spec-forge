# Skill: Lancer le fuzzing

Lance les tests de fuzzing sur spec-forge.

## Instructions

1. Liste les targets de fuzz disponibles :
```bash
ls fuzz/fuzz_targets/
```

2. Si l'utilisateur a specifie un target dans les arguments, lance-le. Sinon, demande quel target lancer parmi les disponibles.

3. Lance le fuzzing (duree par defaut : 60 secondes) :
```bash
cargo +nightly fuzz run <target> -- -max_total_time=60
```

Note : cargo-fuzz necessite le toolchain nightly. Si nightly n'est pas installe, indique :
```bash
rustup toolchain install nightly
cargo +nightly install cargo-fuzz
```

4. Analyse les resultats :
   - Si un crash est detecte, lis l'input qui a cause le crash dans `fuzz/artifacts/`
   - Determine la cause racine
   - Propose un fix

5. Resume :
   - Nombre d'iterations executees
   - Crashes trouves (oui/non)
   - Recommandations

Arguments : $ARGUMENTS (target de fuzz, ex: "fuzz_story_parser")
