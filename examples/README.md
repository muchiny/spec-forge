# 📚 Examples — Exemples de User Stories

> Ce dossier contient des exemples de User Stories prêts à l'emploi
> pour tester et démontrer les capacités de spec-forge.

---

## 🏗️ Vue d'ensemble

```mermaid
graph LR
    subgraph "📚 Exemples disponibles"
        S1["📝 sample_us.md<br/><i>Bibliothèque simple</i>"]
        S2["🛒 ecommerce_platform.md<br/><i>E-commerce complet</i>"]
        S3["📊 saas_project_management.md<br/><i>SaaS gestion projet</i>"]
        S4["🏦 mobile_banking.yaml<br/><i>Banque mobile (YAML)</i>"]
    end

    subgraph "🔄 Pipeline"
        PIP["spec-forge pipeline"]
    end

    subgraph "📤 Sorties"
        SPEC["📋 Spécifications"]
        FEAT["🧪 Tests .feature"]
        TRACE["📊 Traçabilité"]
    end

    S1 & S2 & S3 & S4 --> PIP
    PIP --> SPEC & FEAT & TRACE

    style S1 fill:#4CAF50,stroke:#333,color:#fff
    style S2 fill:#FF9800,stroke:#333,color:#fff
    style S3 fill:#2196F3,stroke:#333,color:#fff
    style S4 fill:#9C27B0,stroke:#333,color:#fff
    style PIP fill:#F44336,stroke:#333,color:#fff
```

---

## 📁 Fichiers

| Fichier | Format | Domaine | Complexité | User Stories |
|---------|--------|---------|------------|--------------|
| 📝 `user_stories/sample_us.md` | Markdown | 📖 Bibliothèque | ⭐ Simple | ~3 US |
| 🛒 `user_stories/ecommerce_platform.md` | Markdown | 🛍️ E-commerce | ⭐⭐⭐ Complexe | ~10+ US |
| 📊 `user_stories/saas_project_management.md` | Markdown | 📋 Gestion projet | ⭐⭐⭐ Complexe | ~10+ US |
| 🏦 `user_stories/mobile_banking.yaml` | YAML | 🏦 Banque mobile | ⭐⭐ Moyen | ~8 US |

---

## 🚀 Utilisation

### ⚡ Pipeline complet sur un exemple

```bash
# 📝 Exemple simple (bibliothèque)
cargo run -- pipeline -i examples/user_stories/sample_us.md -o output/

# 🛒 Exemple complexe (e-commerce)
cargo run -- pipeline -i examples/user_stories/ecommerce_platform.md -o output/

# 🏦 Exemple YAML (banque mobile)
cargo run -- pipeline -i examples/user_stories/mobile_banking.yaml -o output/
```

### 🔧 Raffinement seul

```bash
cargo run -- refine -i examples/user_stories/sample_us.md -o output/specs/
```

---

## 📝 Format Markdown attendu

```markdown
# User Stories - Mon Projet

## Titre de la US

En tant que [acteur], je veux [action]
afin de [bénéfice].

- Critère d'acceptation 1
- Critère d'acceptation 2
```

## 📄 Format YAML attendu

```yaml
project: Mon Projet
user_stories:
  - title: Titre de la US
    actor: acteur
    action: action souhaitée
    benefit: bénéfice attendu
    priority: P1
    acceptance_criteria:
      - Critère d'acceptation 1
      - Critère d'acceptation 2
```

---

## 🌍 Multi-langue

| Langue | Détection | Format Gherkin |
|--------|-----------|----------------|
| 🇫🇷 Français | `"En tant que..."` | Soit / Quand / Alors |
| 🇬🇧 Anglais | `"As a..."` | Given / When / Then |
