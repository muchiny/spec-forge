<!--
  Sync Impact Report
  ==================
  Version change: (none) → 1.0.0
  Modified principles: N/A (initial creation)
  Added sections:
    - Core Principles (6 principles)
    - Contraintes Techniques
    - Processus de Developpement
    - Governance
  Removed sections: N/A
  Templates requiring updates:
    - .specify/templates/plan-template.md — ✅ compatible (Constitution Check
      section aligns with principles)
    - .specify/templates/spec-template.md — ✅ compatible (FR-xxx format,
      MUST/SHOULD normative words, priorities align)
    - .specify/templates/tasks-template.md — ✅ compatible (test-first
      discipline, phased delivery, parallel markers align)
  Follow-up TODOs: none
-->

# spec-forge Constitution

## Core Principles

### I. Architecture Hexagonale Stricte

Le projet DOIT suivre une architecture hexagonale (ports & adapters) :

- Chaque couche a une responsabilite unique et des dependances
  unidirectionnelles : `domain/ → ports/ → adapters/ → application/
  → infrastructure/ → tui/`
- Les contrats entre couches sont definis par des traits dans `ports/`
  avec `#[async_trait]` + `Send + Sync`
- L'injection de dependances se fait via `Arc<dyn Trait>`
- Toute nouvelle fonctionnalite DOIT definir le port d'abord, puis
  l'adapter
- Aucun import direct entre adapters — la communication passe
  toujours par les ports

**Rationale** : Garantit la testabilite, la modularite et
l'independance vis-a-vis des implementations concretes (LLM, I/O,
templates).

### II. Purete du Domaine (NON-NEGOTIABLE)

Le module `domain/` ne DOIT avoir AUCUNE dependance externe d'I/O :

- Pas de `reqwest`, `tokio::fs`, `handlebars`, `std::fs` ou tout
  crate effectuant des operations I/O
- Seules dependances autorisees : `serde`, `thiserror`, `uuid`,
  `chrono`
- Les erreurs domaine utilisent `thiserror` exclusivement
- Les erreurs application utilisent `anyhow::Result`
- Le domaine contient la logique metier pure : validation,
  transformation, modeles

**Rationale** : Un domaine pur permet des tests unitaires rapides,
deterministes et sans mock d'infrastructure.

### III. Conformite ISO

Le projet DOIT respecter les normes ISO suivantes :

- **ISO/IEC/IEEE 29148:2018** : 9 criteres de bien-formation des
  exigences (implementes dans `domain/validation.rs`)
- **ISO/IEC 25010:2023** : 9 caracteristiques qualite
- **ISO/IEC 25023:2016** : metriques qualite
- **ISO/IEC/IEEE 29119** : niveaux de test et techniques

Regles normatives :
- Les exigences DOIVENT contenir des mots normatifs : MUST, SHALL,
  SHOULD, COULD (ou DOIT, DEVRAIT, POURRAIT)
- Les mots ambigus sont INTERDITS dans les exigences (voir
  `AMBIGUOUS_WORDS` dans `domain/validation.rs`)
- Les profils de conformite reglementaire (Aviation DO-178C, Medical
  IEC 62304, Automotive ISO 26262, Railway EN 50716, Safety
  IEC 61508) DOIVENT etre supportes

**Rationale** : La conformite ISO est la proposition de valeur
centrale du projet — elle distingue spec-forge des outils generiques
de generation de tests.

### IV. Discipline de Test

Le projet DOIT maintenir une couverture de test rigoureuse :

- Tests unitaires inline `#[cfg(test)]` dans chaque module du domaine
- Tests d'integration dans `tests/integration/`
- Property-based testing avec `proptest` pour les invariants
- Snapshot testing avec `insta` pour les sorties stables
- Mock HTTP avec `wiremock` pour les interactions reseau
- Fuzz testing avec `cargo-fuzz` dans `fuzz/`
- `.unwrap()` est INTERDIT dans le code de production (autorise
  uniquement dans les tests)
