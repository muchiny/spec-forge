//! Erreurs de domaine
//!
//! Erreurs typees pour chaque couche du pipeline.

use thiserror::Error;

/// Erreur principale du domaine
#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Erreur d'entree: {0}")]
    Input(#[from] InputError),

    #[error("Erreur de raffinement: {0}")]
    Refinement(#[from] RefinementError),

    #[error("Erreur de generation: {0}")]
    Generation(#[from] GenerationError),

    #[error("Erreur de validation: {0}")]
    Validation(#[from] ValidationError),
}

/// Erreurs de lecture des entrees
#[derive(Error, Debug)]
pub enum InputError {
    #[error("Fichier non trouve: {path}")]
    FileNotFound { path: String },

    #[error("Format invalide: {details}")]
    InvalidFormat { details: String },

    #[error("Aucune user story trouvee dans l'entree")]
    NoStoriesFound,

    #[error("Erreur de lecture: {0}")]
    ReadError(String),

    #[error("Erreur de parsing: {0}")]
    ParseError(String),
}

/// Erreurs de raffinement US -> Spec
#[derive(Error, Debug)]
pub enum RefinementError {
    #[error("Echec du raffinement LLM: {details}")]
    LlmFailed { details: String },

    #[error("Echec du parsing de la sortie: {details}")]
    OutputParseFailed { details: String },

    #[error("Specification incomplete: sections manquantes: {missing_sections:?}")]
    IncompleteSpec { missing_sections: Vec<String> },

    #[error("Sortie LLM tronquee (max_tokens atteint): {details}")]
    OutputTruncated { details: String },
}

/// Erreurs de generation de tests
#[derive(Error, Debug)]
pub enum GenerationError {
    #[error("Echec de la generation Gherkin: {details}")]
    GherkinFailed { details: String },

    #[error("Syntaxe Gherkin invalide a la ligne {line}: {message}")]
    InvalidGherkinSyntax { line: usize, message: String },

    #[error("La specification n'a pas ete raffinee")]
    SpecNotRefined,

    #[error("Sortie LLM tronquee (max_tokens atteint): {details}")]
    OutputTruncated { details: String },
}

/// Erreurs de validation
#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Erreur de syntaxe Gherkin dans {file}: {message}")]
    GherkinSyntax { file: String, message: String },

    #[error("Ecart de tracabilite: exigences sans couverture: {missing:?}")]
    TraceabilityGap { missing: Vec<String> },

    #[error("Completude insuffisante: {score:.1}% (minimum: {threshold:.1}%)")]
    CompletenessBelow { score: f32, threshold: f32 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_error_display() {
        let err = InputError::FileNotFound {
            path: "test.md".into(),
        };
        assert_eq!(err.to_string(), "Fichier non trouve: test.md");
    }

    #[test]
    fn test_domain_error_from_input() {
        let input_err = InputError::NoStoriesFound;
        let domain_err = DomainError::from(input_err);
        assert!(matches!(domain_err, DomainError::Input(_)));
    }

    #[test]
    fn test_validation_error_display() {
        let err = ValidationError::CompletenessBelow {
            score: 60.0,
            threshold: 80.0,
        };
        assert!(err.to_string().contains("60.0%"));
    }
}
