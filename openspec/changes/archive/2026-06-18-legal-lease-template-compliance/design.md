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

- **Décision: Génération de PDF côté serveur (server-side)**
  - Rationale: Garantit que le PDF généré correspond exactement à ce qui a été validé et persisté ; élimine la dérive frontend ; permet le versioning des templates légaux (si la loi change en 2027, les anciens PDF conservent le texte légal de 2026) ; fournit une piste d'audit et une défense juridique.
  - Alternatives considered: client-side (jsPDF) rejeté car expose au drift de rendu et complique la gestion des versions légales.
  - Implications: le backend assemblerait le bail complet en HTML/template, le serveur générerait le PDF binaire, le frontend ne ferait que télécharger.

- **Décision: Architecture « Sections légales verrouillées + Clauses personnalisées modifiables »**
  - Rationale: Sections auto-générées obligatoires (I à VIII, XI partiellement) sont non-éditables et conformes à la loi ; seule la section X (Autres conditions particulières) est éditable, sous validation serveur (interdiction de termes bannies) ; cela augmente la conformité, ne la réduit pas.
  - Alternatives considered: tout éditable (risque d'invalider sans le savoir), tout verrouillé (perte d'flexibilité contrat).
  - Implications: le modèle Lease persiste les champs personnalisés séparément ; la génération du bail construit: {blocs légaux auto-générés} + {contenu custom validé}.

- **Décision: Filigrane « Projet/Non conforme » pour les PDFs non-conformes**
  - Rationale: Permet au locataire/bailleur de télécharger un brouillon pour révision/itération même non-conforme ; empêche l'émission accidentelle de contrats invalides ; aligne avec le champ `compliance_status` déjà dans le modèle Lease ; défendable juridiquement (l'utilisateur a explicitement téléchargé un brouillon).
  - Alternatives considered: bloquer complètement le téléchargement (frustrant pour la révision), aucun marquage (confusion légale).
  - Implications: le PDF inclut un filigrane diagonal visible si `compliance_status != 'compliant'`.

## Architectural Overview: Canonical Legal Contract Snapshot

```
UI Form (sectioned stepper)
    ↓
  Draft Payload
    ↓
  Backend Legal Validator (source of truth)
    ↓
  Lease persisted + compliance_status + canonical snapshot JSON
    ↓
  Server-side PDF renderer (from snapshot, not from live form)
    ↓
  HTML/PDF with locked legal sections + custom clauses + watermark if needed
    ↓
  Frontend downloads/previews
```

**Canonical Contract Snapshot** = JSON object containing:
- All nine legal sections (I through XI) with their fields fully resolved
- Auto-generated legal text blocks (resolutory clause, article 5-I if mandataire, etc.)
- Conditional sections computed (colocation clause, rent-control wording, student non-renewal rule, etc.)
- User-provided custom clauses (validated)
- Metadata: date of generation, legal template version, compliance status

This snapshot is generated **on every lease save** and stored ; it becomes the source of truth for PDF rendering. The PDF renderer never assembles text dynamically—it only formats the snapshot.

## Canonical Lease Snapshot DTO (JSON Schema)

