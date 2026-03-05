# language: fr

@US-004 @P1
Fonctionnalité: Gestion des cartes bancaires
  Permet à l'utilisateur de gérer les paramètres de sa carte bancaire depuis l'application.

  Contexte:
    Soit L'utilisateur est connecté et a sélectionné une carte.

  @happy_path @FR-005
  Plan du Scénario: Scenario happy path - Activation du paiement sans contact
    Soit L'utilisateur est connecté et a sélectionné une carte.
    Quand L'utilisateur active le paiement sans contact.
    Alors L'activation prend effet en moins de 30 secondes.

    Exemples:
      | type_paiement |
      | sans_contact |

  @edge_case @FR-005
  Plan du Scénario: Scenario cas limite - Activation sans code PIN
    Soit L'utilisateur est connecté mais n'a pas entré le code PIN.
    Quand L'utilisateur active le paiement sans contact.
    Alors L'activation est refusée et un message d'erreur est affiché.

    Exemples:
      | etat_code_pin |
      | non_entre |

  @error @FR-005
  Plan du Scénario: Scenario erreur - Activation de la carte avec photo de mauvaise qualité
    Soit L'utilisateur est connecté et a sélectionné une carte.
    Quand L'utilisateur active le paiement sans contact.
    Alors L'activation est refusée et un message d'erreur est affiché.

    Exemples:
      | qualite_photo |
      | mauvaise_qualite |

