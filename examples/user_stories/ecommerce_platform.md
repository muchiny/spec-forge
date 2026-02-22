# User Stories - Plateforme E-Commerce "ShopFlow"

## Recherche produits avec suggestions intelligentes

En tant que client, je veux rechercher des produits avec des suggestions en temps reel afin de trouver rapidement ce que je cherche meme si je ne connais pas le nom exact.

- La barre de recherche affiche des suggestions apres 3 caracteres saisis
- Les suggestions incluent des noms de produits, des categories et des marques
- Le temps de reponse des suggestions est inferieur a 200ms
- Les fautes de frappe courantes sont corrigees automatiquement (ex: "telphone" → "telephone")
- L'historique des 10 dernieres recherches de l'utilisateur est propose en priorite
- La recherche fonctionne en mode hors-ligne avec le cache local des produits consultes

## Panier d'achat persistant

En tant que client connecte, je veux retrouver mon panier d'achat sur tous mes appareils afin de pouvoir commencer mes achats sur mobile et finaliser sur ordinateur.

- Le panier est synchronise entre les sessions en temps reel
- Les produits dont le stock est epuise sont signales avec un bandeau orange
- Le prix total est recalcule automatiquement si un prix change entre deux visites
- L'utilisateur est notifie par email si un produit de son panier est en rupture depuis 48h
- Le panier conserve les articles pendant 30 jours maximum
- Un panier abandonner depuis 24h declenche un email de rappel avec 5% de reduction

## Paiement securise multi-moyens

En tant que client, je veux pouvoir payer par carte bancaire, PayPal ou virement SEPA afin de choisir le moyen de paiement qui me convient.

- Les paiements par carte sont traites via Stripe avec 3D Secure obligatoire au-dessus de 30 euros
- Le tunnel de paiement ne depasse pas 3 etapes (recapitulatif, paiement, confirmation)
- Les donnees de carte sont tokenisees et jamais stockees en clair sur nos serveurs
- En cas d'echec de paiement, l'utilisateur peut reessayer sans ressaisir ses informations
- Un recu PDF est genere et envoye par email dans les 5 minutes suivant le paiement
- Les paiements par virement SEPA sont confirmes sous 2 jours ouvrables maximum
- Le systeme supporte les paiements en 3 fois sans frais pour les commandes superieures a 150 euros

## Gestion des retours et remboursements

En tant que client, je veux pouvoir initier un retour en ligne afin de renvoyer un produit sans devoir contacter le service client par telephone.

- Le bouton "Retourner cet article" est disponible pendant 30 jours apres la livraison
- L'utilisateur doit selectionner un motif de retour parmi une liste predefinite
- Une etiquette de retour prepayee est generee au format PDF
- Le remboursement est effectue sous 14 jours apres reception du colis retourne
- L'utilisateur peut suivre l'etat de son retour en temps reel (initie, expedie, recu, rembourse)
- Les articles personnalises ou perissables ne sont pas retournables, le bouton est masque

## Tableau de bord vendeur

En tant que vendeur partenaire, je veux consulter un tableau de bord avec mes statistiques de ventes afin de piloter mon activite sur la plateforme.

- Le dashboard affiche le chiffre d'affaires du jour, de la semaine et du mois
- Un graphique montre l'evolution des ventes sur les 12 derniers mois
- Les commandes en attente de traitement sont mises en evidence avec un compteur
- Le taux de retour est affiche par categorie de produit
- Les alertes de stock bas (moins de 5 unites) sont visibles en temps reel
- L'export CSV des donnees de ventes est disponible pour la comptabilite
- Le temps de chargement du dashboard est inferieur a 3 secondes

## Systeme d'avis et de notation

En tant que client ayant recu sa commande, je veux pouvoir noter et commenter les produits achetes afin d'aider les autres acheteurs dans leur choix.

- L'utilisateur peut attribuer une note de 1 a 5 etoiles
- Un commentaire textuel de 50 a 2000 caracteres est obligatoire avec la note
- L'ajout de photos (maximum 5, formats JPG/PNG, 5 Mo max chacune) est optionnel
- Les avis sont moderes automatiquement pour filtrer les injures et le spam
- Le vendeur peut repondre publiquement a chaque avis
- L'avis n'est publie que si la commande est confirmee comme livree
- La note moyenne d'un produit est mise a jour en temps reel sur la fiche produit

## Gestion de l'inventaire multi-entrepots

En tant que gestionnaire logistique, je veux visualiser et gerer le stock reparti sur plusieurs entrepots afin d'optimiser les expeditions et eviter les ruptures.

- La vue consolidee affiche le stock total et par entrepot pour chaque produit
- Un transfert inter-entrepots peut etre initie en 3 clics maximum
- Le systeme propose automatiquement un reapprovisionnement quand le stock passe sous le seuil d'alerte
- L'historique des mouvements de stock est consultable sur 2 ans avec filtres par date, produit et entrepot
- La synchronisation avec le systeme ERP se fait toutes les 15 minutes
- En cas de commande, l'entrepot le plus proche du client est selectionne automatiquement

## Notifications push personnalisees

En tant que client ayant installe l'application mobile, je veux recevoir des notifications pertinentes afin d'etre informe des promotions et du suivi de mes commandes.

- Les notifications de suivi de commande sont envoyees a chaque changement de statut
- Les promotions ciblees sont basees sur l'historique d'achat (maximum 2 par semaine)
- L'utilisateur peut configurer ses preferences de notification par categorie
- Les notifications ne sont pas envoyees entre 22h et 8h sauf pour les livraisons express
- Un lien profond dans la notification ouvre directement la page concernee dans l'app
- Le taux de delivrabilite des notifications est superieur a 95%

## Programme de fidelite a points

En tant que client fidele, je veux cumuler des points de fidelite sur mes achats afin de beneficier de reductions sur mes futures commandes.

- 1 euro depense = 1 point de fidelite
- Les points sont credites 48h apres la confirmation de livraison
- A partir de 500 points, l'utilisateur peut convertir en bon de reduction (500 points = 5 euros)
- Le solde de points est visible dans le profil et sur le recapitulatif de commande
- Les points expirent 12 mois apres leur derniere utilisation
- Les retours entrainent le retrait automatique des points correspondants
- Les offres exclusives sont proposees aux clients ayant plus de 1000 points

## Administration et moderation de la plateforme

En tant qu'administrateur, je veux pouvoir moderer le contenu et gerer les utilisateurs afin de garantir la qualite et la securite de la plateforme.

- La recherche d'utilisateur fonctionne par nom, email ou numero de commande
- Un utilisateur peut etre suspendu temporairement (1 jour, 1 semaine, 1 mois) ou banni definitivement
- Les avis signales par les utilisateurs apparaissent dans une file de moderation prioritaire
- Le journal d'audit enregistre toutes les actions administratives avec horodatage et identifiant
- Les statistiques de moderation (avis supprimes, comptes suspendus) sont consultables sur le dashboard admin
- Le systeme detecte automatiquement les tentatives de creation de comptes multiples par la meme personne
