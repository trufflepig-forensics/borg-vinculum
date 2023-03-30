use actix_web::post;
use actix_web::web::{Data, Json};
use borgbackup::common::{CommonOptions, ListOptions};
use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;
use rorm::{insert, query, Database, Model};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::handler::{ApiError, ApiResult};
use crate::models::{Drone, DroneInsert};

/// The request to create a new drone
#[derive(Deserialize, ToSchema)]
pub struct CreateDroneRequest {
    #[schema(example = "one_of_nine")]
    name: String,
    #[schema(example = "user@example.com:server/1_of_9")]
    repository: String,
    #[schema(example = "super_secure_passphrase")]
    passphrase: String,
}

/// The response of a request to create a drone
#[derive(Serialize, ToSchema)]
pub struct CreateDroneResponse {
    uuid: Uuid,
    #[schema(example = "bearer_token_be_here")]
    token: String,
}

/// Create a new drone
///
/// The `name` parameter must be unique for all drones.
///
/// A uuid for identification and a bearer token for use in borg drone is returned.
#[utoipa::path(
    tag = "Drone management",
    context_path = "/api/frontend/v1",
    responses(
        (status = 200, description = "Created new drone", body = CreateDroneResponse),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse)
    ),
    request_body = CreateDroneRequest,
    security(("session_cookie" = [])),
)]
#[post("/drones")]
pub async fn create_drone(
    req: Json<CreateDroneRequest>,
    db: Data<Database>,
    common_options: Data<CommonOptions>,
) -> ApiResult<Json<CreateDroneResponse>> {
    let mut tx = db.start_transaction().await?;

    if req.name.is_empty() {
        return Err(ApiError::InvalidName);
    }

    let drone_ct = query!(&mut tx, (Drone::F.uuid.count(),))
        .condition(Drone::F.name.equals(&req.name))
        .one()
        .await?
        .0;

    let repo_ct = query!(&mut tx, (Drone::F.repository.count(),))
        .condition(Drone::F.repository.equals(&req.repository))
        .one()
        .await?
        .0;

    if drone_ct != 0 {
        return Err(ApiError::NameAlreadyExists);
    }

    if repo_ct != 0 {
        return Err(ApiError::RepositoryAlreadyExists);
    }

    let token = Alphanumeric.sample_string(&mut thread_rng(), 255);

    let uuid = insert!(&mut tx, DroneInsert)
        .return_primary_key()
        .single(&DroneInsert {
            uuid: Uuid::new_v4(),
            name: req.name.clone(),
            token: token.clone(),
            repository: req.repository.clone(),
            passphrase: req.passphrase.clone(),
        })
        .await?;

    borgbackup::asynchronous::list(
        &ListOptions {
            repository: req.repository.clone(),
            passphrase: Some(req.passphrase.clone()),
        },
        common_options.get_ref(),
    )
    .await?;

    tx.commit().await?;

    Ok(Json(CreateDroneResponse { uuid, token }))
}
