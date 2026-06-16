# Contrat Type de Location de Logement Meublé — Référence Agent

> **Source légale** : Décret n° 2015-587 du 29 mai 2015, Annexe 2, modifié par Décret n°2023-796 du 18 août 2023 (en vigueur au 1er janvier 2025)  
> **Applicable à** : locations et colocations de logement meublé constituant la résidence principale du locataire  
> **JORF** : n°0124 du 31 mai 2015 — Soumis au titre Ier bis de la loi du 6 juillet 1989

---

## Usage de ce fichier

Ce document est une adaptation structurée du contrat type légal (Annexe 2 du Décret 2015-587) destinée à guider le développement d'un logiciel d'édition de baux en ligne. Chaque section liste les champs à saisir, leur caractère obligatoire, leur type de données, et les règles métier associées.

---

## Champ d'application

### Inclus
- Locations meublées à usage de résidence principale
- Colocations meublées avec **un seul contrat** pour tous les colocataires

### Exclus
- Colocations avec plusieurs contrats distincts entre locataires et bailleur
- Logements HLM faisant l'objet d'une convention (art. L.351-2 CCH)

---

## Structure du contrat — Sections et Champs

### I. Désignation des parties

#### Bailleur (propriétaire)
| Champ | Obligatoire | Type | Notes |
|-------|-------------|------|-------|
| Nom et prénom (personne physique) OU dénomination (personne morale) | Oui | string | Préciser si SCI entre parents jusqu'au 4e degré inclus |
| Domicile ou siège social | Oui | string | |
| Qualité du bailleur | Oui | enum: `personne_physique` \| `personne_morale` | |
| Adresse électronique | Non | email | Facultatif |

#### Mandataire (si bailleur représenté)
| Champ | Obligatoire | Type | Notes |
|-------|-------------|------|-------|
| Nom ou raison sociale | Conditionnel | string | Obligatoire si mandataire présent |
| Adresse du mandataire | Conditionnel | string | |
| Activité exercée | Conditionnel | string | |
| Numéro de carte professionnelle | Conditionnel | string | Obligatoire si professionnel art. 1 loi n°70-9 du 2 jan. 1970 |
| Lieu de délivrance de la carte | Conditionnel | string | |
| Nom et adresse du garant | Conditionnel | string | |

#### Locataire(s)
| Champ | Obligatoire | Type | Notes |
|-------|-------------|------|-------|
| Nom et prénom | Oui | string | Répéter pour chaque colocataire |
| Adresse électronique | Non | email | Facultatif |

---

### II. Objet du contrat

#### A. Consistance du logement
| Champ | Obligatoire | Type | Exemples / Notes |
|-------|-------------|------|-----------------|
| Adresse complète | Oui | string | Adresse, bâtiment, étage, porte |
| Identifiant fiscal du logement (IFL) | Oui | string | Sauf DOM-TOM |
| Type d'habitat | Oui | enum: `collectif` \| `individuel` | |
| Régime juridique de l'immeuble | Oui | enum: `monopropriété` \| `copropriété` | |
| Période de construction | Oui | enum: `avant_1949` \| `1949_1974` \| `1975_1989` \| `1989_2005` \| `depuis_2005` | |
| Surface habitable (m²) | Oui | number | **Critique** — absence ou erreur expose à action en diminution de loyer |
| Nombre de pièces principales | Oui | integer | |
| Autres parties du logement | Non | string | Grenier, terrasse, balcon, loggia, jardin |
| Éléments d'équipement | Non | string | Cuisine équipée, sanitaires |
| Modalité de chauffage | Oui | enum: `individuel` \| `collectif` | Si collectif : préciser modalités de répartition |
| Modalité d'eau chaude sanitaire | Oui | enum: `individuelle` \| `collective` | Si collective : préciser modalités de répartition |
| Classe DPE | Oui | enum: `A` \| `B` \| `C` \| `D` \| `E` \| `F` \| `G` | Sauf DOM-TOM |

##### Seuils de performance énergétique minimale (France métropolitaine)
| Date d'effet | Classe DPE minimale requise |
|---|---|
| 1er janvier 2025 | F |
| 1er janvier 2028 | E |
| 1er janvier 2034 | D |

##### Seuils DOM-TOM (Guadeloupe, Martinique, Guyane, Réunion, Mayotte)
| Date d'effet | Classe DPE minimale requise |
|---|---|
| 1er janvier 2028 | F |
| 1er janvier 2031 | E |

