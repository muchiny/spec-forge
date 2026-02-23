# 🖥️ Infrastructure — Configuration & Logging

> La couche **infrastructure** gère la configuration YAML hiérarchique et le logging structuré.
> Elle fournit les paramètres à toutes les autres couches.

---

## 🏗️ Vue d'ensemble

```mermaid
graph TB
    subgraph "🖥️ Infrastructure"
        CFG["⚙️ Config<br/><i>config.rs</i><br/>Configuration YAML"]
        LOG["📜 Logging<br/><i>logging.rs</i><br/>tracing + EnvFilter"]
    end

    subgraph "📄 Sources de config"
        YML["📄 config.yaml<br/><i>Fichier par défaut</i>"]
        ENV["🌍 Variables d'env<br/><i>SPEC_FORGE_*</i>"]
        CLI["🖥️ Arguments CLI<br/><i>clap overrides</i>"]
    end

    YML --> CFG
    ENV --> CFG
    CLI --> CFG

    CFG --> APP["⚙️ Application"]
    CFG --> ADP["🔧 Adapters"]
    LOG --> ALL["📊 Tous les modules"]

    style CFG fill:#2196F3,stroke:#333,color:#fff
    style LOG fill:#4CAF50,stroke:#333,color:#fff
    style YML fill:#FF9800,stroke:#333,color:#fff
```

---

## 📁 Fichiers

| Fichier | Rôle | Taille |
|---------|------|--------|
| ⚙️ `config.rs` | Chargement et validation de la configuration YAML | ~17 Ko |
| 📜 `logging.rs` | Initialisation de `tracing` avec filtre par niveau | ~1 Ko |

---

## ⚙️ Configuration — Sections

```mermaid
graph LR
    CFG["⚙️ Config"]
    CFG --> PIP["🔄 Pipeline<br/><i>max_retries, language,<br/>token_budget</i>"]
    CFG --> LLM["🤖 LLM<br/><i>provider, model,<br/>temperature, context</i>"]
    CFG --> OUT["📤 Output<br/><i>spec_format,<br/>gherkin_language</i>"]
    CFG --> VAL["✅ Validation<br/><i>coverage %, syntax,<br/>max_clarifications</i>"]
    CFG --> LOGG["📜 Logging<br/><i>level, format,<br/>colors</i>"]

    style CFG fill:#2196F3,stroke:#333,color:#fff
    style PIP fill:#FF9800,stroke:#333,color:#fff
    style LLM fill:#4CAF50,stroke:#333,color:#fff
    style OUT fill:#9C27B0,stroke:#333,color:#fff
```

### 📋 Paramètres clés

| Section | Paramètre | Défaut | Description |
|---------|-----------|--------|-------------|
| `llm` | `provider` | `ollama` | Provider LLM |
| `llm` | `model_name` | `qwen2.5:7b` | Modèle à utiliser |
| `llm` | `temperature` | `0.1` | Créativité (0.0–1.0) |
| `llm` | `context_size` | `8192` | Fenêtre de contexte (tokens) |
| `pipeline` | `max_retries` | `3` | Tentatives max par appel LLM |
| `pipeline` | `default_language` | `fr` | Langue par défaut |
| `output` | `gherkin_language` | `fr` | Mots-clés Gherkin (fr/en) |
| `validation` | `min_coverage_percent` | `80` | Couverture minimale exigée |
| `logging` | `level` | `info` | Niveau de log |

---

## 📜 Logging — tracing

Le système utilise `tracing` avec `EnvFilter` pour un logging structuré :

```bash
# Ajuster le niveau de log
RUST_LOG=debug cargo run -- pipeline -i input.md -o output/

# Niveaux disponibles : error, warn, info, debug, trace
```
