## ADDED Requirements

### Requirement: System captures and renders textual descriptions of property composition
The system SHALL capture and render textual descriptions of property composition across five categories required by Décret 2015-587 Section II.

#### Scenario: Capture autres parties du logement
- **WHEN** user enters text in "Autres parties du logement" field (e.g., "Grenier accessible, terrasse 8m², balcon")
- **THEN** the system stores text in database column `autres_parties`
- **AND** the text persists when lease is saved and retrieved

#### Scenario: Capture éléments d'équipement
- **WHEN** user enters text in "Éléments d'équipement" field (e.g., "Cuisine équipée, salle de bain avec baignoire et douche")
- **THEN** the system stores text in database column `elements_equipement`
- **AND** the text persists when lease is saved and retrieved

#### Scenario: Capture locaux privatifs accessoires
- **WHEN** user enters text in "Locaux privatifs accessoires" field (e.g., "Cave 15m², parking fermé")
- **THEN** the system stores text in database column `privatifs_accessoires`
- **AND** the text persists when lease is saved and retrieved

#### Scenario: Capture parties et équipements communs
- **WHEN** user enters text in "Parties et équipements communs" field (e.g., "Ascenseur, local vélo, espaces verts entretenus")
- **THEN** the system stores text in database column `parties_communes`
- **AND** the text persists when lease is saved and retrieved

#### Scenario: Capture équipements d'accès technologique
- **WHEN** user enters text in "Équipements d'accès technologique" field (e.g., "Fibre optique, TNT, accès internet collectif")
- **THEN** the system stores text in database column `tech_equipements`
- **AND** the text persists when lease is saved and retrieved

### Requirement: Property descriptions render in lease contract
The system SHALL render all captured property description fields in the generated lease contract in Section II.

#### Scenario: Render autres parties in contract
- **WHEN** a lease with "Autres parties du logement" text is generated
- **THEN** the PDF contract shows the text in Section II-A (Consistance du logement)

#### Scenario: Render équipement in contract
- **WHEN** a lease with "Éléments d'équipement" text is generated
- **THEN** the PDF contract shows the text in Section II-A

#### Scenario: Render accessoires in contract
- **WHEN** a lease with "Locaux privatifs accessoires" text is generated
- **THEN** the PDF contract shows the text in Section II-C

#### Scenario: Render common areas in contract
- **WHEN** a lease with "Parties et équipements communs" text is generated
- **THEN** the PDF contract shows the text in Section II-D

#### Scenario: Render tech access in contract
- **WHEN** a lease with "Équipements d'accès technologique" text is generated
- **THEN** the PDF contract shows the text in Section II-E

### Requirement: Form provides clear placeholders and hints for property descriptions
The system form SHALL display helpful placeholder text and hints for each property description field to guide users on what content is expected.

#### Scenario: Placeholder text for autres parties
- **WHEN** user views the "Autres parties du logement" field
- **THEN** the field displays placeholder text: "Décrivez tout espace annexé: grenier, cave, terrasse, jardin, balcon, etc."

#### Scenario: Placeholder text for équipement
- **WHEN** user views the "Éléments d'équipement" field
- **THEN** the field displays placeholder text: "Décrivez les équipements présents: cuisine équipée, chauffage, type sanitaires, etc."