**Règle logicielle** : afficher un avertissement si la classe DPE saisie est inférieure au seuil légal applicable à la date de signature du bail.

#### B. Destination des locaux
| Champ | Obligatoire | Type | Valeurs |
|-------|-------------|------|---------|
| Destination | Oui | enum: `habitation` \| `mixte_professionnel_habitation` | |

#### C. Locaux privatifs accessoires
| Champ | Obligatoire | Type | Exemples |
|-------|-------------|------|---------|
| Description | Non | string | Cave, parking, garage |

#### D. Parties et équipements communs
| Champ | Obligatoire | Type | Exemples |
|-------|-------------|------|---------|
| Description | Non | string | Garage à vélos, ascenseur, espaces verts, laverie, local poubelle, gardiennage |

#### E. Équipements d'accès aux technologies de l'information
| Champ | Obligatoire | Type | Exemples |
|-------|-------------|------|---------|
| Description | Non | string | TNT, fibre, modalités de réception TV/internet |

---

### III. Date de prise d'effet et durée du contrat

| Champ | Obligatoire | Type | Notes |
|-------|-------------|------|-------|
| Date de prise d'effet | Oui | date | |
| Durée du contrat | Oui | enum: `12_mois` \| `9_mois_etudiant` | 1 an standard ; 9 mois si locataire étudiant |

#### Règles de renouvellement
- **Durée 1 an** : reconduction tacite annuelle ; locataire peut donner congé à tout moment ; bailleur peut donner congé à l'échéance pour reprise, vente ou motif légitime
- **Durée 9 mois (étudiant)** : **pas** de reconduction tacite ; fin de bail à l'échéance sans congé obligatoire

---

### IV. Conditions financières

#### A. Loyer

| Champ | Obligatoire | Type | Notes |
|-------|-------------|------|-------|
| Montant du loyer mensuel (€) | Oui | number | Loyer base + complément éventuel |
| Soumis au décret d'évolution des loyers à la relocation | Conditionnel | boolean | Zones tendues |
| Soumis au loyer de référence majoré préfectoral | Conditionnel | boolean | Zones encadrées |
| Montant du loyer de référence (€/m²) | Conditionnel | number | Si zone encadrée |
| Montant du loyer de référence majoré (€/m²) | Conditionnel | number | Si zone encadrée |
| Complément de loyer (€) | Conditionnel | number | Seulement si `loyer_base` = loyer de référence majoré |
| Caractéristiques justifiant le complément de loyer | Conditionnel | string | |
| Montant du dernier loyer du précédent locataire (€) | Conditionnel | number | **Obligatoire** si départ < 18 mois |
| Date de versement du dernier loyer | Conditionnel | date | **Obligatoire** si départ < 18 mois |
| Date de dernière révision du loyer | Conditionnel | date | **Obligatoire** si départ < 18 mois |
| Date de révision annuelle | Non | string | Ex : "1er janvier" ou date anniversaire |
| Trimestre de référence IRL | Non | string | Index de Référence des Loyers |

#### B. Charges récupérables

| Champ | Obligatoire | Type | Notes |
|-------|-------------|------|-------|
| Modalité de règlement des charges | Oui | enum: `provisions_regularisation` \| `paiement_periodique` \| `forfait` | |
| Montant des provisions ou du forfait (€/mois) | Conditionnel | number | Si provisions ou forfait |
| Modalités de révision du forfait | Conditionnel | string | Si forfait : révision dans mêmes conditions que loyer |

#### C. Assurance colocataires (colocation uniquement)

| Champ | Obligatoire | Type | Notes |
|-------|-------------|------|-------|
| Bailleur souscrit assurance pour les colocataires | Conditionnel | boolean | En cas de colocation |
| Montant total annuel récupérable (€) | Conditionnel | number | Prime d'assurance annuelle, majorée dans limite du décret |
| Montant récupérable par douzième (€) | Conditionnel | number | |

#### D. Modalités de paiement

| Champ | Obligatoire | Type | Notes |
|-------|-------------|------|-------|
| Périodicité | Oui | enum: `mensuel` \| `trimestriel` | Mensuel de droit à tout moment à la demande du locataire |
| Échéance | Oui | enum: `a_echoir` \| `a_terme_echu` | |
| Date ou période de paiement | Oui | string | Ex : "le 1er de chaque mois" |
| Lieu de paiement | Non | string | Facultatif |
| Montant total à la première échéance (€) | Non | number | Loyer + charges + assurance coloc éventuelle |

