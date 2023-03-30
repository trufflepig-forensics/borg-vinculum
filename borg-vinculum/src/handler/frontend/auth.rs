use actix_toolbox::tb_middleware::Session;
use actix_web::web::{Data, Json};
use actix_web::{get, post, HttpResponse};
use argon2::password_hash::{Error, PasswordHash};
use argon2::{Argon2, PasswordVerifier};
use chrono::Utc;
use rorm::{query, update, Database, Model};
use serde::Deserialize;

use crate::handler::frontend::{ApiError, ApiResult};
use crate::models::Account;

/// The request to login
#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[post("/login")]
pub async fn login(
    req: Json<LoginRequest>,
    session: Session,
    db: Data<Database>,
) -> ApiResult<HttpResponse> {
    let mut tx = db.start_transaction().await?;

    let user = query!(&mut tx, Account)
        .condition(Account::F.username.equals(&req.username))
        .optional()
        .await?
        .ok_or(ApiError::LoginFailed)?;

    Argon2::default()
        .verify_password(
            req.password.as_bytes(),
            &PasswordHash::new(&user.password_hash)?,
        )
        .map_err(|e| match e {
            Error::Password => ApiError::LoginFailed,
            _ => ApiError::InvalidHash(e),
        })?;

    update!(&mut tx, Account)
        .condition(Account::F.uuid.equals(user.uuid.as_ref()))
        .set(Account::F.last_login_at, Some(Utc::now().naive_utc()))
        .exec()
        .await?;

    tx.commit().await?;

    session.insert("uuid", user.uuid)?;
    session.insert("logged_in", true)?;

    Ok(HttpResponse::Ok().finish())
}

/// Log out of this session
///
/// Logs a logged-in user out of his session.
#[utoipa::path(
    tag = "Authentication",
    context_path = "/api/v2/auth",
    responses(
        (status = 200, description = "Logout successful"),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse)
    ),
)]
#[get("/logout")]
pub(crate) async fn logout(session: Session) -> ApiResult<HttpResponse> {
    session.purge();

    Ok(HttpResponse::Ok().finish())
}