The canonical snapshot is a versioned, immutable JSON document stored in the `leases` table (column `canonical_snapshot`). It contains all information required to render the PDF deterministically, independent of the live lease record.

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "lease_id": "550e8400-e29b-41d4-a716-446655440001",
  "legal_template_version": "2026-06-18",
  "generated_at": "2026-06-18T10:30:00Z",
  "language": "fr",
  
  "parties": {
    "landlord_full_name": "Jean Dupont",
    "landlord_address": "123 rue de Paris, 75001 Paris",
    "landlord_phone": "+33612345678",
    "lessee_full_name": "Marie Martin",
    "lessee_address": "456 avenue Lyon, 69000 Lyon",
    "lessee_email": "marie.martin@example.com",
    "lessee_birth_date": "1995-03-15",
    "lessee_birth_place": "Lyon"
  },
  
  "property": {
    "address": "456 avenue Lyon, 69000 Lyon",
    "property_type": "apartment",
    "habitable_surface": 45.5,
    "main_room_count": 2,
    "furnished": true,
    "heating_mode": "collective",
    "hot_water_mode": "collective"
  },
  
  "lease_terms": {
    "lease_kind": "standard",
    "start_date": "2026-07-01",
    "duration_months": 12,
    "end_date": "2027-06-30",
    "auto_renewal": true,
    "destination": "residence_principale",
    "is_colocation": false,
    "tenant_count": 1
  },
  
  "financial_terms": {
    "monthly_rent": "850.00",
    "charges_monthly": "50.00",
    "rent_payment_frequency": "monthly",
    "rent_payment_timing": "due_first_day",
    "deposit_amount": "1700.00",
    "rent_controlled": false,
    "reference_rent": null,
    "reference_rent_majorated": null,
    "rent_complement": null,
    "rent_complement_justification": null
  },
  
  "professional_mandate": {
    "applies": false,
    "agency_fee_tenant": null,
    "agency_fee_landlord": null
  },
  
  "diagnostics": {
    "dpe_class": "D",
    "dpe_effective_date": "2026-06-18",
    "is_dom_tom": false,
    "energy_cost_annual": "1200.00"
  },
  
  "previous_tenancy": {
    "applies": false,
    "previous_tenant_last_rent": null,
    "previous_tenant_departure_date": null
  },
  
  "lease_sections": {
    "section_i_parties": {
      "text": "SECTION I - DÉSIGNATION DES PARTIES...",
      "auto_generated": true
    },
    "section_ii_property": {
      "text": "SECTION II - OBJET DU CONTRAT...",
      "auto_generated": true
    },
    "section_iii_duration": {
      "text": "SECTION III - DURÉE DU BAIL ET RENOUVELLEMENT...",
      "auto_generated": true,
      "computed_for_lease_kind": "standard"
    },
    "section_iv_financial": {
      "text": "SECTION IV - CONDITIONS FINANCIÈRES...",
      "auto_generated": true
    },
    "section_v_works": {
      "text": "SECTION V - TRAVAUX...",
      "auto_generated": true
    },
    "section_vi_guarantees": {
      "text": "SECTION VI - GARANTIES...",
      "auto_generated": true
    },
    "section_vii_solidarity": {
      "text": null,
      "auto_generated": false,
      "conditional": true,
      "applies_when": "is_colocation"
    },
    "section_viii_resolutory": {
      "text": "SECTION VIII - CLAUSE RÉSOLUTOIRE...",
      "auto_generated": true,
      "locked": true
    },
    "section_x_custom": {
      "text": "SECTION X - AUTRES CONDITIONS PARTICULIÈRES...",
      "auto_generated": false,
      "validated": true,
      "prohibited_patterns_rejected": [
        "prélèvement automatique comme seul mode",
        "interdiction d'héberger"
      ]
    },
    "section_xi_annexes": {
      "text": "SECTION XI - ANNEXES ET DOCUMENTS...",
      "auto_generated": true,
      "annex_legal_notice_provided": true,
      "annex_dpe_provided": true,
      "annex_entry_inventory_provided": true,
      "annex_furniture_inventory_provided": true
    }
  },
  
  "compliance": {
    "compliance_status": "compliant",
    "compliance_errors": [],
    "lease_valid_for_issuance": true
  }
}
```

**Key Design Decisions:**
- `legal_template_version`: Pins the legal text version used; if law changes, new leases use new version, old leases can be re-rendered using original text
- `auto_generated: true` blocks user modification in PDF; `auto_generated: false` allows editing
- `locked: true` on resolutory clause ensures it cannot be removed or altered
- `validated: true` on custom clauses indicates server-side prohibition checks have passed
- All financial fields are strings (will be serialized from `BigDecimal` in Rust)
- All dates are ISO 8601 strings

## Template Directory Structure and Versioning

Templates are stored in version-controlled folders within the repository, loaded at server startup, and cached in memory. This approach enables:
- Easy git-based revision tracking of legal text
- Scheduled deployment of law changes (e.g., 2027 DPE threshold)
- Deterministic PDF regeneration of old leases

```
backend/src/legal_templates/
├── manifest.json
│   {
│     "current_version": "2026-06-18",
│     "versions": [
│       {
│         "version": "2026-06-18",
│         "effective_date": "2026-06-18",
│         "language": "fr",
│         "legislative_refs": [
│           "Décret n°2015-587 modifié",
│           "Loi du 6 juillet 1989"
│         ],
│         "template_files": [
│           "section_i_parties.html",
│           "section_ii_property.html",
│           "section_iii_duration.html",
│           "section_iv_financial.html",
│           "section_v_works.html",
│           "section_vi_guarantees.html",
│           "section_vii_solidarity.html",
│           "section_viii_resolutory.html",
│           "section_x_custom.html",
│           "section_xi_annexes.html"
│         ]
│       },
│       {
│         "version": "2025-06-01",
│         "effective_date": "2025-06-01",
│         ...
│       }
│     ]
│   }
│
├── 2026-06-18/
│   ├── section_i_parties.html
│   │   Contains: {{landlord_full_name}}, {{landlord_address}}, {{lessee_full_name}}, {{lessee_birth_date}}, {{lessee_birth_place}}
│   ├── section_ii_property.html
│   │   Contains: {{property_address}}, {{property_type}}, {{habitable_surface}}, {{main_room_count}}, {{furnished}}
│   ├── section_iii_duration.html
│   │   Contains: {{start_date}}, {{duration_months}}, {{end_date}}, {{auto_renewal}}, {{lease_kind}}
│   ├── section_iv_financial.html
│   │   Contains: {{monthly_rent}}, {{charges_monthly}}, {{deposit_amount}}, {{rent_controlled}}, {{reference_rent}}
│   ├── section_v_works.html
│   │   Contains: {{damages_description}} (if applicable)
│   ├── section_vi_guarantees.html
│   │   Contains: {{deposit_amount}}, {{dpe_class}}, {{entry_inventory_date}}
│   ├── section_vii_solidarity.html
│   │   Conditional; only rendered if {{is_colocation}} = true
│   ├── section_viii_resolutory.html
│   │   Standard resolutory clause (non-editable, locked legal text)
│   ├── section_x_custom.html
│   │   Contains: {{custom_clauses}} (user-provided, validated)
│   ├── section_xi_annexes.html
│   │   Checklist of: legal notice, DPE, entry inventory, furniture inventory
│   └── layout.html
│       Master HTML wrapper:
│       <!DOCTYPE html>
│       <html>
│       <head>
│         <style>/* French legal document formatting */</style>
│       </head>
│       <body>
│         {{section_i_parties}}
│         {{section_ii_property}}
│         ...
│         {{section_xi_annexes}}
│       </body>
│       </html>
│
└── 2025-06-01/
    └── (previous version templates, if law changed)
    