#### E. Réévaluation de loyer (renouvellement uniquement)

| Champ | Obligatoire | Type | Notes |
|-------|-------------|------|-------|
| Montant de la hausse ou baisse mensuelle (€) | Conditionnel | number | Renouvellement uniquement |
| Modalité d'application annuelle | Conditionnel | enum: `par_tiers` \| `par_sixieme` | Selon durée du contrat et montant |

#### F. Dépenses énergétiques (information obligatoire)

| Champ | Obligatoire | Type | Notes |
|-------|-------------|------|-------|
| Montant estimé des dépenses annuelles d'énergie (€) | Oui | number ou string | Issu du DPE ; peut être une fourchette min-max |
| Année de référence des prix énergétiques | Oui | integer | Année figurant dans le DPE |

---

### V. Travaux

| Champ | Obligatoire | Type | Notes |
|-------|-------------|------|-------|
| Nature des travaux depuis le dernier contrat/renouvellement | Non | string | Amélioration ou mise en conformité décence |
| Montant des travaux (€) | Non | number | Préciser aussi si travaux réalisés dans les 6 derniers mois |
| Majoration loyer pour travaux bailleur en cours de bail | Non | object | Nature, modalités d'exécution, délai, montant de la majoration |
| Diminution loyer pour travaux locataire | Non | object | Durée, modalités de dédommagement si départ anticipé |

> **Note** : la clause de majoration est **invalide** pour les travaux de mise en conformité aux caractéristiques de décence.

---

### VI. Garanties

| Champ | Obligatoire | Type | Notes |
|-------|-------------|------|-------|
| Montant du dépôt de garantie (€) | Non | number | Maximum légal : **2 mois de loyer hors charges** |

---

### VII. Clause de solidarité (colocation uniquement)

| Champ | Obligatoire | Type | Notes |
|-------|-------------|------|-------|
| Clause de solidarité et indivisibilité des obligations | Conditionnel | boolean | Uniquement si plusieurs locataires |

---

### VIII. Clause résolutoire — OBLIGATOIRE ET AUTOMATIQUE

La clause résolutoire est **obligatoire** dans tout bail. Elle prévoit la résiliation de plein droit pour :
- Défaut de paiement du loyer ou des charges aux termes convenus
- Non-versement du dépôt de garantie
- Non-souscription de l'assurance risques locatifs
- Troubles de voisinage constatés par décision de justice passée en force de chose jugée

**Implémentation** : cette clause doit être générée automatiquement dans le document et non modifiable par l'utilisateur.

---

### IX. Honoraires de location (uniquement si mandataire professionnel)

> Reproduction obligatoire des alinéas 1 à 3 de l'article 5-I de la loi du 6 juillet 1989.

#### Plafonds applicables aux locataires
| Champ | Obligatoire | Type | Notes |
|-------|-------------|------|-------|
| Plafond honoraires visite/constitution dossier/rédaction bail (€/m²) | Conditionnel | number | Fixé par voie réglementaire, révisé annuellement |
| Plafond honoraires état des lieux d'entrée (€/m²) | Conditionnel | number | Fixé par voie réglementaire, révisé annuellement |

#### Honoraires à la charge du bailleur
| Champ | Type | Notes |
|-------|------|-------|
| Prestations visite/dossier/bail — détail et montant TTC (€) | string + number | Dus à la signature du bail |
| Prestation état des lieux d'entrée — montant TTC (€) | number | Dus à la réalisation |
| Autres prestations | string | |

#### Honoraires à la charge du locataire
| Champ | Type | Contrainte |
|-------|------|-----------|
| Prestations visite/dossier/bail — détail et montant TTC (€) | string + number | ≤ montant bailleur ET ≤ plafond/m² — dus à la signature |
| Prestation état des lieux d'entrée — montant TTC (€) | number | ≤ montant bailleur ET ≤ plafond/m² — dus à la réalisation |

---

### X. Autres conditions particulières

Champ libre pour clauses négociées, dans la limite des **clauses interdites** (voir section XI).

---

### XI. Annexes obligatoires

