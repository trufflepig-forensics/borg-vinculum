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

use crate::config::Config;
use crate::handler::frontend::{create_drone, login, logout};
use crate::middleware::{handle_not_found, json_extractor_error, AuthenticationRequired};
use crate::swagger::{ApiDoc, FrontendDoc};

/// Start the server
pub async fn start_server(config: &Config, db: Database) -> Result<(), String> {
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
            "ssh -i {}",
            shlex::quote(&config.borg.ssh_key_path)
        )),
    };

    let s_addr = SocketAddr::new(config.server.listen_address, config.server.listen_port);
    info!("Starting to listen on {}", s_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(PayloadConfig::default())
            .app_data(JsonConfig::default().error_handler(json_extractor_error))
            .app_data(Data::new(db.clone()))
            .app_data(Data::new(common_options.clone()))
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
                    .service(create_drone),
            )
            .service(scope("/api/drone/v1"))
    })
    .bind((config.server.listen_address, config.server.listen_port))
    .map_err(|e| e.to_string())?
    .run()
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}
