# auth-and-access Specification

## Purpose
Define authentication and access-control behavior for user registration, login, and current-user identity retrieval.

## Requirements

### Requirement: User account registration
The system MUST allow a new user to create an account by providing required profile and credential data.

#### Scenario: Successful account creation
- **WHEN** a user submits a registration request with a valid email, a password of at least 8 characters, and required profile fields
- **THEN** the system creates a new user account and returns an authenticated session token with the created user profile

#### Scenario: Duplicate email registration attempt
- **WHEN** a user submits a registration request with an email that is already registered
- **THEN** the system rejects the request and reports that the email is already registered

#### Scenario: Invalid registration input
- **WHEN** a user submits a registration request with an invalid email format or a password shorter than 8 characters
- **THEN** the system rejects the request with a validation error

### Requirement: User login
The system MUST authenticate an existing user using email and password credentials.

#### Scenario: Successful login
- **WHEN** a user submits valid login credentials for an existing account
- **THEN** the system authenticates the user and returns an authenticated session token with the user profile

#### Scenario: Invalid login credentials
- **WHEN** a user submits an unknown email or incorrect password
- **THEN** the system rejects authentication and reports invalid email or password

### Requirement: Authenticated identity access
The system MUST restrict current-user identity retrieval to requests that include a valid bearer token.

#### Scenario: Authenticated current-user retrieval
- **WHEN** a request to retrieve the current user includes a valid bearer token for an existing user
- **THEN** the system returns the corresponding user profile

#### Scenario: Missing or invalid bearer token
- **WHEN** a request to retrieve the current user is sent without a valid bearer token
- **THEN** the system rejects the request as unauthorized