| Document | Obligatoire | Condition |
|----------|-------------|-----------|
| Extrait règlement de copropriété | Conditionnel | Si immeuble en copropriété |
| Diagnostic de performance énergétique (DPE) valide | Oui | Toujours |
| Constat de risque d'exposition au plomb (Crep) | Oui | Immeubles construits avant 1949 |
| État amiante | Conditionnel | Selon décret d'application |
| État installation intérieure électricité | Conditionnel | Installation > 15 ans |
| État installation intérieure gaz | Conditionnel | Installation > 15 ans |
| État des risques naturels et technologiques | Conditionnel | Si zone PPR ou sismique |
| Diagnostic bruit | Conditionnel | Si zone concernée |
| Notice d'information (droits et obligations) | Oui | Arrêté du 29 mai 2015 |
| État des lieux d'entrée | Oui | Établi à la remise des clés |
| Inventaire et état détaillé du mobilier | Oui | Logement meublé |
| Attestation d'assurance risques locatifs | Oui | Fournie par le locataire |
| Grille de vétusté | Conditionnel | Si convenu entre les parties |
| Autorisation préalable de mise en location | Conditionnel | Si zone d'habitat indigne |
| Références aux loyers habituellement constatés | Conditionnel | Si loyer manifestement sous-évalué |
| Acte de cautionnement | Conditionnel | Si caution exigée par le bailleur |

---

## Règles métier pour l'implémentation logicielle

### Validations critiques
- `surface_habitable` : obligatoire — absence expose le bailleur à une action en diminution de loyer
- `depot_de_garantie` : maximum = 2 × loyer mensuel hors charges
- `honoraires_locataire` : ≤ honoraires bailleur ET ≤ plafond réglementaire/m²
- `loyer_complement` : uniquement si `loyer_base` = loyer de référence majoré
- `dernier_loyer_precedent_locataire` : obligatoire si départ du précédent locataire < 18 mois avant signature

### Logique conditionnelle — Colocation
- Afficher section VII (solidarité) uniquement si plusieurs locataires
- Afficher section IX.C (assurance colocataires) uniquement si colocation
- Nombre d'exemplaires = nombre de parties (bailleur + chaque colocataire)

### Logique conditionnelle — Zone encadrée
Afficher les champs loyer de référence/majoré si la commune est dans la liste :
`Paris`, `Bordeaux`, `Lille`, `Hellemmes`, `Lomme`, `Lyon`, `Villeurbanne`, `Montpellier`, `Est Ensemble`, `Grenoble-Alpes Métropole (partiel)`, `Pays Basque`, `Plaine Commune`

### Logique durée et renouvellement
```
si locataire_etudiant == true:
  durée_contrat = 9 mois
  renouvellement_tacite = false
  afficher: "Le bail prend fin à son terme sans reconduction automatique"
sinon:
  durée_contrat = 12 mois minimum
  renouvellement_tacite = true (annuel, mêmes conditions)
```

### Clauses interdites (à bloquer dans l'interface)
- Prélèvement automatique comme seul mode de paiement
- Interdiction d'héberger des tiers
- Frais de délivrance ou d'envoi de quittance
- Toute clause contraire aux dispositions d'ordre public de la loi du 6 juillet 1989

### Clause résolutoire
Toujours incluse automatiquement dans le document généré — non modifiable, non supprimable par l'utilisateur.

### DPE et décence (avertissements logiciels)
Afficher un avertissement bloquant si la classe DPE saisie est < seuil légal à la date du bail :
- Depuis 01/01/2025 : classe G = non louable (< F exclue)
- À partir 01/01/2028 : classe F = non louable (< E exclue)
- À partir 01/01/2034 : classe E = non louable (< D exclue)

---

## Texte original de référence

Le texte légal original intégral figure ci-dessous pour consultation.

Champ du contrat type : Le présent contrat type de location est applicable aux locations et aux colocations de logement meublé et qui constitue la résidence principale du preneur, à l'exception :

- des colocations formalisées par la conclusion de plusieurs contrats entre les locataires et le bailleur ;
- des locations de logement appartenant à un organisme d'habitation à loyer modéré et faisant l'objet d'une convention passée en application de l'article L.351-2 du code de la construction et de l'habitation.

Modalités d'application du contrat type : Le régime de droit commun en matière de baux d'habitation est défini principalement par la loi n° 89-462 du 6 juillet 1989 tendant à améliorer les rapports locatifs et portant modification de la loi n° 86-1290 du 23 décembre 1986. L'ensemble de ces dispositions étant d'ordre public, elles s'imposent aux parties qui, en principe, ne peuvent pas y renoncer.
En conséquence :