- Les assertions utilisent `pretty_assertions` pour des diffs lisibles

**Rationale** : La fiabilite des specifications generees depend
directement de la fiabilite du code qui les produit.

### V. Francais par Defaut

Toute communication generee par le code DOIT etre en francais :

- Commentaires de code en francais
- Messages d'erreur en francais
- Documentation `///` pour tous les items publics, en francais
- Detection automatique FR/EN pour les entrees utilisateur
  (`"En tant que..."` / `"As a..."`)
- Gherkin en FR (`Soit`/`Quand`/`Alors`) ou EN
  (`Given`/`When`/`Then`) selon configuration

**Rationale** : Le projet cible en priorite un public francophone et
s'inscrit dans l'ecosysteme de la qualite logicielle francaise.

### VI. Simplicite et YAGNI

Le code DOIT rester simple et focalisé sur le besoin actuel :

- Pas de sur-ingenierie : seuls les changements directement
  necessaires sont implementes
- Pas d'abstractions prematurees : trois lignes similaires valent
  mieux qu'une abstraction speculative
- Pas de feature flags ou shims de retro-compatibilite sans
  justification
- Le pattern matching DOIT etre exhaustif (eviter `_` catch-all
  sauf si justifie)
- La complexite ajoutee DOIT etre justifiee dans le Complexity
  Tracking du plan d'implementation

**Rationale** : Un outil CLI de transformation DOIT rester
comprehensible et maintenable par un contributeur unique.

## Contraintes Techniques

- **Langage** : Rust, edition 2024, version >= 1.93.1
- **Runtime async** : `tokio` (full features)
- **CLI** : `clap` 4 (derive API)
- **TUI** : `ratatui` 0.30 + `crossterm` 0.29
- **LLM** : Ollama via `reqwest` (HTTP/JSON), templates Handlebars 6
- **Serialisation** : `serde` + `serde_json` + `serde_yaml`
- **Configuration** : YAML hierarchique via le crate `config`
- **Logging** : `tracing` (logs structures)
- Les templates LLM (dans `templates/`) DOIVENT correspondre aux
  schemas JSON des structs Rust dans `domain/`
- Toute modification de template DOIT etre validee contre les
  structs de sortie

## Processus de Developpement

- `cargo fmt` DOIT passer sans modification avant tout commit
- `cargo clippy` DOIT produire 0 warning
- `cargo test` DOIT passer (125+ tests) avant toute merge
- Les revues de code DOIVENT verifier la conformite a cette
  constitution
- Toute nouvelle feature suit le cycle : port → adapter → service
  → tests
- Les commandes de reference :
  - `cargo build` : compilation
  - `cargo test` : tous les tests
  - `cargo clippy` : lint
  - `cargo fmt` : formatage
  - `cargo run -- check` : verification connexion Ollama
  - `cargo run -- pipeline -i <input> -o output` : pipeline complet
  - `cargo fuzz run fuzz_story_parser` : fuzzing

## Governance

Cette constitution est le document de reference pour toutes les
decisions de developpement sur spec-forge. Elle prevaut sur toute
autre convention implicite.

**Procedure d'amendement** :
1. Proposer la modification avec justification
2. Verifier l'impact sur les templates dependants (plan, spec, tasks)
3. Mettre a jour la constitution et incrementer la version
4. Propager les changements aux artefacts impactes

**Politique de versionnement** :
- MAJOR : suppression ou redefinition incompatible d'un principe
- MINOR : ajout d'un principe ou expansion materielle d'une section
- PATCH : clarifications, corrections de formulation, corrections
  typographiques

**Revue de conformite** :
- Chaque PR DOIT etre verifiee contre les principes de cette
  constitution
- Le fichier `CLAUDE.md` sert de guide operationnel pour le
  developpement au quotidien

**Version**: 1.0.0 | **Ratified**: 2026-02-22 | **Last Amended**: 2026-02-22
