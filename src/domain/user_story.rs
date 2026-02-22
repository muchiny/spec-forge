//! Modele de domaine - User Story
//!
//! Represente une User Story en entree du pipeline.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Priorite MoSCoW alignee avec spec-kit P1/P2/P3
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    /// Must have (critique)
    P1,
    /// Should have (important)
    P2,
    /// Could have (souhaitable)
    P3,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::P1 => write!(f, "P1 (Must)"),
            Priority::P2 => write!(f, "P2 (Should)"),
            Priority::P3 => write!(f, "P3 (Could)"),
        }
    }
}

/// Langues supportees pour les entrees/sorties
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Language {
    #[default]
    #[serde(alias = "fr", alias = "FR", alias = "french")]
    French,
    #[serde(alias = "en", alias = "EN", alias = "english")]
    English,
}

impl Language {
    /// Code Gherkin pour le header `# language:`
    pub fn gherkin_code(&self) -> &'static str {
        match self {
            Language::French => "fr",
            Language::English => "en",
        }
    }

    /// Construit une Language a partir d'un code ("fr", "en")
    pub fn from_code(code: &str) -> Self {
        match code {
            "en" | "EN" | "english" => Language::English,
            _ => Language::French,
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::French => write!(f, "fr"),
            Language::English => write!(f, "en"),
        }
    }
}

/// User Story en entree (avant raffinement)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStory {
    /// Identifiant interne
    pub id: Uuid,

    /// Identifiant externe si fourni (ex: "US-001")
    #[serde(default)]
    pub external_id: Option<String>,

    /// Titre de la user story
    pub title: String,

    /// Acteur ("En tant que ...")
    pub actor: String,

    /// Action ("Je veux ...")
    pub action: String,

    /// Benefice ("Afin de ...")
    pub benefit: String,

    /// Priorite (peut etre vide, sera inferee par le LLM)
    #[serde(default)]
    pub priority: Option<Priority>,

    /// Criteres d'acceptation bruts (peut etre vide avant raffinement)
    #[serde(default)]
    pub acceptance_criteria: Vec<String>,

    /// Texte source original
    #[serde(default)]
    pub raw_text: String,

    /// Tags/etiquettes
    #[serde(default)]
    pub tags: Vec<String>,

    /// Stakeholder/partie prenante (ISO 29148)
    #[serde(default)]
    pub stakeholder: Option<String>,
}

impl UserStory {
    /// Cree une nouvelle UserStory avec un UUID genere
    pub fn new(title: String, actor: String, action: String, benefit: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            external_id: None,
            title,
            actor,
            action,
            benefit,
            priority: None,
            acceptance_criteria: Vec::new(),
            raw_text: String::new(),
            tags: Vec::new(),
            stakeholder: None,
        }
    }

    /// Formate la user story en format standard
    pub fn to_standard_format(&self) -> String {
        format!(
            "En tant que {}, je veux {} afin de {}.",
            self.actor, self.action, self.benefit
        )
    }
}

/// Collection de user stories en entree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStorySet {
    /// Liste des user stories
    pub stories: Vec<UserStory>,

    /// Fichiers sources
    #[serde(default, alias = "source_file")]
    pub source_files: Vec<String>,

    /// Langue des user stories
    #[serde(default)]
    pub language: Language,
}

impl UserStorySet {
    /// Fusionne plusieurs UserStorySet en un seul.
    /// Utilise la langue du premier set. Avertit si des langues mixtes sont detectees.
    pub fn merge(sets: Vec<UserStorySet>) -> Self {
        if sets.is_empty() {
            return Self {
                stories: Vec::new(),
                source_files: Vec::new(),
                language: Language::default(),
            };
        }

        let language = sets[0].language;
        let mut stories = Vec::new();
        let mut source_files = Vec::new();

        for set in &sets {
            if set.language != language {
                tracing::warn!(
                    expected = %language,
                    found = %set.language,
                    files = ?set.source_files,
                    "Langues mixtes detectees lors de la fusion — utilisation de '{}'",
                    language
                );
            }
        }

        for set in sets {
            stories.extend(set.stories);
            source_files.extend(set.source_files);
        }

        Self {
            stories,
            source_files,
            language,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_story_creation() {
        let us = UserStory::new(
            "Recherche ISBN".into(),
            "bibliothecaire".into(),
            "rechercher un livre par ISBN".into(),
            "trouver rapidement un ouvrage".into(),
        );
        assert!(!us.id.is_nil());
        assert_eq!(us.title, "Recherche ISBN");
        assert!(us.priority.is_none());
    }

    #[test]
    fn test_standard_format() {
        let us = UserStory::new(
            "Test".into(),
            "utilisateur".into(),
            "me connecter".into(),
            "acceder a mon compte".into(),
        );
        assert_eq!(
            us.to_standard_format(),
            "En tant que utilisateur, je veux me connecter afin de acceder a mon compte."
        );
    }

    #[test]
    fn test_priority_display() {
        assert_eq!(Priority::P1.to_string(), "P1 (Must)");
        assert_eq!(Priority::P2.to_string(), "P2 (Should)");
        assert_eq!(Priority::P3.to_string(), "P3 (Could)");
    }

    #[test]
    fn test_language_gherkin_code() {
        assert_eq!(Language::French.gherkin_code(), "fr");
        assert_eq!(Language::English.gherkin_code(), "en");
    }

    #[test]
    fn test_merge_story_sets() {
        let set1 = UserStorySet {
            stories: vec![UserStory::new(
                "US1".into(),
                "actor".into(),
                "action1".into(),
                "benefit1".into(),
            )],
            source_files: vec!["file1.md".into()],
            language: Language::French,
        };
        let set2 = UserStorySet {
            stories: vec![
                UserStory::new(
                    "US2".into(),
                    "actor".into(),
                    "action2".into(),
                    "benefit2".into(),
                ),
                UserStory::new(
                    "US3".into(),
                    "actor".into(),
                    "action3".into(),
                    "benefit3".into(),
                ),
            ],
            source_files: vec!["file2.yaml".into()],
            language: Language::French,
        };

        let merged = UserStorySet::merge(vec![set1, set2]);
        assert_eq!(merged.stories.len(), 3);
        assert_eq!(merged.source_files, vec!["file1.md", "file2.yaml"]);
        assert_eq!(merged.language, Language::French);
    }

    #[test]
    fn test_merge_empty() {
        let merged = UserStorySet::merge(vec![]);
        assert!(merged.stories.is_empty());
        assert!(merged.source_files.is_empty());
    }
}
