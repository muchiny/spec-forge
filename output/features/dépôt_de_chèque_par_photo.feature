# language: fr

@US-006 @P2
Fonctionnalité: Dépôt de chèque par photo
  Dépôt de chèque via l'application avec prise de photo du recto et du verso

  Contexte:
    Soit L'utilisateur a photographié le recto et le verso d'un chèque

  @happy_path @FR-035 @FR-036 @FR-037 @FR-038 @FR-040 @FR-041
  Plan du Scénario: Dépôt de chèque - Happy Path
    Quand L'utilisateur valide les informations extraites
    Alors Le chèque est soumis et un statut est affiché

    Exemples:
      | montant | statut |
      | 4500 | en cours |

  @edge_case @FR-035 @FR-036 @FR-037 @FR-038 @FR-040 @FR-041
  Plan du Scénario: Dépôt de chèque - Cas limite
    Quand L'utilisateur tente de déposer un chèque avec une photo de mauvaise qualité
    Alors Le système refuse le dépôt et affiche un message d'erreur

    Exemples:
      | montant | statut |
      | 4500 | en cours |

  @error @FR-035 @FR-036 @FR-037 @FR-038 @FR-040 @FR-041
  Plan du Scénario: Dépôt de chèque - Erreur
    Quand L'utilisateur tente de déposer un chèque avec une photo de mauvaise qualité
    Alors Le système refuse le dépôt et affiche un message d'erreur

    Exemples:
      | montant | statut |
      | 4500 | en cours |

