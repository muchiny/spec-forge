//! spec-forge - Pipeline CLI pour transformer des User Stories en Specs et Tests Gherkin/BDD
#![allow(dead_code)]

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use clap::{Parser, Subcommand};
use console::style;

mod adapters;
mod application;
mod domain;
mod infrastructure;
mod ports;
mod tui;

use adapters::llm::ollama_adapter::OllamaAdapter;
use adapters::templates::file_template_engine::FileTemplateEngine;
use application::pipeline::Pipeline;
use infrastructure::config::Config;
use infrastructure::logging;
use ports::llm_service::LlmService;

#[derive(Parser)]
#[command(
    name = "spec-forge",
    about = "Transforme des User Stories en Specifications et Tests Gherkin/BDD via LLM",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Fichier de configuration
    #[arg(short, long, default_value = "config.yaml")]
    config: String,

    /// Verbosité (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Subcommand)]
enum Commands {
    /// Lancer l'interface TUI interactive
    Tui,

    /// Raffiner des User Stories en specification complete
    Refine {
        /// Fichier(s) ou dossier(s) d'entree (Markdown, YAML, PDF, DOCX)
        #[arg(short, long, num_args = 1..)]
        input: Vec<String>,

        /// Repertoire de sortie
        #[arg(short, long, default_value = "output/specs")]
        output: String,

        /// Fichier de constitution (principes du projet)
        #[arg(long)]
        constitution: Option<String>,
    },

    /// Generer des tests Gherkin/BDD depuis une specification
    GenerateTests {
        /// Fichier de specification (Markdown)
        #[arg(short, long)]
        spec: String,

        /// Repertoire de sortie pour les .feature
        #[arg(short, long, default_value = "output/features")]
        output: String,
    },

    /// Pipeline complet: US -> Specs -> Gherkin
    Pipeline {
        /// Fichier(s) ou dossier(s) d'entree (User Stories)
        #[arg(short, long, num_args = 1..)]
        input: Vec<String>,

        /// Repertoire de sortie
        #[arg(short, long, default_value = "output")]
        output: String,

        /// Fichier de constitution
        #[arg(long)]
        constitution: Option<String>,
    },

    /// Verifier la connectivite LLM
    Check,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Charger la config
    let config = Config::load().unwrap_or_else(|_| {
        eprintln!(
            "{} config.yaml non trouve, utilisation des valeurs par defaut",
            style("!").yellow()
        );
        Config::default()
    });

    // Valider la configuration
    if let Err(e) = config.validate() {
        eprintln!("{} Configuration invalide: {}", style("!!").red().bold(), e);
        std::process::exit(1);
    }

    // Si pas de sous-commande ou commande Tui → lancer la TUI
    let command = match cli.command {
        None | Some(Commands::Tui) => {
            return tui::run(config).await;
        }
        Some(cmd) => cmd,
    };

    // Init logging (seulement pour le mode CLI)
    let mut log_config = config.logging.clone();
    match cli.verbose {
        1 => log_config.level = "debug".to_string(),
        2.. => log_config.level = "trace".to_string(),
        _ => {}
    }
    logging::init_logging(&log_config);

