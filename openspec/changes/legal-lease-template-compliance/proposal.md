## Why

Le flux de génération de bail meublé actuel n'assure pas une conformité complète au contrat type réglementaire ni à l'ensemble des obligations légales documentées dans `doc/bail-template.md` et `doc/legislation-bail.md`. Cette lacune expose les utilisateurs à un risque juridique réel (bail contestable, clauses invalides, mentions/annexes manquantes) et doit être corrigée avant toute mise en production orientée usage réel.

## What Changes

- Aligner le comportement métier `leases` sur la structure complète du contrat type meublé (Annexe 2 du décret n°2015-587 modifié), section par section.
- Renforcer les validations serveur et front-end pour toutes les contraintes critiques: surface habitable, dépôt de garantie, complément de loyer, honoraires, champs conditionnels, annexes obligatoires.
- Introduire des règles conditionnelles exhaustives (colocation, zone encadrée, durée étudiant 9 mois, DOM/TOM, diagnostics selon conditions).
- Imposer des clauses légales auto-générées non modifiables (notamment clause résolutoire obligatoire, texte article 5-I si mandataire professionnel).
- Garantir que les documents générés sont exploitables en conditions réelles (mentions légales complètes, logique de conformité, contrôles bloquants/avertissements selon loi).
- **BREAKING**: certains baux auparavant générables avec données incomplètes ou incohérentes seront désormais bloqués tant que les exigences légales ne sont pas satisfaites.

## Capabilities

### New Capabilities
- *(none)*

### Modified Capabilities
- `leases`: Mise en conformité légale complète de la création, validation, stockage et génération du bail meublé pour correspondre au contrat type et à la législation documentée.

## Impact

- Backend: `backend/src/routes/leases.rs`, `backend/src/models/lease.rs`, migrations potentielles pour couvrir tous les champs légaux requis et leurs contraintes.
- Frontend: `frontend/src/components/LeaseForm.vue`, `frontend/src/components/LeasePreview.vue`, vues de génération/impression de bail, stores et types liés.
- Types/Contrats: `frontend/src/types/index.ts`, schémas API de bail, sérialisation des champs conditionnels et annexes.
- Conformité documentaire: alignement explicite avec `doc/bail-template.md` et `doc/legislation-bail.md` comme sources de référence légale produit.
