# language: fr

@US-008 @P2
Fonctionnalité: Coffre-fort de documents bancaires
  Permet à l'utilisateur de consulter et de télécharger ses documents bancaires depuis l'application.

  Contexte:
    Soit L'utilisateur est connecté et a sélectionné un document.

  @happy_path @FR-005
  Plan du Scénario: Scenario happy path - Telechargement de document
    Soit L'utilisateur est connecté et a sélectionné un document.
    Quand L'utilisateur tente de le télécharger.
    Alors Le téléchargement est protégé par la biométrie ou le code PIN.

    Exemples:
      | type_document |
      | relevé |
      | contrat |
      | attestation |

  @edge_case @FR-005
  Plan du Scénario: Scenario cas limite - Telechargement de document sans validation de la biometrie
    Soit L'utilisateur est connecté et a sélectionné un document mais n'a pas validé la biometrie.
    Quand L'utilisateur tente de le télécharger.
    Alors Le téléchargement est refusé et un message d'erreur est affiché.

    Exemples:
      | etat_biometrie |
      | non_validee |

  @error @FR-005
  Plan du Scénario: Scenario erreur - Telechargement de document sans code PIN
    Soit L'utilisateur est connecté et a sélectionné un document mais n'a pas entré le code PIN.
    Quand L'utilisateur tente de le télécharger.
    Alors Le téléchargement est refusé et un message d'erreur est affiché.

    Exemples:
      | etat_code_pin |
      | non_entre |

