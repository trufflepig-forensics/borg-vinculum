use actix_web::get;
use actix_web::web::{Data, Json};
use log::error;
use serde::Serialize;
use utoipa::ToSchema;

use crate::config::Config;
use crate::handler::{ApiError, ApiResult};

/// The response to a get key request
#[derive(Serialize, ToSchema)]
pub struct GetKeyResponse {
    public_key: String,
}

/// Request the public key of the server
#[utoipa::path(
    tag = "Key",
    context_path = "/api/frontend/v1",
    responses(
        (status = 200, description = "Created new drone", body = GetKeyResponse),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse)
    ),
    security(("session_cookie" = [])),
)]
#[get("/key")]
pub async fn get_key(config: Data<Config>) -> ApiResult<Json<GetKeyResponse>> {
    Ok(Json(GetKeyResponse {
        public_key: config
            .private_key
            .as_ref()
            .ok_or(ApiError::InternalServerError)?
            .public_key()
            .to_openssh()
            .map_err(|e| {
                error!("Could not encode public key to an openssh encoded key: {e}");
                ApiError::InternalServerError
            })?,
    }))
}
