# Skill: Analyse de couverture de tests

Analyse la couverture de tests du projet spec-forge.

## Instructions

### 1. Etat actuel des tests
```bash
cargo test 2>&1
```
Compte le nombre de tests par categorie (unit, integration).

### 2. Analyse de couverture par module
Pour chaque module, verifie la presence de tests :

| Module | Fichier | Tests inline | Tests integration |
|--------|---------|--------------|-------------------|
| domain/user_story | src/domain/user_story.rs | oui/non | oui/non |
| domain/specification | ... | ... | ... |
| ... | ... | ... | ... |

Lis chaque fichier source et verifie la presence d'un bloc `#[cfg(test)]`.

### 3. Zones non couvertes
Identifie les fichiers/fonctions qui manquent de tests :
- Fonctions publiques sans test correspondant
- Branches de match non testees
- Cas d'erreur non testes

### 4. Couverture instrumentee (si disponible)
Si `cargo-llvm-cov` est installe :
```bash
cargo llvm-cov --summary-only 2>&1
```
Sinon, indique comment l'installer :
```bash
cargo install cargo-llvm-cov
rustup component add llvm-tools-preview
```

### 5. Recommandations
- Liste les tests manquants les plus critiques (tries par impact)
- Propose des tests a ajouter avec un squelette de code
- Priorise : domain/ > application/ > adapters/
