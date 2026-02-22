# Skill: Verification qualite complete

Lance une verification qualite complete du projet spec-forge.

## Instructions

Execute les etapes suivantes dans l'ordre. A chaque etape, rapporte le resultat. Si une etape echoue, corrige le probleme avant de passer a la suivante.

### 1. Formatage
```bash
cargo fmt --check
```
Si des fichiers ne sont pas formates, lance `cargo fmt` pour corriger.

### 2. Lint (Clippy)
```bash
cargo clippy -- -D warnings 2>&1
```
Si des warnings Clippy sont detectes, corrige-les. Verifie que les corrections respectent l'architecture hexagonale (pas d'import de crates externes dans domain/).

### 3. Tests unitaires et integration
```bash
cargo test 2>&1
```
Tous les 125+ tests doivent passer. Si un test echoue, analyse la cause et propose un fix.

### 4. Verification des imports domaine
Verifie que `src/domain/` n'importe aucune crate externe sauf : serde, thiserror, uuid, chrono, regex.
Si un import interdit est detecte, signale-le comme violation d'architecture.

### 5. Resume
Produis un resume avec :
- Nombre de fichiers verifies (fmt)
- Nombre de warnings clippy (0 = OK)
- Nombre de tests passes / echoues
- Violations d'architecture detectees
- Score global : PASS ou FAIL
