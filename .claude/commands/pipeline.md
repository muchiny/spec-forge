# Skill: Pipeline spec-forge

Lance le pipeline complet de spec-forge (User Stories -> Specifications -> Tests Gherkin).

## Instructions

1. Verifie d'abord que Ollama est accessible :
   ```bash
   cargo run -- check
   ```
   Si Ollama n'est pas accessible, indique a l'utilisateur de lancer `ollama serve` et de s'assurer que le modele est telecharge (`ollama pull qwen3:8b`).

2. Identifie les fichiers d'entree. Si l'utilisateur a specifie un fichier, utilise-le. Sinon, liste les fichiers disponibles dans `examples/` et `input/` et demande lequel utiliser.

3. Lance le pipeline :
   ```bash
   cargo run -- pipeline -i <fichier_entree> -o output
   ```

4. Apres execution, affiche un resume :
   - Nombre de specifications generees
   - Nombre de tests Gherkin generes
   - Chemin des fichiers de sortie
   - Toute clarification necessaire

5. Si des erreurs surviennent (parsing JSON du LLM, timeout), propose :
   - Relancer avec un modele different
   - Augmenter le timeout dans config.yaml
   - Verifier les templates dans `templates/`

Arguments optionnels : $ARGUMENTS (fichier d'entree, ex: `examples/sample_us.md`)
