## Context

Le comportement `leases` existant couvre le cycle de vie de base (CRUD, accès, calcul de date de fin, associations mobilier) mais ne formalise pas l'ensemble des obligations légales d'un bail meublé en résidence principale. Les sources de référence projet (`doc/bail-template.md` et `doc/legislation-bail.md`) exigent une conformité structurelle complète (sections contractuelles), des validations conditionnelles strictes et des clauses/annexes obligatoires.

Contrainte produit: le bail généré doit être utilisable en conditions réelles, donc juridiquement cohérent, complet et défensif face aux cas d'invalidité documentaire.

## Goals / Non-Goals

**Goals:**
- Définir des exigences de conformité complètes pour le capability `leases` alignées sur le contrat type meublé.
- Encadrer les règles conditionnelles critiques: colocation, zone encadrée, étudiant, diagnostics, DOM/TOM.
- Imposer des validations bloquantes et des clauses auto-générées non modifiables lorsque la loi l'exige.
- Spécifier les contraintes de données minimales pour un bail réellement exploitable (mentions, annexes, contrôles).

**Non-Goals:**
- Fournir un avis juridique individualisé ou remplacer la revue d'un professionnel du droit.
- Couvrir le bail mobilité (hors périmètre documenté).
- Modifier dans ce changement le capability `receipts` ou les règles de quittance.

## Decisions

- Décision: Modifier le capability existant `leases` au lieu d'en créer un nouveau.
  - Rationale: La conformité concerne directement le comportement existant de création/édition/génération de bail.
  - Alternatives considered: capability séparé `lease-legal-compliance` rejeté car fragmenterait artificiellement le domaine.

- Décision: Exprimer la conformité via plusieurs exigences testables (structure du bail, validations, règles conditionnelles, clauses obligatoires, annexes).
  - Rationale: Permet des tests d'acceptation ciblés et traçables par article/règle.
  - Alternatives considered: une exigence unique monolithique rejetée car difficile à vérifier.

- Décision: Rendre bloquantes les violations légales critiques et non bloquants les avertissements d'information selon cas.
  - Rationale: Équilibre entre conformité stricte et expérience utilisateur guidée.
  - Alternatives considered: tout bloquer (trop rigide), tout avertir (insuffisant juridiquement).

- Décision: Conserver la clause résolutoire en contenu auto-généré non éditable.
  - Rationale: Obligation légale explicite et réduction du risque de suppression involontaire.
  - Alternatives considered: clause éditable rejetée pour risque de non-conformité.

## Risks / Trade-offs

- [Complexité de formulaire accrue] → Mitigation: sections progressives, champs conditionnels dynamiques, aide contextuelle explicite.
- [Faux positifs de validation selon contexte local] → Mitigation: règles paramétrables (zone, date, DOM/TOM) et messages explicites.
- [Évolution législative future] → Mitigation: centraliser les seuils/règles légales et versionner les politiques de validation.
- [Risque de rupture API/UX] → Mitigation: marquer les validations renforcées comme breaking change, fournir plan de migration des données.

## Migration Plan

1. Étendre le modèle `leases` et les DTO/API pour les champs obligatoires/conditionnels manquants.
2. Implémenter validations serveur (source de vérité) puis validations front équivalentes.
3. Mettre à jour génération documentaire pour sections obligatoires, clauses auto-générées et annexes.
4. Ajouter migration/base de données pour nouvelles colonnes contraintes.
5. Introduire jeux de tests de conformité (succès/échec) alignés aux scenarios spec.
6. Déployer avec feature flag si nécessaire pour absorber les dossiers existants incomplets.
7. Prévoir rollback via désactivation flag et maintien compatibilité lecture des anciens enregistrements.

## Open Questions

- Quelle granularité retenir pour persister les annexes (booléens, métadonnées détaillées, pièces jointes)?
- Le texte exact de la clause résolutoire et de l'article 5-I doit-il être figé en français legal copy dans le backend ou dans un template versionné?
- Faut-il implémenter une géolocalisation automatique de zone encadrée ou une table de référence administrable?
- Quel niveau de blocage UX pour DPE non conforme (blocage strict vs blocage avec dérogation tracée)?