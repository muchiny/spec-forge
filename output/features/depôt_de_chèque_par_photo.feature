# language: fr

@US-006 @P2
Fonctionnalité: Depôt de chèque par photo
  Permet à l'utilisateur de déposer un chèque en le photographiant avec l'application.

  Contexte:
    Soit L'utilisateur a photographié le recto et le verso d'un chèque.

  @happy_path @FR-005
  Plan du Scénario: Scenario happy path - Depôt de chèque par photo
    Soit L'utilisateur a photographié le recto et le verso d'un chèque.
    Quand L'utilisateur valide les informations extraites.
    Alors Le chèque est soumis et un statut est affiché.

    Exemples:
      | type_chèque |
      | recto |
      | verso |
      | recto_et_verso |

  @edge_case @FR-005
  Plan du Scénario: Scenario cas limite - Depôt de chèque avec photo de mauvaise qualité
    Soit L'utilisateur a photographié le recto et le verso d'un chèque avec une photo de mauvaise qualité.
    Quand L'utilisateur valide les informations extraites.
    Alors Le chèque est refusé et un message d'erreur est affiché.

    Exemples:
      | qualite_photo |
      | mauvaise_qualite |

  @error @FR-005
  Plan du Scénario: Scenario erreur - Depôt de chèque sans validation de la biometrie
    Soit L'utilisateur a photographié le recto et le verso d'un chèque mais n'a pas validé la biometrie.
    Quand L'utilisateur valide les informations extraites.
    Alors Le chèque est refusé et un message d'erreur est affiché.

    Exemples:
      | etat_biometrie |
      | non_validee |

