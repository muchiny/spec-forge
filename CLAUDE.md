# spec-forge - Regles Claude Code

## Projet

spec-forge est un CLI Rust qui transforme des User Stories en Specifications (ISO 29148) et Tests Gherkin/BDD via LLM local (Ollama). Architecture hexagonale, ~6700 LOC, 125+ tests.

## Architecture hexagonale stricte

```
domain/        Logique metier pure (PAS d'I/O, pas de crate externe sauf serde/thiserror/uuid/chrono)
ports/         Traits abstraits (#[async_trait] + Send + Sync)
adapters/      Implementations concretes des ports
  llm/         OllamaAdapter, MockAdapter
  input/       MarkdownReader, YamlReader, PdfReader, DocxReader
  output/      MarkdownWriter, GherkinWriter, TraceabilityWriter
  templates/   FileTemplateEngine (Handlebars)
application/   Orchestration (Pipeline, RefineService, GenerateTestsService)
infrastructure/ Config YAML, logging (tracing)
tui/           Interface TUI (ratatui + crossterm)
```

### Regles d'architecture
- **domain/** ne depend de RIEN d'externe (pas de reqwest, pas de tokio fs, pas de handlebars)
- Les ports definissent les contrats via `#[async_trait]` avec `Send + Sync`
- Injection de dependances via `Arc<dyn Trait>`
- Erreurs domaine avec `thiserror`, erreurs application avec `anyhow::Result`
- Pour ajouter une nouvelle feature : definir le port d'abord, puis l'adapter

## Normes ISO (coeur du projet)

- **ISO/IEC/IEEE 29148:2018** : 9 criteres de bien-formation des exigences (voir `domain/validation.rs`)
- **ISO/IEC 25010:2023** : 9 caracteristiques qualite (FunctionalSuitability, PerformanceEfficiency, Compatibility, InteractionCapability, Reliability, Security, Maintainability, Flexibility, Safety)
- **ISO/IEC 25023:2016** : metriques qualite (functional completeness, requirement stability, test adequacy)
- **ISO/IEC/IEEE 29119** : niveaux de test (Unit, Integration, System, Acceptance) et techniques (EquivalencePartitioning, BoundaryValueAnalysis, DecisionTable, StateTransition, ErrorGuessing)
- Mots normatifs obligatoires dans les exigences : MUST/SHALL/SHOULD/COULD (ou DOIT/DEVRAIT/POURRAIT)
- Mots **interdits** dans les exigences : environ, quelques, peut-etre, parfois, souvent, approximativement, etc (voir AMBIGUOUS_WORDS dans validation.rs)

## Style de code Rust

- **Edition 2024**, Rust >= 1.93.1
- Commentaires, messages d'erreur et documentation en **francais**
- Documentation `///` pour tous les items publics
- Pattern matching exhaustif (eviter `_` catch-all sauf si justifie)
- Serialisation : `serde` avec `#[derive(Serialize, Deserialize)]`
- Async : `tokio` runtime, `#[async_trait]` pour les traits
- Pas de `.unwrap()` dans le code de production (seulement dans les tests)

## Tests

- Tests unitaires inline `#[cfg(test)]` dans chaque module du domaine
- Tests d'integration dans `tests/integration/`
- Property-based testing avec `proptest`
- Snapshot testing avec `insta`
- Mock HTTP avec `wiremock`
- Fuzz testing avec `cargo-fuzz` dans `fuzz/`
- Assertion : `pretty_assertions` pour les diffs lisibles

## Templates LLM

Les prompts LLM sont dans `templates/` au format Handlebars :
- `refine_system.md` / `refine_user.md` : US -> Specification
- `generate_tests_system.md` / `generate_tests_user.md` : Specification -> Gherkin
- Variables : `{{title}}`, `{{actor}}`, `{{action}}`, `{{benefit}}`, `{{language}}`, etc.

Lors de la modification des templates, toujours valider que le JSON schema de sortie correspond aux structs Rust dans `domain/`.

## Commandes de developpement

```bash
cargo build                    # Compilation
cargo test                     # Tous les tests (125+)
cargo clippy                   # Lint
cargo fmt                      # Formatage
cargo run -- check             # Verifier connexion Ollama
cargo run -- tui               # Interface TUI
cargo run -- pipeline -i examples/sample_us.md -o output  # Pipeline complet
cargo run -- refine -i examples/sample_us.md               # Raffinement seul
cargo fuzz run fuzz_story_parser  # Fuzzing
```

## Fichiers cles

| Fichier | Role |
|---------|------|
| `src/domain/specification.rs` | Modele Specification + types compliance |
| `src/domain/validation.rs` | Validation ISO 29148 (9 criteres) |
| `src/domain/traceability.rs` | Matrice de tracabilite bidirectionnelle |
| `src/domain/test_case.rs` | Modele Feature/Scenario/Step Gherkin |
| `src/ports/llm_service.rs` | Trait LLM + LlmConfig |
| `src/adapters/llm/ollama_adapter.rs` | Client HTTP Ollama |
| `src/application/pipeline.rs` | Orchestrateur principal |
| `src/application/refine_service.rs` | Raffinement US -> Spec via LLM |
| `src/application/generate_tests_service.rs` | Generation Spec -> Gherkin via LLM |
| `src/infrastructure/config.rs` | Configuration YAML hierarchique |
| `src/tui/` | Interface TUI ratatui |
| `config.yaml` | Configuration par defaut |
| `templates/` | Prompts LLM Handlebars |

## Compliance profiles

Le systeme supporte plusieurs profils de conformite reglementaire :
- `General` : ISO 29148 basique
- `Aviation(DalLevel)` : DO-178C (A/B/C/D/E)
- `Medical(SwClass)` : IEC 62304 (A/B/C)
- `Automotive(AsilLevel)` : ISO 26262 (A/B/C/D)
- `Railway(SsilLevel)` : EN 50716 (1/2/3/4)
- `Safety(SilLevel)` : IEC 61508 (1/2/3/4)

## Multi-langue

- Detection automatique FR (`"En tant que..."`) / EN (`"As a..."`)
- Gherkin en FR (Soit/Quand/Alors) ou EN (Given/When/Then) selon config
- Configuration via `pipeline.default_language` et `output.gherkin_language`