- le présent contrat type de location contient uniquement les clauses essentielles du contrat dont la législation et la réglementation en vigueur au jour de sa publication imposent la mention par les parties dans le contrat. Il appartient cependant aux parties de s'assurer des dispositions applicables au jour de la conclusion du contrat.
- au-delà de ces clauses, les parties sont également soumises à l'ensemble des dispositions légales et réglementaires d'ordre public applicables aux baux d'habitation sans qu'il soit nécessaire de les faire figurer dans le contrat et qui sont rappelées utilement dans la notice d'information qui doit être jointe à chaque contrat.
- les parties sont libres de prévoir dans le contrat d'autres clauses particulières, propres à chaque location, dans la mesure où celles-ci sont conformes aux dispositions législatives et réglementaires en vigueur. Les parties peuvent également convenir de l'utilisation de tout autre support pour établir leur contrat, dans le respect du présent contrat type.

Le contrat type de location ou de colocation contient les éléments suivants :

I. Désignation des parties

Le présent contrat est conclu entre les soussignés :

- [nom et prénom, ou dénomination du bailleur / domicile ou siège social / qualité du bailleur (personne physique, personne morale,(24)) / adresse électronique (facultatif)] (25) désigné(s) ci-après le bailleur .
- Le cas échéant, représenté par le mandataire :
- [nom ou raison sociale et adresse du mandataire ainsi que l'activité exercée] ;
- Le cas échéant [numéro et lieu de délivrance de la carte professionnelle / nom et adresse du garant](26).
- [nom et prénom du ou des locataires ou, en cas de colocation, des colocataires, adresse électronique (facultatif)] désigné(s) ci-après le locataire .

Il a été convenu ce qui suit :

II. Objet du contrat

Le présent contrat a pour objet la location d'un logement ainsi déterminé :

A. Consistance du logement

- localisation du logement : [exemples : adresse / bâtiment / étage / porte etc.] ;
- identifiant fiscal du logement : [Numéro Identifiant Fiscal du logement] ;
- type d'habitat : [immeuble collectif ou individuel] ;
- régime juridique de l'immeuble : [mono propriété ou copropriété] ;
- période de construction : [exemples : avant 1949, de 1949 à 1974, de 1975 à 1989, de 1989 à 2005, depuis 2005] ;
- surface habitable : […] m2 ;
- nombre de pièces principales : […] ;
- le cas échéant, autres parties du logement : [exemples : grenier, comble aménagé ou non, terrasse, balcon, loggia, jardin etc.] ;
- le cas échéant, Eléments d'équipements du logement : [exemples : cuisine équipée, détail des installations sanitaires etc.] ;
- modalité de production chauffage : [individuel ou collectif] (27) ;
- modalité de production d'eau chaude sanitaire : [individuelle ou collective] (28) ;

- rappel : un logement décent doit respecter les critères minimaux de performance suivants :

a) En France métropolitaine :

i) A compter du 1er janvier 2025, le niveau de performance minimal correspond à la classe F du DPE ;

ii) A compter du 1er janvier 2028, le niveau de performance minimal correspond à la classe E du DPE ;

iii) A compter du 1er janvier 2034, le niveau de performance minimal correspond à la classe D du DPE.

b) En Guadeloupe, en Martinique, en Guyane, à La Réunion et à Mayotte :

i) A compter du 1er janvier 2028, le niveau de performance minimal du logement correspond à la classe F du DPE ;

ii) A compter du 1er janvier 2031, le niveau de performance minimal du logement correspond à la classe E du DPE.

La consommation d'énergie finale et le niveau de performance du logement sont déterminés selon la méthode du diagnostic de performance énergétique mentionné à l'article L. 126-26 du code de la construction et de l'habitation.

- niveau de performance du logement : [classe du diagnostic de performance énergétique].

B. Destination des locaux : [usage d'habitation ou usage mixte professionnel et d'habitation]

C. Le cas échéant, Désignation des locaux et équipements accessoires de l'immeuble à usage privatif du locataire : [exemples : cave, parking, garage etc.]

D. Le cas échéant, Enumération des locaux, parties, équipements et accessoires de l'immeuble à usage commun : [Garage à vélo, ascenseur, espaces verts, aires et équipements de jeux, laverie, local poubelle, gardiennage, autres prestations et services collectifs etc.]

