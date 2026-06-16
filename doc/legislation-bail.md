# Législation — Bail d'Habitation Meublé : Guide de Référence Agent

> **Source** : Service-Public.fr / Direction de l'information légale et administrative (Premier ministre)  
> **Vérifié le** : 06 juin 2025  
> **Loi principale** : Loi n°89-462 du 6 juillet 1989 tendant à améliorer les rapports locatifs  
> **Scope** : Logement meublé, bail signé depuis le 20 janvier 2025

## Usage de ce fichier

Ce document est une adaptation structurée de la législation applicable aux baux meublés, destinée à guider le développement d'un logiciel d'édition de baux en ligne. Il décrit les obligations légales, les champs requis, les règles de validation et les contraintes de contenu.

> **Bail mobilité** : fait l'objet de règles spécifiques (https://www.service-public.gouv.fr/particuliers/vosdroits/F34759) non couvertes dans ce document.

---

## 1. Forme du bail

| Règle | Détail |
|-------|--------|
| Support | Écrit obligatoire (acte sous signature privée ou acte authentique) |
| Nombre d'exemplaires | Autant qu'il y a de parties (propriétaire + chaque locataire/colocataire) |
| Modèle réglementaire | Conforme à l'Annexe 2 du Décret n°2015-587 (logement meublé) — voir bail-template.md |
| Clause résolutoire | **Obligatoire** — résiliation de plein droit si non-paiement loyer/charges, non-versement dépôt de garantie, non-souscription assurance locative |

---

## 2. Contenu obligatoire du bail

### 2.1 Informations sur les parties

| Champ | Obligatoire | Notes |
|-------|-------------|-------|
| Nom et domicile du propriétaire | Oui | |
| Nom, adresse et siège social du gestionnaire | Conditionnel | Si logement non géré directement par le propriétaire |
| Nom(s) du ou des locataires | Oui | |
| Date de prise d'effet du bail | Oui | |
| Durée du bail | Oui | |

### 2.2 Informations sur le logement

| Champ | Obligatoire | Notes / Risques légaux |
|-------|-------------|----------------------|
| Identifiant fiscal du logement (IFL) | Oui | Sauf Guadeloupe, Martinique, Guyane, La Réunion, Mayotte |
| Localisation et destination de l'immeuble | Oui | |
| Description du logement | Oui | Maison ou appartement, copropriété ou non, nombre de pièces, équipements privatifs et communs |
| Nature et montant des travaux depuis le dernier contrat | Oui | Travaux d'amélioration ou de mise en conformité décence |
| Classe DPE | Oui | Sauf DOM-TOM |
| Surface habitable (m²) | Oui | **Critique** : absence ou erreur expose le bailleur à une action en diminution de loyer |

#### Risques liés à la surface habitable

| Situation | Recours du locataire | Délais |
|-----------|---------------------|--------|
| Surface absente | Mise en demeure (1 mois) puis saisine juge si refus | Saisine dans les 3 mois de la mise en demeure |
| Surface erronée (supérieure à la réalité) | Demande de diminution proportionnelle à l'écart | Réponse bailleur : 2 mois ; saisine juge : 4 mois après demande |
| Réduction accordée — demande dans les 6 premiers mois | Applicable à la date de signature | |
| Réduction accordée — demande après 6 mois | Applicable à la date de la demande | |

### 2.3 Informations sur le loyer, les charges et le dépôt de garantie

#### Cas général (toutes zones)

| Champ | Obligatoire |
|-------|-------------|
| Montant du loyer et modalités de paiement (date, fréquence) | Oui |
| Règles de révision du loyer | Oui |
| Montant du dernier loyer du précédent locataire | Conditionnel — si départ < 18 mois |
| Modalités de récupération des charges (forfait ou réel) | Oui |
| Montant du forfait de charges | Conditionnel — si forfait |
| Montant du dépôt de garantie | Conditionnel — si prévu |
| Montant des dépenses théoriques de chauffage + année de référence des prix de l'énergie | Oui |

#### Zones avec encadrement des loyers — Champs supplémentaires

Ces zones nécessitent l'ajout du loyer de référence et du loyer de référence majoré dans le bail :