```

**Template Loading Strategy (Rust code outline):**
```rust
pub struct TemplateCache {
    templates: HashMap<String, HashMap<String, String>>,  // version → (section_name → html)
}

impl TemplateCache {
    pub fn new(template_dir: &Path) -> Result<Self> {
        let manifest = load_manifest(&template_dir.join("manifest.json"))?;
        let mut templates = HashMap::new();
        
        for version in &manifest.versions {
            let mut sections = HashMap::new();
            for file in &version.template_files {
                let path = template_dir.join(&version.version).join(file);
                let content = std::fs::read_to_string(path)?;
                sections.insert(file.replace(".html", ""), content);
            }
            templates.insert(version.version.clone(), sections);
        }
        
        Ok(TemplateCache { templates })
    }
    
    pub fn render_section(&self, version: &str, section: &str, context: &serde_json::Value) -> Result<String> {
        let template = self.templates.get(version)?.get(section)?;
        // Use liquid or tera to render {{placeholders}} from context
        liquid::ParserBuilder::default().build()?.parse(&template)?.render(&context)
    }
}
```

## API Contract for PDF Generation

### Endpoint 1: Generate/Fetch Lease PDF

**Request:**
```
GET /api/leases/{lease_id}/pdf
Headers:
  Authorization: Bearer {jwt_token}
Query Parameters:
  ?format=pdf           (default; could support ?format=html for preview)
  ?watermark=true       (if compliance_status != 'compliant', always true; can force watermark)
