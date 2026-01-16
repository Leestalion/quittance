use axum::{
    Router,
    routing::{post, get},
    extract::State,
    Json,
    http::{StatusCode, HeaderMap},
};
use serde::{Deserialize, Serialize};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};
use uuid::Uuid;
use crate::{
    db::Database,
    models::user::User,
    error::AppError,
};

// JWT secret key - in production, load from environment variable
const JWT_SECRET: &[u8] = b"your-secret-key-change-in-production";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,  // user id
    exp: usize,   // expiration time
}

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
    name: String,
    address: String,
    phone: Option<String>,
    birth_date: Option<String>,
    birth_place: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct AuthResponse {
    token: String,
    user: User,
}

pub fn router() -> Router<Database> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/me", get(get_current_user))
}

async fn register(
    State(db): State<Database>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    // Validate email format
    if !payload.email.contains('@') {
        return Err(AppError::Validation("Invalid email format".to_string()));
    }

    // Validate password length
    if payload.password.len() < 8 {
        return Err(AppError::Validation("Password must be at least 8 characters".to_string()));
    }

    // Check if user already exists
    let existing = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)",
        payload.email.to_lowercase()
    )
    .fetch_one(&db.pool)
    .await?;

    if existing.unwrap_or(false) {
        return Err(AppError::Validation("Email already registered".to_string()));
    }

    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| AppError::Internal)?
        .to_string();

    // Parse birth_date if provided
    let birth_date = if let Some(date_str) = payload.birth_date {
        Some(
            chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                .map_err(|_| AppError::Validation("Invalid birth_date format".to_string()))?
        )
    } else {
        None
    };

    // Create user
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (email, password_hash, name, address, phone, birth_date, birth_place)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, email, password_hash, name, address, phone, birth_date, birth_place, created_at, updated_at
        "#,
        payload.email.to_lowercase(),
        password_hash,
        payload.name,
        payload.address,
        payload.phone,
        birth_date,
        payload.birth_place
    )
    .fetch_one(&db.pool)
    .await?;

    // Generate JWT token
    let token = generate_token(&user.id)?;

    Ok((StatusCode::CREATED, Json(AuthResponse { token, user })))
}

async fn login(
    State(db): State<Database>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Find user by email
    let user = sqlx::query_as!(
        User,
        "SELECT id, email, password_hash, name, address, phone, birth_date, birth_place, created_at, updated_at
         FROM users
         WHERE email = $1",
        payload.email.to_lowercase()
    )
    .fetch_optional(&db.pool)
    .await?
    .ok_or_else(|| AppError::Auth("Invalid email or password".to_string()))?;

    // Verify password
    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| AppError::Internal)?;
    
    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Auth("Invalid email or password".to_string()))?;

    // Generate JWT token
    let token = generate_token(&user.id)?;

    Ok(Json(AuthResponse { token, user }))
}

async fn get_current_user(
    State(db): State<Database>,
    headers: HeaderMap,
) -> Result<Json<User>, AppError> {
    // Extract user_id from JWT token
    let user_id = extract_user_id_from_headers(&headers)?;

    // Fetch user
    let user = sqlx::query_as!(
        User,
        "SELECT id, email, password_hash, name, address, phone, birth_date, birth_place, created_at, updated_at
         FROM users
         WHERE id = $1",
        user_id
    )
    .fetch_optional(&db.pool)
    .await?
    .ok_or_else(|| AppError::Auth("User not found".to_string()))?;

    Ok(Json(user))
}

fn generate_token(user_id: &Uuid) -> Result<String, AppError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
    .map_err(|_| AppError::Internal)
}

pub fn extract_user_id_from_headers(headers: &HeaderMap) -> Result<Uuid, AppError> {
    let token = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::Auth("Missing or invalid authorization header".to_string()))?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )
    .map_err(|_| AppError::Auth("Invalid token".to_string()))?;

    Uuid::parse_str(&token_data.claims.sub)
        .map_err(|_| AppError::Auth("Invalid user ID in token".to_string()))
}