| Zone | Lien référence des loyers |
|------|--------------------------|
| Paris | https://www.service-public.gouv.fr/particuliers/vosdroits/R46641 |
| Bordeaux | https://www.service-public.gouv.fr/particuliers/vosdroits/R62916 |
| Lille, Hellemmes, Lomme | https://www.service-public.gouv.fr/particuliers/vosdroits/R55687 |
| Lyon, Villeurbanne | https://www.service-public.gouv.fr/particuliers/vosdroits/R61528 |
| Montpellier | https://www.service-public.gouv.fr/particuliers/vosdroits/R62655 |
| Est Ensemble | https://www.service-public.gouv.fr/particuliers/vosdroits/R61539 |
| Grenoble-Alpes Métropole (partiel) | https://www.service-public.gouv.fr/particuliers/vosdroits/R72371 (simulateur) |
| Pays Basque | https://www.service-public.gouv.fr/particuliers/vosdroits/R70609 |
| Plaine Commune | https://www.service-public.gouv.fr/particuliers/vosdroits/R59027 |

**Pour les zones encadrées, ajouter** :
- Montant du loyer de référence (EUR/m²)
- Montant du loyer de référence majoré (EUR/m²)
- Montant et justification du complément de loyer (si applicable)

**Simulateur** pour identifier si une commune est concernée : https://www.service-public.gouv.fr/particuliers/vosdroits/R75612

### 2.4 Frais d'agence (si mandataire professionnel)

Obligation de reproduire l'article 5 I de la loi du 6 juillet 1989 (alinéas 1 à 3) et d'indiquer les plafonds applicables au locataire.

| Règle | Détail |
|-------|--------|
| Honoraires mise en location | A la charge exclusive du bailleur |
| Honoraires visite + dossier + bail | Partagés ; part locataire <= part bailleur ET <= plafond/m² réglementaire |
| Honoraires état des lieux d'entrée | Partagés ; part locataire <= part bailleur ET <= plafond/m² réglementaire |
| Echéance paiement | A la signature du bail (visite/dossier/bail) ou à la réalisation (état des lieux) |

### 2.5 Clauses interdites

Les clauses suivantes sont réputées non écrites même si elles figurent dans le bail :

| Clause interdite |
|-----------------|
| Prélèvement automatique imposé comme seul mode de paiement |
| Interdiction d'héberger des personnes ne vivant pas habituellement avec le locataire |
| Frais de délivrance ou d'envoi de quittance |
| Toute clause contraire aux dispositions d'ordre public de la loi du 6 juillet 1989 |

---

## 3. Documents annexes obligatoires

### 3.1 Dossier de diagnostic technique (DDT)

| Document | Obligatoire | Condition |
|----------|-------------|-----------|
| Diagnostic de performance énergétique (DPE) | Oui | Valide |
| Constat de risque d'exposition au plomb (Crep) | Oui | |
| Etat de l'installation intérieure d'électricité | Conditionnel | Installation > 15 ans |
| Etat de l'installation intérieure de gaz | Conditionnel | Installation > 15 ans |
| Etat des risques (naturels, miniers, technologiques, sismiques, radon) | Conditionnel | Si zone concernée |
| Diagnostic bruit | Conditionnel | Si zone concernée |
| Diagnostic amiante | Recommandé | Pas obligatoire ; recommandé de le tenir à disposition |

> Le DDT doit être établi par un **diagnostiqueur certifié**. Il est transmis au locataire par mail sauf opposition de celui-ci.

### 3.2 Autres documents annexes

| Document | Obligatoire | Condition |
|----------|-------------|-----------|
| Notice d'information (droits et obligations) | Oui | Arrêté du 29 mai 2015 |
| Etat des lieux d'entrée | Oui | Etabli à la remise des clés |
| Etat des lieux de sortie | Oui | Etabli à la restitution des clés |
| Inventaire et état détaillé du mobilier | Oui | Logement meublé — établi à la remise et à la restitution |
| Attestation d'assurance risques locatifs | Oui | Fournie par le locataire |
| Equipements d'accès TV/internet (TNT, fibre...) | Oui | |
| Acte de cautionnement | Conditionnel | Si caution exigée par le bailleur |
| Grille de vétusté | Conditionnel | Si convenu entre les parties |
| Extrait règlement de copropriété | Conditionnel | Si immeuble en copropriété |
| Autorisation préalable de mise en location | Conditionnel | Si zone d'habitat indigne |

