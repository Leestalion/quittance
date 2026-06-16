# Receipts

## Purpose
Define receipt lifecycle behavior, validation rules, regeneration logic, and payload fields for listing, creating, reading, deleting, and regenerating rent receipts.

## Requirements

### Requirement: Receipt listing supports global and lease-scoped views
The system MUST support listing receipts globally and for a specific lease, ordered from newest period to oldest.

#### Scenario: List receipts for a lease
- **WHEN** a receipts list request includes a lease identifier
- **THEN** the system returns receipts for that lease ordered by period_year descending and period_month descending

#### Scenario: List all receipts
- **WHEN** a receipts list request does not include a lease identifier
- **THEN** the system returns all receipts ordered by period_year descending and period_month descending

### Requirement: Receipt creation validates period and amounts and upserts by period
The system MUST validate receipt input and create or update one receipt per lease-period combination.

#### Scenario: Reject invalid receipt month
- **WHEN** a receipt create request uses a period month outside 1 to 12
- **THEN** the system rejects the request

#### Scenario: Reject negative amounts
- **WHEN** a receipt create request provides a negative base rent or negative charges
- **THEN** the system rejects the request

#### Scenario: Reject period outside lease dates
- **WHEN** a receipt create request targets a month that does not overlap the lease start/end period
- **THEN** the system rejects the request

#### Scenario: Upsert existing lease-period receipt
- **WHEN** a receipt create request targets a lease and period that already exists
- **THEN** the system updates the existing receipt values and keeps one unique receipt for that lease-period

#### Scenario: Create new receipt with generated status
- **WHEN** a valid receipt create request targets a new lease-period
- **THEN** the system creates a new receipt with status generated

### Requirement: Receipt read and delete support individual record lifecycle
The system MUST allow reading and deleting a receipt by identifier.

#### Scenario: Read existing receipt
- **WHEN** a request retrieves a receipt by identifier and the receipt exists
- **THEN** the system returns that receipt

#### Scenario: Delete existing receipt
- **WHEN** a request deletes a receipt by identifier and the receipt exists
- **THEN** the system deletes the receipt and returns a no-content success response

#### Scenario: Delete unknown receipt
- **WHEN** a request deletes a receipt by identifier that does not exist
- **THEN** the system rejects the request as not found

### Requirement: Receipt regeneration creates missing historical receipts up to previous month
The system MUST regenerate receipts for a lease month by month from lease start up to the previous month cutoff, using prorated amounts when the lease covers part of a month.

#### Scenario: Regenerate missing receipts without purge
- **WHEN** a regenerate request is sent with purge_existing set to false
- **THEN** the system creates only missing receipts up to the previous month cutoff and keeps existing receipts unchanged

#### Scenario: Regenerate receipts with purge
- **WHEN** a regenerate request is sent with purge_existing set to true
- **THEN** the system deletes existing receipts up to the previous month cutoff and recreates receipts month by month

#### Scenario: Prorated amount for partial month coverage
- **WHEN** a month in regeneration is only partially covered by the lease start/end dates
- **THEN** the system computes base_rent and charges prorated to covered days in that month

### Requirement: Receipt payload includes persisted status and delivery metadata fields
The system MUST persist and return current receipt status and delivery metadata fields.

#### Scenario: Receipt response includes status and email metadata
- **WHEN** a receipt is returned by create, list, read, or regenerate operations
- **THEN** the response includes status and email_sent_at fields together with core rent and period fields
