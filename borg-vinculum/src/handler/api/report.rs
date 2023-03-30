use actix_web::web::{Data, Json};
use actix_web::{post, HttpRequest, HttpResponse};
use common::StatReport;
use log::debug;
use rorm::fields::ForeignModelByField;
use rorm::{insert, query, Database, Model};
use uuid::Uuid;

use crate::handler::{ApiError, ApiResult};
use crate::models::{Drone, DroneStatsInsert};

#[utoipa::path(
    context_path = "/api/drone/v1",
    responses(
        (status = 200, description = "Stats reported"),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse)
    ),
    request_body = StatReport,
    security(("token" = [])),
)]
#[post("/stats")]
pub async fn stats(
    req: Json<StatReport>,
    raw_req: HttpRequest,
    db: Data<Database>,
) -> ApiResult<HttpResponse> {
    let mut tx = db.start_transaction().await?;

    // Retrieve drone and check for authentication
    let drone = if let Some(auth_header) = raw_req.headers().get("Authorization") {
        let auth_value = auth_header.to_str().map_err(|e| {
            debug!("Invalid characters in header: {e}");
            ApiError::Unauthenticated
        })?;

        let h: Vec<&str> = auth_value.split(' ').collect();
        if h.len() != 2 {
            return Err(ApiError::Unauthenticated);
        }

        if *h.first().unwrap() != "Bearer" {
            return Err(ApiError::Unauthenticated);
        }

        let token = *h.get(1).unwrap();

        let drone = query!(&mut tx, Drone)
            .condition(Drone::F.token.equals(token))
            .optional()
            .await?
            .ok_or(ApiError::Unauthenticated)?;

        drone
    } else {
        return Err(ApiError::Unauthenticated);
    };

    let mut complete_duration = req.create_stats.duration;
    if let Some(pre) = req.pre_hook_stats {
        complete_duration += pre.duration
    }
    if let Some(post) = req.post_hook_stats {
        complete_duration += post.duration
    }

    insert!(&mut tx, DroneStatsInsert)
        .return_nothing()
        .single(&DroneStatsInsert {
            uuid: Uuid::new_v4(),
            drone: ForeignModelByField::Key(drone.uuid),
            pre_hook_duration: req.pre_hook_stats.map(|x| x.duration as i64),
            post_hook_duration: req.pre_hook_stats.map(|x| x.duration as i64),
            create_duration: req.create_stats.duration as i64,
            complete_duration: complete_duration as i64,
            original_size: req.create_stats.original_size as i64,
            compressed_size: req.create_stats.compressed_size as i64,
            deduplicated_size: req.create_stats.deduplicated_size as i64,
            nfiles: req.create_stats.nfiles as i64,
        })
        .await?;

    tx.commit().await?;

    Ok(HttpResponse::Ok().finish())
}