> L'inventaire et l'état du mobilier doivent être signés par les deux parties. Leur établissement **ne donne lieu à aucune facturation** supplémentaire.

---

## 4. Durée du bail

| Type | Durée | Renouvellement |
|------|-------|----------------|
| Standard | 1 an minimum | Tacite, annuel, mêmes conditions |
| Locataire étudiant | 9 mois | **Pas de reconduction tacite** — fin à l'échéance ; nouveau bail si poursuite |

---

## 5. Références légales

| Texte | Objet |
|-------|-------|
| Loi n°89-462 du 6 juillet 1989, art. 3 | Mentions obligatoires du bail |
| Loi n°89-462 du 6 juillet 1989, art. 3-1 | Ecart de superficie |
| Loi n°89-462 du 6 juillet 1989, art. 3-3 | Dossier de diagnostic |
| Loi n°89-462 du 6 juillet 1989, art. 5 | Mentions frais d'agence |
| Loi n°89-462 du 6 juillet 1989, art. 10 | Durée du bail |
| Loi n°89-462 du 6 juillet 1989, art. 17 | Complément de loyer |
| Décret n°2015-587 du 29 mai 2015 | Contrats types de location — modèle réglementaire |
| Décret n°2015-650 du 10 juin 2015 | Encadrement des loyers — complément de loyer (art. 3) |
| Décret n°2015-981 du 31 juillet 2015 | Liste des éléments de mobilier d'un logement meublé |
| Décret n°2016-382 du 30 mars 2016 | Etat des lieux et grille de vétusté |
| Décret n°2016-1104 du 11 août 2016 | Diagnostic gaz |
| Décret n°2016-1105 du 11 août 2016 | Diagnostic électricité |
| Code de la santé publique R1334-29-4 à R1334-29-7 | Diagnostic amiante |
| Arrêté du 29 mai 2015 | Notice d'information à annexer au bail |

---

## 6. Critères de mobilier pour un logement meublé (Décret n°2015-981)

Eléments minimum obligatoires depuis le 1er septembre 2015 :

| Elément | Obligatoire |
|---------|-------------|
| Literie avec couette ou couverture | Oui |
| Volets ou rideaux dans les chambres | Oui |
| Plaques de cuisson | Oui |
| Four ou four à micro-ondes | Oui |
| Réfrigérateur | Oui |
| Congélateur ou compartiment à congélation | Oui |
| Vaisselle en nombre suffisant pour les repas | Oui |
| Ustensiles de cuisine | Oui |
| Table et sièges | Oui |
| Etagères de rangement | Oui |
| Luminaires | Oui |
| Matériel d'entretien ménager adapté | Oui |

---

## 7. Implications pour le logiciel d'édition de bail

### Interface utilisateur recommandée

1. **Etape 1** : Saisie des parties (bailleur, mandataire éventuel, locataire(s))
2. **Etape 2** : Description du logement avec détection automatique de la zone selon la commune saisie
3. **Etape 3** : Conditions financières avec champs conditionnels selon la zone (encadrée ou non)
4. **Etape 4** : Durée et date de prise d'effet avec impact automatique sur le renouvellement
5. **Etape 5** : Travaux, garanties, clauses particulières
6. **Etape 6** : Récapitulatif des annexes requises avec checklist

### Validations à implémenter côté serveur

- Surface habitable non nulle et > 0
- Dépôt de garantie <= 2 × loyer hors charges
- Si zone encadrée : loyer <= loyer de référence majoré (sauf complément justifié)
- Honoraires locataire <= honoraires bailleur ET <= plafond/m²
- Blocage si classe DPE < seuil légal en vigueur à la date du bail

### Clauses auto-générées (non modifiables par l'utilisateur)

- Clause résolutoire (obligatoire légalement)
- Reproduction article 5 I de la loi du 6 juillet 1989 (si mandataire)

### Données à stocker pour chaque bail

Voir la modélisation dans `DATABASE_SCHEMA.md` — entité `leases` et tables associées.