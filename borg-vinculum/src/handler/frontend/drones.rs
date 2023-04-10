use actix_web::web::{Data, Json, Path};
use actix_web::{delete, get, post, HttpResponse};
use borgbackup::common::{CommonOptions, ListOptions};
use chrono::{DateTime, Utc};
use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;
use rorm::{insert, query, Database, Model};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::handler::{ApiError, ApiResult, PathUuid};
use crate::models::{Drone, DroneInsert, DroneStats};

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

/// The representation of a single drone.
///
/// The parameter `token` is used as bearer token to authenticate the drone to the vinculum.
#[derive(Serialize, ToSchema)]
pub struct GetDroneResponse {
    uuid: Uuid,
    #[schema(example = "one_of_nine")]
    name: String,
    active: bool,
    #[schema(example = "bearer_token_will_be_here")]
    token: String,
    #[schema(example = "user@example.com:server/one_of_nine")]
    repository: String,
    created_at: DateTime<Utc>,
    last_activity: Option<DateTime<Utc>>,
}

/// All available drones in the vinculum
#[derive(Serialize, ToSchema)]
pub struct GetAllDronesResponse {
    drones: Vec<GetDroneResponse>,
}

/// Retrieve all drones from the vinculum
#[utoipa::path(
    tag = "Drone management",
    context_path = "/api/frontend/v1",
    responses(
        (status = 200, description = "Retrieve all drones", body = GetAllDronesResponse),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse)
    ),
    security(("session_cookie" = [])),
)]
#[get("/drones")]
pub async fn get_all_drones(db: Data<Database>) -> ApiResult<Json<GetAllDronesResponse>> {
    let drones = query!(db.as_ref(), Drone).all().await?;

    Ok(Json(GetAllDronesResponse {
        drones: drones
            .into_iter()
            .map(|x| GetDroneResponse {
                uuid: x.uuid,
                name: x.name,
                repository: x.repository,
                token: x.token,
                active: x.active,
                created_at: DateTime::from_local(x.created_at, Utc),
                last_activity: x.last_activity.map(|x| DateTime::from_local(x, Utc)),
            })
            .collect(),
    }))
}

/// Retrieve a drone by its uuid
///
/// The parameter `token` is used as bearer token to authenticate the drone to the vinculum.
#[utoipa::path(
    tag = "Drone management",
    context_path = "/api/frontend/v1",
    responses(
        (status = 200, description = "Retrieve the selected drone", body = GetDroneResponse),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse)
    ),
    params(PathUuid),
    security(("session_cookie" = [])),
)]
#[get("/drones/{uuid}")]
pub async fn get_drone(
    path: Path<PathUuid>,
    db: Data<Database>,
) -> ApiResult<Json<GetDroneResponse>> {
    let drone = query!(db.as_ref(), Drone)
        .condition(Drone::F.uuid.equals(path.uuid.as_ref()))
        .optional()
        .await?
        .ok_or(ApiError::InvalidUuid)?;

    Ok(Json(GetDroneResponse {
        uuid: drone.uuid,
        name: drone.name,
        repository: drone.repository,
        token: drone.token,
        active: drone.active,
        created_at: DateTime::from_local(drone.created_at, Utc),
        last_activity: drone.last_activity.map(|x| DateTime::from_local(x, Utc)),
    }))
}

/// A single stat record of a drone
#[derive(Serialize, ToSchema)]
pub struct DroneStat {
    pre_hook_duration: Option<i64>,
    post_hook_duration: Option<i64>,
    create_duration: i64,
    complete_duration: i64,
    original_size: i64,
    compressed_size: i64,
    deduplicated_size: i64,
    nfiles: i64,
    created_at: DateTime<Utc>,
}

/// The stats of a drone
#[derive(Serialize, ToSchema)]
pub struct GetDroneStats {
    stats: Vec<DroneStat>,
}

#[utoipa::path(
    tag = "Drone management",
    context_path = "/api/frontend/v1",
    responses(
        (status = 200, description = "Retrieved the stats of the drone", body = GetDroneStats),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse)
    ),
    params(PathUuid),
    security(("session_cookie" = [])),
)]
#[get("/drones/{uuid}/stats")]
pub async fn get_drone_stats(
    path: Path<PathUuid>,
    db: Data<Database>,
) -> ApiResult<Json<GetDroneStats>> {
    let mut tx = db.start_transaction().await?;

    let mut drone = query!(&mut tx, Drone)
        .condition(Drone::F.uuid.equals(path.uuid.as_ref()))
        .optional()
        .await?
        .ok_or(ApiError::InvalidUuid)?;

    Drone::F.stats.populate(&mut tx, &mut drone).await?;

    tx.commit().await?;

    Ok(Json(GetDroneStats {
        stats: drone
            .stats
            .cached
            .unwrap()
            .into_iter()
            .map(|x: DroneStats| DroneStat {
                pre_hook_duration: x.pre_hook_duration,
                post_hook_duration: x.post_hook_duration,
                create_duration: x.create_duration,
                complete_duration: x.complete_duration,
                nfiles: x.nfiles,
                original_size: x.original_size,
                compressed_size: x.compressed_size,
                deduplicated_size: x.deduplicated_size,
                created_at: DateTime::from_utc(x.created_at, Utc),
            })
            .collect(),
    }))
}

/// Delete a drone by its uuid
#[utoipa::path(
    tag = "Drone management",
    context_path = "/api/frontend/v1",
    responses(
        (status = 200, description = "Drone got deleted"),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse)
    ),
    params(PathUuid),
    security(("session_cookie" = [])),
)]
#[delete("/drones/{uuid}")]
pub async fn delete_drone(path: Path<PathUuid>, db: Data<Database>) -> ApiResult<HttpResponse> {
    let mut tx = db.start_transaction().await?;

    let ct = query!(&mut tx, (Drone::F.uuid.count(),))
        .condition(Drone::F.uuid.equals(path.uuid.as_ref()))
        .one()
        .await?
        .0;

    if ct == 0 {
        return Err(ApiError::InvalidUuid);
    }

    rorm::delete!(&mut tx, Drone)
        .condition(Drone::F.uuid.equals(path.uuid.as_ref()))
        .await?;

    tx.commit().await?;

    Ok(HttpResponse::Ok().finish())
}
