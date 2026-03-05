# language: fr

@US-008 @P2
Fonctionnalité: Téléchargement sécurisé des documents bancaires
  Fonctionnalité permettant à l'utilisateur de consulter et de télécharger ses documents bancaires depuis l'application pour accéder à ses relevés, contrats et attestations sans attendre le courrier postal.

  Contexte:
    Soit L'utilisateur est connecté et a sélectionné un document.

  @happy_path @FR-055 @FR-048 @FR-054
  Plan du Scénario: Téléchargement sécurisé des documents bancaires (happy path)
    Soit L'utilisateur est connecté et a sélectionné un document.
    Quand L'utilisateur tente de le télécharger.
    Alors Le téléchargement est protégé par la biométrie ou le code PIN.

    Exemples:
      | document_type | format |
      | Relevé de compte | PDF |
      | Contrat | PDF |
      | Attestation | PDF |
      | RIB | PDF |
      | IBAN | PDF |
      | Document bancaire | PDF |

  @edge_case @FR-055 @FR-048 @FR-054
  Plan du Scénario: Téléchargement sécurisé des documents bancaires (cas limite - absence de biométrie ou code PIN)
    Soit L'utilisateur est connecté et a sélectionné un document.
    Quand L'utilisateur tente de le télécharger sans avoir validé la biométrie ou le code PIN.
    Alors Le système bloque le téléchargement.
    Et Le système affiche un message d'erreur indiquant la nécessité de la biométrie ou du code PIN.

    Exemples:
      | document_type | format |
      | Relevé de compte | PDF |
      | Contrat | PDF |
      | Attestation | PDF |
      | RIB | PDF |
      | IBAN | PDF |
      | Document bancaire | PDF |

  @error @FR-055 @FR-048 @FR-054
  Plan du Scénario: Téléchargement sécurisé des documents bancaires (erreur - absence de données)
    Soit L'utilisateur est connecté et a sélectionné un document.
    Quand L'utilisateur tente de le télécharger.
    Alors Le système affiche un message d'erreur indiquant l'absence de document.
    Et Le téléchargement n'est pas effectué.

    Exemples:
      | document_type | format |
      | Relevé de compte | PDF |
      | Contrat | PDF |
      | Attestation | PDF |
      | RIB | PDF |
      | IBAN | PDF |
      | Document bancaire | PDF |

