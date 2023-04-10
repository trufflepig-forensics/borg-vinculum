//! The server is defined in this module

use std::net::SocketAddr;

use actix_toolbox::tb_middleware::{
    setup_logging_mw, DBSessionStore, LoggingMiddlewareConfig, PersistentSession, SessionMiddleware,
};
use actix_web::cookie::time::Duration;
use actix_web::cookie::Key;
use actix_web::http::StatusCode;
use actix_web::middleware::{Compress, ErrorHandlers};
use actix_web::web::{scope, Data, JsonConfig, PayloadConfig};
use actix_web::{App, HttpServer};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use borgbackup::common::CommonOptions;
use log::info;
use rorm::Database;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

use crate::chan::MatrixNotifierChan;
use crate::config::Config;
use crate::handler::api::{error, stats};
use crate::handler::frontend::{
    create_drone, delete_drone, get_all_drones, get_drone, get_drone_stats, get_key, login, logout,
    test,
};
use crate::middleware::{handle_not_found, json_extractor_error, AuthenticationRequired};
use crate::swagger::{ApiDoc, FrontendDoc};

/// Start the server
pub async fn start_server(
    config: &Config,
    db: Database,
    matrix_notifier_chan: MatrixNotifierChan,
) -> Result<(), String> {
    let key = Key::try_from(
        BASE64_STANDARD
            .decode(&config.server.secret_key)
            .map_err(|e| {
                format!("Could not decode SecretKey: {e}. Generate one using the keygen subcommand")
            })?
            .as_slice(),
    )
    .map_err(|_| "Invalid SecretKey. Generate one using the keygen subcommand".to_string())?;

    let common_options = CommonOptions {
        local_path: Some(config.borg.borg_path.clone()),
        remote_path: config.borg.borg_remote_path.clone(),
        upload_ratelimit: None,
        rsh: Some(format!(
            "ssh -i {} -o 'StrictHostKeyChecking accept-new'",
            shlex::quote(&config.borg.ssh_key_path)
        )),
    };

    let s_addr = SocketAddr::new(config.server.listen_address, config.server.listen_port);
    info!("Starting to listen on {}", s_addr);

    let conf_data = Data::new(config.clone());
    HttpServer::new(move || {
        App::new()
            .app_data(PayloadConfig::default())
            .app_data(JsonConfig::default().error_handler(json_extractor_error))
            .app_data(Data::new(db.clone()))
            .app_data(Data::new(common_options.clone()))
            .app_data(Data::new(matrix_notifier_chan.clone()))
            .app_data(conf_data.clone())
            .wrap(setup_logging_mw(LoggingMiddlewareConfig::default()))
            .wrap(Compress::default())
            .wrap(
                SessionMiddleware::builder(DBSessionStore::new(db.clone()), key.clone())
                    .session_lifecycle(PersistentSession::session_ttl(
                        PersistentSession::default(),
                        Duration::hours(24),
                    ))
                    .build(),
            )
            .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, handle_not_found))
            .service(SwaggerUi::new("/docs/{_:.*}").urls(vec![
                (
                    Url::new("api-docs", "/api-doc/apidocs.json"),
                    ApiDoc::openapi(),
                ),
                (
                    Url::new("frontend-docs", "/api-doc/frontenddocs.json"),
                    FrontendDoc::openapi(),
                ),
            ]))
            .service(
                scope("/api/frontend/v1/auth")
                    .service(login)
                    .service(logout),
            )
            .service(
                scope("/api/frontend/v1")
                    .wrap(AuthenticationRequired)
                    .service(test)
                    .service(get_key)
                    .service(create_drone)
                    .service(get_all_drones)
                    .service(get_drone)
                    .service(delete_drone)
                    .service(get_drone_stats),
            )
            .service(scope("/api/drone/v1").service(stats).service(error))
    })
    .bind((config.server.listen_address, config.server.listen_port))
    .map_err(|e| e.to_string())?
    .run()
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}