```

**Response (200 OK):**
```
Content-Type: application/pdf
Content-Disposition: attachment; filename="bail_{lease_id}_{date}.pdf"
[Binary PDF data]
```

**Response (400 Bad Request):**
```json
{
  "error": "lease_not_found",
  "message": "Lease 550e8400-e29b-41d4-a716-446655440001 not found in your organization"
}
```

**Response (422 Unprocessable Entity):**
```json
{
  "error": "pdf_generation_failed",
  "message": "Failed to render PDF: wkhtmltopdf process timed out after 30s",
  "details": "Check that HTML template is valid and snapshot data is complete"
}
```

### Endpoint 2: Fetch Canonical Snapshot (for debugging/preview)

**Request:**
```
GET /api/leases/{lease_id}/snapshot
Headers:
  Authorization: Bearer {jwt_token}
```

**Response (200 OK):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "lease_id": "550e8400-e29b-41d4-a716-446655440001",
  "legal_template_version": "2026-06-18",
  ... (full snapshot as documented above)
}
```

**Response (400 Bad Request):**
```json
{
  "error": "lease_not_found",
  "message": "Lease 550e8400-e29b-41d4-a716-446655440001 not found"
}
```

### Error Handling and Timeouts

- **wkhtmltopdf process timeout:** 30 seconds (configurable via env `PDF_GENERATION_TIMEOUT_SECS`)
- **Retry logic:** None (user can retry GET request)
- **Logging:** All PDF generation attempts logged with lease_id, template_version, success/failure, duration
- **Monitoring:** Counter metrics for PDF generation success/failure rates

## Risks / Trade-offs

- [Complexité de formulaire accrue] → Mitigation: sections progressives, champs conditionnels dynamiques, aide contextuelle explicite.
- [Faux positifs de validation selon contexte local] → Mitigation: règles paramétrables (zone, date, DOM/TOM) et messages explicites.
- [Évolution législative future] → Mitigation: centraliser les seuils/règles légales et versionner les politiques de validation.
- [Risque de rupture API/UX] → Mitigation: marquer les validations renforcées comme breaking change, fournir plan de migration des données.
- [Complexité serveur PDF] → Mitigation: utiliser une bibliothèque éprouvée (e.g., wkhtmltopdf, Puppeteer, ou Rust crate comme `printpdf`) ; tester rendu sur navigateurs multiples avant serveur.
- [Déploiement progressif] → Mitigation: feature flag pour basculer client-side ↔ server-side PDF pendant transition.

## Migration Plan

1. Étendre le modèle `leases` et les DTO/API pour les champs obligatoires/conditionnels manquants.
2. Implémenter validations serveur (source de vérité) puis validations front équivalentes.
3. Mettre à jour génération documentaire pour sections obligatoires, clauses auto-générées et annexes.
4. Ajouter migration/base de données pour nouvelles colonnes contraintes.
5. Introduire jeux de tests de conformité (succès/échec) alignés aux scenarios spec.
6. Déployer avec feature flag si nécessaire pour absorber les dossiers existants incomplets.
7. Prévoir rollback via désactivation flag et maintien compatibilité lecture des anciens enregistrements.

## Open Questions Resolved

✅ **Q: Génération PDF côté client ou serveur?** → **Serveur** (voir decision ci-dessus)
✅ **Q: Texte légal éditable ou verrouillé?** → **Verrouillé + Custom modifiable** (voir decision ci-dessus)
✅ **Q: Bloquer les PDFs non-conformes?** → **Non, filigrane brouillon** (voir decision ci-dessus)

## Remaining Open Questions

- Quelle granularité retenir pour persister les annexes (booléens, métadonnées détaillées, pièces jointes)?
- Le texte exact de la clause résolutoire et de l'article 5-I doit-il être figé en français legal copy dans le backend ou dans un template versionné?
- Faut-il implémenter une géolocalisation automatique de zone encadrée ou une table de référence administrable?
- Quelle bibliothèque choisir pour server-side PDF (wkhtmltopdf, Puppeteer Rust binding, `printpdf` crate)?