    match command {
        Commands::Tui => unreachable!("Tui est gere plus haut"),
        Commands::Check => {
            check_llm(&config).await?;
        }
        Commands::Refine {
            input,
            output,
            constitution,
        } => {
            let pipeline = create_pipeline(&config)?;
            let constitution_text = load_constitution(constitution.as_deref()).await?;
            let input_paths: Vec<PathBuf> = input.iter().map(PathBuf::from).collect();

            println!(
                "{} Raffinement de {} fichier(s) ...",
                style(">>").cyan().bold(),
                style(input_paths.len()).green()
            );

            let spec = pipeline
                .refine(
                    &input_paths,
                    &PathBuf::from(&output),
                    constitution_text.as_deref(),
                )
                .await?;

            println!(
                "{} Specification generee: {} scenarios, {} exigences, {} cas limites",
                style("OK").green().bold(),
                spec.user_scenarios.len(),
                spec.functional_requirements.len(),
                spec.edge_cases.len(),
            );

            if spec.has_unresolved_clarifications() {
                println!(
                    "{} {} clarification(s) necessaire(s)",
                    style("!").yellow().bold(),
                    spec.clarifications_needed
                        .iter()
                        .filter(|c| !c.resolved)
                        .count()
                );
            }
        }
        Commands::GenerateTests { spec, output: _ } => {
            println!(
                "{} Generation de tests depuis {} ...",
                style(">>").cyan().bold(),
                style(&spec).green()
            );

            // Pour cette commande, on devrait lire la spec depuis le fichier
            // Pour l'instant, on indique que c'est un TODO
            println!(
                "{} La commande generate-tests standalone sera implementee prochainement.",
                style("!").yellow()
            );
            println!("  Utilisez 'spec-forge pipeline' pour le flux complet.");
        }
        Commands::Pipeline {
            input,
            output,
            constitution,
        } => {
            let pipeline = create_pipeline(&config)?;
            let constitution_text = load_constitution(constitution.as_deref()).await?;
            let input_paths: Vec<PathBuf> = input.iter().map(PathBuf::from).collect();

            println!(
                "{} Pipeline complet: {} fichier(s) -> specs -> gherkin",
                style(">>").cyan().bold(),
                style(input_paths.len()).green()
            );

            let result = pipeline
                .run_full(
                    &input_paths,
                    &PathBuf::from(&output),
                    constitution_text.as_deref(),
                )
                .await?;

            println!();
            println!(
                "{} Pipeline termine avec succes!",
                style("OK").green().bold()
            );
            println!();
            println!(
                "  Specification: {}",
                style(result.spec_path.display()).cyan()
            );
            println!(
                "  - {} scenarios utilisateur",
                result.specification.user_scenarios.len()
            );
            println!(
                "  - {} exigences fonctionnelles",
                result.specification.functional_requirements.len()
            );
            println!();
            println!("  Tests Gherkin:");
            for path in &result.feature_paths {
                println!("  - {}", style(path.display()).cyan());
            }
            println!(
                "  - {} scenarios ({} happy path, {} edge case, {} erreur)",
                result.test_suite.total_scenarios,
                result.test_suite.coverage.scenarios_by_type.happy_path,
                result.test_suite.coverage.scenarios_by_type.edge_case,
                result.test_suite.coverage.scenarios_by_type.error_scenario,
            );
            println!(
                "  - Couverture: {:.0}%",
                result.test_suite.coverage.coverage_percentage
            );

            if let Some(trace_path) = &result.traceability_path {
                println!();
                println!("  Tracabilite: {}", style(trace_path.display()).cyan());
            }
        }
    }

    Ok(())
}

fn create_pipeline(config: &Config) -> Result<Pipeline> {
    let llm = Arc::new(OllamaAdapter::new(config.llm.clone())?);
    let templates = Arc::new(FileTemplateEngine::new(&config.templates.directory)?);
    Ok(Pipeline::new(llm, templates, config.clone()))
}

async fn check_llm(config: &Config) -> Result<()> {
    println!(
        "{} Verification de la connexion LLM...",
        style(">>").cyan().bold()
    );
    println!(
        "  Provider: {}, Modele: {}, URL: {}",
        config.llm.provider, config.llm.model_name, config.llm.api_base_url
    );

    let adapter = OllamaAdapter::new(config.llm.clone())?;

    if adapter.is_ready().await {
        println!("{} Ollama est accessible", style("OK").green().bold());

        match adapter.check_model().await {
            Ok(true) => {
                println!(
                    "{} Modele '{}' disponible",
                    style("OK").green().bold(),
                    config.llm.model_name
                );
            }
            Ok(false) => {
                println!(
                    "{} Modele '{}' non trouve. Executez: ollama pull {}",
                    style("!!").red().bold(),
                    config.llm.model_name,
                    config.llm.model_name
                );
            }
            Err(e) => {
                println!(
                    "{} Erreur verification modele: {}",
                    style("!!").red().bold(),
                    e
                );
            }
        }
    } else {
        println!(
            "{} Ollama n'est pas accessible a {}",
            style("!!").red().bold(),
            config.llm.api_base_url
        );
        println!("  Lancez Ollama: ollama serve");
    }

    Ok(())
}

async fn load_constitution(path: Option<&str>) -> Result<Option<String>> {
    match path {
        Some(p) => {
            let content = tokio::fs::read_to_string(p).await?;
            Ok(Some(content))
        }
        None => Ok(None),
    }
}
