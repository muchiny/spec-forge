# Skill: Creer un nouveau port

Aide a creer un nouveau port (trait abstrait) dans l'architecture hexagonale de spec-forge.

## Instructions

1. Demande a l'utilisateur :
   - Le nom du port (ex: `NotificationService`, `CacheService`)
   - Les methodes necessaires (noms, parametres, retours)
   - Si le port est async ou sync

2. Cree le fichier dans `src/ports/<nom_snake_case>.rs` en suivant les conventions :

```rust
//! Port <NomDuPort> - <description en francais>

use async_trait::async_trait;
use thiserror::Error;

/// Erreurs du service <nom>
#[derive(Error, Debug)]
pub enum <Nom>Error {
    #[error("<description>: {0}")]
    <Variante>(String),
}

/// Trait definissant le service <nom>
#[async_trait]
pub trait <NomDuPort>: Send + Sync {
    /// <description methode>
    async fn <methode>(&self, ...) -> Result<..., <Nom>Error>;
}
```

3. Mets a jour `src/ports/mod.rs` pour exporter le nouveau module.

4. Lance `cargo check` pour verifier la compilation.

5. Indique les prochaines etapes :
   - Creer un adapter concret (propose d'utiliser `/add-adapter`)
   - Integrer le port dans `Pipeline` ou le service concerne
   - Ajouter la configuration necessaire dans `config.yaml`

Arguments : $ARGUMENTS (nom du port, ex: "notification" ou "cache")
