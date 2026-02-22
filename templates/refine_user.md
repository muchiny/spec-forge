Raffine la User Story suivante en une specification complete:

**Titre**: {{title}}
**Acteur**: {{actor}}
**Action**: {{action}}
**Benefice**: {{benefit}}
{{#if priority}}**Priorite suggeree**: {{priority}}{{/if}}

{{#if acceptance_criteria}}
**Criteres d'acceptation existants**:
{{#each acceptance_criteria}}
- {{this}}
{{/each}}
{{/if}}

{{#if raw_text}}
**Texte source complet**:
{{raw_text}}
{{/if}}

{{#if constitution}}
**Principes du projet (constitution)**:
{{constitution}}
{{/if}}
