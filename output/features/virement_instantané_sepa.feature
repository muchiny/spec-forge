# language: fr

@US-003 @P1
Fonctionnalité: Virement instantané SEPA
  Permet à l'utilisateur d'effectuer un virement instantané vers un autre compte bancaire en zone SEPA.

  Contexte:
    Soit L'utilisateur est connecté et a sélectionné un bénéficiaire.

  @happy_path @FR-005
  Plan du Scénario: Scenario happy path - Virement instantané SEPA
    Soit L'utilisateur est connecté et a sélectionné un bénéficiaire.
    Quand L'utilisateur valide le virement.
    Alors Le virement est exécuté en moins de 10 secondes.

    Exemples:
      | montant_virement |
      | 100.00 |
      | 500.00 |

  @edge_case @FR-005
  Plan du Scénario: Scenario cas limite - Virement sans code PIN
    Soit L'utilisateur est connecté mais n'a pas entré le code PIN.
    Quand L'utilisateur valide le virement.
    Alors Le virement est refusé et un message d'erreur est affiché.

    Exemples:
      | etat_code_pin |
      | non_entre |

  @error @FR-005
  Plan du Scénario: Scenario erreur - Virement avec photo de mauvaise qualité
    Soit L'utilisateur est connecté et a sélectionné un bénéficiaire.
    Quand L'utilisateur valide le virement.
    Alors Le virement est refusé et un message d'erreur est affiché.

    Exemples:
      | qualite_photo |
      | mauvaise_qualite |

