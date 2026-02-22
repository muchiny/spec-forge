# Skill: Creer un nouvel adapter

Aide a creer un nouvel adapter en respectant l'architecture hexagonale de spec-forge.

## Instructions

1. Demande a l'utilisateur quel type d'adapter il veut creer :
   - **LLM** (nouveau provider LLM, ex: OpenAI, Anthropic) -> implemente `LlmService`
   - **Input** (nouveau format d'entree, ex: JSON, CSV) -> implemente `InputReader`
   - **Output** (nouveau format de sortie, ex: HTML, PDF) -> implemente `OutputWriter`
   - **Template** (nouveau moteur de templates) -> implemente `TemplateEngine`

2. Identifie le trait port correspondant dans `src/ports/` et lis-le pour comprendre le contrat.

3. Cree le fichier adapter dans le bon sous-dossier de `src/adapters/` :
   - `src/adapters/llm/<nom>_adapter.rs`
   - `src/adapters/input/<nom>_reader.rs`
   - `src/adapters/output/<nom>_writer.rs`
   - `src/adapters/templates/<nom>_template_engine.rs`

4. Implemente le trait avec :
   - `#[async_trait]` si le trait est async
   - Gestion d'erreur avec les types d'erreur du domaine
   - Documentation `///` en francais
   - Un constructeur `new()` avec les parametres necessaires

5. Mets a jour le `mod.rs` du sous-dossier pour exporter le nouveau module.

6. Ajoute des tests unitaires inline `#[cfg(test)]` dans le fichier.

7. Si des crates externes sont necessaires, indique quoi ajouter dans `Cargo.toml`.

8. Lance `cargo check` pour verifier la compilation.

Arguments : $ARGUMENTS (type d'adapter et nom, ex: "llm anthropic" ou "input json")