E. Le cas échéant, Equipement d'accès aux technologies de l'information et de la communication : [exemples : modalités de réception de la télévision dans l'immeuble, modalités de raccordement internet etc.]

III. Date de prise d'effet et durée et du contrat

La durée du contrat et sa date de prise d'effet sont ainsi définies :

A. Date de prise d'effet du contrat : […]

B. Durée du contrat : [durée minimale d'un an ou de neuf mois si la location est consentie à un étudiant]

A l'exception des locations consenties à un étudiant pour une durée de neuf mois, les contrats de location de logements meublés sont reconduits tacitement à leur terme pour une durée d'un an et dans les mêmes conditions. Le locataire peut mettre fin au bail à tout moment, après avoir donné congé. Le bailleur peut, quant à lui, mettre fin au bail à son échéance et après avoir donné congé, soit pour reprendre le logement en vue de l'occuper lui-même ou une personne de sa famille, soit pour le vendre, soit pour un motif sérieux et légitime.
Les contrats de locations meublées consenties à un étudiant pour une durée de neuf mois ne sont pas reconduits tacitement à leur terme et le locataire peut mettre fin au bail à tout moment, après avoir donné congé. Le bailleur peut, quant à lui, mettre fin au bail à son échéance et après avoir donné congé.

IV. Conditions financières

Les parties conviennent des conditions financières suivantes :

A. Loyer

1° Fixation du loyer initial :
a) Montant du loyer mensuel : […] (29).
b) Le cas échéant, Modalités particulières de fixation initiale du loyer applicables dans certaines zones tendues (30) :

- le loyer du logement objet du présent contrat est soumis au décret fixant annuellement le montant maximum d'évolution des loyers à la relocation : [Oui / Non] ;
- le loyer du logement objet du présent contrat est soumis au loyer de référence majoré fixé par arrêté préfectoral : [Oui / Non].
- montant du loyer de référence : […] €/m2 / Montant du loyer de référence majoré : […] €/m2 ;
- Le cas échéant, Complément de loyer : [si un complément de loyer est prévu, indiquer le montant du loyer de base, nécessairement égal au loyer de référence majoré,, le montant du complément de loyer et les caractéristiques du logement justifiant le complément de loyer].

c) Le cas échéant, Informations relatives au loyer du dernier locataire : [montant du dernier loyer acquitté par le précédent locataire, date de versement et date de la dernière révision du loyer](31).
2° Le cas échéant, Modalités de révision :
a) Date de révision : […].
b) Date ou trimestre de référence de l'IRL : […].

B. Charges récupérables

1. Modalité de règlement des charges récupérables : [Provisions sur charges avec régularisation annuelle ou paiement périodique des charges sans provision ou récupération des charges par le bailleur sous la forme d'un forfait].
2. Le cas échéant, Montant des provisions sur charges ou du forfait de charges […].
3. Le cas échéant, Modalités de révision du forfait de charges : […] (32).

C. Le cas échéant, En cas de colocation, souscription par le bailleur d'une assurance pour le compte des colocataires (33) : [Oui / Non]

a) Montant total annuel récupérable au titre de l'assurance pour compte des colocataires : […] (34).
b) Montant récupérable par douzième : […].

D. Modalités de paiement

