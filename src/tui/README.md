# 🖥️ TUI — Interface Terminal Interactive

> L'interface TUI offre une expérience interactive complète pour piloter spec-forge
> directement depuis le terminal, construite avec **ratatui** + **crossterm**.

---

## 🏗️ Vue d'ensemble

```mermaid
graph TB
    subgraph "🖥️ TUI"
        MOD["🎮 mod.rs<br/><i>Boucle événementielle</i>"]
        APP["📦 app.rs<br/><i>État applicatif</i>"]
        EVT["⚡ event.rs<br/><i>Gestion événements</i>"]
        UI["🎨 ui.rs<br/><i>Dispatch rendu</i>"]
        THM["🎨 theme.rs<br/><i>Thème couleurs</i>"]
        LOG["📜 log_layer.rs<br/><i>Tracing → TUI</i>"]
    end

    subgraph "📱 Screens (8)"
        S1["🏠 Dashboard"]
        S2["📂 FilePicker"]
        S3["⚙️ Pipeline"]
        S4["📋 SpecViewer"]
        S5["🧪 GherkinViewer"]
        S6["📊 Traceability"]
        S7["🔧 Config"]
        S8["📜 Logs"]
    end

    subgraph "🧩 Widgets (4)"
        W1["📊 Header"]
        W2["❓ Help"]
        W3["📈 Progress"]
        W4["ℹ️ StatusBar"]
    end

    MOD --> APP
    MOD --> EVT
    MOD --> UI
    UI --> S1 & S2 & S3 & S4 & S5 & S6 & S7 & S8
    UI --> W1 & W2 & W3 & W4

    style MOD fill:#FF9800,stroke:#333,color:#fff
    style APP fill:#2196F3,stroke:#333,color:#fff
    style S1 fill:#4CAF50,stroke:#333,color:#fff
    style S3 fill:#4CAF50,stroke:#333,color:#fff
```

---

## 📱 Écrans

```mermaid
graph LR
    S1["🏠 1 Dashboard"] --> S2["📂 2 Fichier"]
    S2 --> S3["⚙️ 3 Pipeline"]
    S3 --> S4["📋 4 Spec"]
    S4 --> S5["🧪 5 Gherkin"]
    S5 --> S6["📊 6 Traçabilité"]
    S6 --> S7["🔧 7 Config"]
    S7 --> S8["📜 8 Logs"]

    style S1 fill:#4CAF50,stroke:#333,color:#fff
    style S3 fill:#FF9800,stroke:#333,color:#fff
    style S6 fill:#9C27B0,stroke:#333,color:#fff
```

| # | Touche | Écran | Description |
|---|--------|-------|-------------|
| 1 | `1` | 🏠 Dashboard | Accueil, statut LLM, résumé projet |
| 2 | `2` | 📂 FilePicker | Navigation et sélection de fichiers |
| 3 | `3` | ⚙️ Pipeline | Lancement et suivi en temps réel |
| 4 | `4` | 📋 SpecViewer | Visualisation spécification raffinée |
| 5 | `5` | 🧪 GherkinViewer | Visualisation tests Gherkin générés |
| 6 | `6` | 📊 Traceability | Matrice de traçabilité interactive |
| 7 | `7` | 🔧 Config | Configuration actuelle |
| 8 | `8` | 📜 Logs | Journaux en temps réel |

### ⌨️ Raccourcis clavier

| Touche | Action |
|--------|--------|
| `1`–`8` | Naviguer vers un écran |
| `q` / `Ctrl+C` | Quitter |
| `Esc` | Annuler pipeline en cours |
| `↑` `↓` | Défiler dans les listes |
| `Enter` | Confirmer/sélectionner |

---

## 📁 Structure

```
tui/
├── 🎮 mod.rs          # Boucle principale, setup terminal
├── 📦 app.rs          # App state (Screen, PipelineStatus, LlmStatus)
├── ⚡ event.rs        # Polling événements clavier/tick
├── 🎨 ui.rs           # Dispatch du rendu par écran
├── 🎨 theme.rs        # Palette de couleurs
├── 📜 log_layer.rs    # Layer tracing → buffer circulaire
├── 📱 screens/
│   ├── dashboard.rs       # 🏠 Accueil
│   ├── file_picker.rs     # 📂 Sélecteur fichiers
│   ├── pipeline.rs        # ⚙️ Pipeline
│   ├── spec_viewer.rs     # 📋 Spécification
│   ├── gherkin_viewer.rs  # 🧪 Gherkin
│   ├── traceability.rs    # 📊 Traçabilité
│   ├── config.rs          # 🔧 Configuration
│   └── logs.rs            # 📜 Journaux
└── 🧩 widgets/
    ├── header.rs      # Barre de titre avec onglets
    ├── help.rs        # Aide contextuelle
    ├── progress.rs    # Barre de progression pipeline
    └── status_bar.rs  # Barre de statut en bas
```

---

## 🔄 Boucle événementielle

```mermaid
sequenceDiagram
    participant Term as 🖥️ Terminal
    participant EVT as ⚡ EventLoop
    participant APP as 📦 App State
    participant UI as 🎨 Renderer

    loop Toutes les 100ms
        EVT->>Term: poll events
        Term-->>EVT: KeyEvent / Tick
        EVT->>APP: update(event)
        APP->>APP: Transition état
        EVT->>UI: draw(frame, app)
        UI->>Term: Render widgets
    end
```

---

## 🚀 Lancement

```bash
spec-forge tui
```