- périodicité du paiement : […(35)] ;
- paiement [à échoir / à terme échu] ;
- date ou période de paiement : […] ;
- le cas échéant, Lieu de paiement : […] ;
- le cas échéant, Montant total du à la première échéance de paiement pour une période complète de location : [Détailler la somme des montants relatifs au loyer, aux charges récupérable, à la contribution pour le partage des économies de charges et, en cas de colocation, à l'assurance récupérable pou le compte des colocataires].

E. Le cas échéant, exclusivement lors d'un renouvellement de contrat, Modalités de réévaluation d'un loyer manifestement sous évalué

3. Montant de la hausse ou de la baisse de loyer mensuelle : […].
4. Modalité d'application annuelle de la hausse : [par tiers ou par sixième selon la durée du contrat et le montant de la hausse de loyer].

F. Dépenses énergétiques (pour information)

Montant estimé des dépenses annuelles d'énergie pour un usage standard de l'ensemble des usages énumérés dans le diagnostic de performance énergétique (chauffage, refroidissement, production d'eau chaude sanitaire, éclairage et auxiliaires de chauffage, de refroidissement, d'eau chaude sanitaire et de ventilation) mentionné à l'article L. 126-26 du code de la construction et de l'habitation : [montant ou fourchette inscrit dans le diagnostic de performance énergétique] (estimation réalisée à partir des prix énergétiques de référence de l'année : [année de référence des prix énergétiques du diagnostic énergétique à l'origine de l'estimation]).

V. Travaux

A. Le cas échéant, Montant et nature des travaux d'amélioration ou de mise en conformité avec les caractéristiques de décence effectués depuis la fin du dernier contrat de location ou depuis le dernier renouvellement : […] (36)

B. Le cas échéant, Majoration du loyer en cours de bail consécutive à des travaux d'amélioration entrepris par le bailleur ou d'acquisitions d'équipements : [nature des travaux ou des équipements, modalités d'exécution, délai de réalisation ou d'acquisition ainsi que montant de la majoration du loyer] (37)

C. Le cas échéant, Diminution de loyer en cours de bail consécutive à des travaux entrepris par le locataire : [durée de cette diminution et, en cas de départ anticipé du locataire, modalités de son dédommagement sur justification des dépenses effectuées]

VI. Garanties

Le cas échéant, Montant du dépôt de garantie de l'exécution des obligations du locataire : [inférieur ou égal à deux mois de loyers hors charges].

VII. Le cas échéant, Clause de solidarité

Modalités particulières des obligations en cas de pluralité de locataires : [clause prévoyant la solidarité des locataires et l'indivisibilité de leurs obligations en cas de pluralité de locataires].

VIII. Le cas échéant, Clause résolutoire

Modalités de résiliation de plein droit du contrat : [clause prévoyant la résiliation de plein droit du contrat de location pour un défaut de paiement du loyer ou des charges aux termes convenus, le non versement du dépôt de garantie, la non-souscription d'une assurance des risques locatifs ou le non-respect de l'obligation d'user paisiblement des locaux loués, résultant de troubles de voisinage constatés par une décision de justice passée en force de chose jugée].

IX. Le cas échéant, Honoraires de location (38)

A. Dispositions applicables

Il est rappelé les dispositions du I de l'article 5 de la loi du 6 juillet 1989, alinéas 1 à 3 : La rémunération des personnes mandatées pour se livrer ou prêter leur concours à l'entremise ou à la négociation d'une mise en location d'un logement, tel que défini aux articles 2 et 25-3, est à la charge exclusive du bailleur, à l'exception des honoraires liés aux prestations mentionnées aux deuxième et troisième alinéas du présent I.
Les honoraires des personnes mandatées pour effectuer la visite du preneur, constituer son dossier et rédiger un bail sont partagés entre le bailleur et le preneur. Le montant toutes taxes comprises imputé au preneur pour ces prestations ne peut excéder celui imputé au bailleur et demeure inférieur ou égal à un plafond par mètre carré de surface habitable de la chose louée fixé par voie réglementaire et révisable chaque année, dans des conditions définies par décret. Ces honoraires sont dus à la signature du bail.
Les honoraires des personnes mandatées pour réaliser un état des lieux sont partagés entre le bailleur et le preneur. Le montant toutes taxes comprises imputé au locataire pour cette prestation ne peut excéder celui imputé au bailleur et demeure inférieur ou égal à un plafond par mètre carré de surface habitable de la chose louée fixé par voie réglementaire et révisable chaque année, dans des conditions définies par décret. Ces honoraires sont dus à compter de la réalisation de la prestation.
Plafonds applicables :

- montant du plafond des honoraires imputables aux locataires en matière de prestation de visite du preneur, de constitution de son dossier et de rédaction de bail : […] €/m2 de surface habitable ;
- montant du plafond des honoraires imputables aux locataires en matière d'établissement de l'état des lieux d'entrée : […] €/m2 de surface habitable.

B. Détail et répartition des honoraires

1. Honoraires à la charge du bailleur :

- prestations de visite du preneur, de constitution de son dossier et de rédaction de bail : [détail des prestations effectivement réalisées et montant des honoraires toutes taxes comprises dus à la signature du bail] ;
- le cas échéant, Prestation de réalisation de l'état des lieux d'entrée : [montant des honoraires toutes taxes comprises dus à compter de la réalisation de la prestation] ;
- le cas échéant, Autres prestations : [détail des prestations et conditions de rémunération].

2. Honoraires à la charge du locataire :

- prestations de visite du preneur, de constitution de son dossier et de rédaction de bail : [détail des prestations effectivement réalisées et montant des honoraires toutes taxes comprises dus à la signature du bail] ;
- le cas échéant, Prestation de réalisation de l'état des lieux d'entrée : [montant des honoraires toutes taxes comprises dus à compter de la réalisation de la prestation].

X. Autres conditions particulières

[A définir par les parties]

XI. Annexes

Sont annexées et jointes au contrat de location les pièces suivantes :

A. Le cas échéant, un extrait du règlement concernant la destination de l'immeuble, la jouissance et l'usage des parties privatives et communes, et précisant la quote-part afférente au lot loué dans chacune des catégories de charges

B. Un dossier de diagnostic technique comprenant

- un diagnostic de performance énergétique ;
- un constat de risque d'exposition au plomb pour les immeubles construits avant le 1er janvier 1949 ;
- le cas échéant, une copie d'un état mentionnant l'absence ou la présence de matériaux ou de produits de la construction contenant de l'amiante (39) ;
- le cas échéant, Un état de l'installation intérieure d'électricité et de gaz, dont l'objet est d'évaluer les risques pouvant porter atteinte à la sécurité des personnes (40) ;
- le cas échéant, un état des risques naturels et technologiques pour le zones couvertes par un plan de prévention des risques technologiques ou par un plan de prévention des risques naturels prévisibles, prescrit ou approuvé, ou dans des zones de sismicité (41).

C. Une notice d'information relative aux droits et obligations des locataires et des bailleurs

D. Un état des lieux, un inventaire et un état détaillé du mobilier (42)

E. Le cas échéant, une autorisation préalable de mise en location (43)

F. Le cas échéant, Les références aux loyers habituellement constatés dans le voisinage pour des logements comparables (44)

Le [date], à [lieu],
Signature du bailleur [ou de son mandataire, le cas échéant] Signature du locataire

(24) Préciser si la personne morale est une société civile constituée exclusivement entre parents et alliés jusqu'au quatrième degré inclus. (25) A reproduire si pluralité de bailleur. (26) Mention obligatoire s'appliquant aux professionnels exerçant une activité mentionnée à l'article 1er de la loi n° 70-9 du 2 janvier 1970 réglementant les conditions d'exercice des activités relatives à certaines opérations portant sur les immeubles et les fonds de commerce. (27) Si chauffage collectif, préciser les modalités de répartition de la consommation du locataire. (28) En cas de production collective, préciser les modalités de répartition de la consommation du locataire. (29) Lorsqu'un complément de loyer est appliqué, le loyer mensuel s'entend comme la somme du loyer de base et de ce complément. (30) Zones d'urbanisation continue de plus de 50 000 habitants où il existe un déséquilibre marqué entre l'offre et la demande de logements, entraînant des difficultés sérieuses d'accès au logement sur l'ensemble du parc résidentiel telles que définies par décret. (31) Mention obligatoire si le précédent locataire a quitté le logement moins de dix-huit mois avant la signature du bail. (32) Si les parties conviennent d'un forfait de charges et de sa révision annuelle, ce forfait est révisé dans les mêmes conditions que le loyer principal. (33) Au cours de l'exécution du contrat de location et dans les conditions prévues par la loi, les colocataires peuvent provoquer la résiliation de l'assurance souscrite par le bailleur pour leur compte. (34) Correspond au montant de la prime d'assurance annuelle, éventuellement majoré dans la limite d'un montant fixé par décret en Conseil d'Etat. (35) Paiement mensuel de droit à tout moment à la demande du locataire. (36) Le cas échéant, préciser par ailleurs le montant des travaux d'amélioration effectués au cours des six dernier mois. (37) Clause invalide pour les travaux de mise en conformité aux caractéristiques de décence. (38) A mentionner lorsque le contrat de location est conclu avec le concours d'une personne mandatée et rémunérée à cette fin. (39) A compter de l'entrée en vigueur du décret d'application lisant notamment les matériaux ou produits concernés. (40) A compter de la date d'entrée en vigueur de cette disposition, prévue par décret. (41) La liste des communes comprises dans ces zones est définie localement par arrêté préfectoral. (42) Ces documents sont établis lors de la remise des clés, dont la date peut être ultérieure à celle de conclusion du contrat. (43) Dispositif applicable dans certains territoires présentant une proportion importante d'habitat dégradé délimité localement par l'établissement public de coopération intercommunale compétent en matière d'habitat ou, à défaut, le conseil municipal (art. 92 de la loi n° 2014-366 du 24 mars 2014 pour l'accès au logement et un urbanisme rénové). (44) Lorsque la détermination du montant du loyer est la conséquence d'une procédure liée au fait que le loyer précédemment appliqué était manifestement sous-évalué.

NOTA :
Conformément à l’article 7 du décret n° 2023-796 du 18 août 2023, ces dispositions entrent en vigueur le 1er janvier 2025.