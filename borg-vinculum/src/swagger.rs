use common::*;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::handler::{api, frontend, ApiErrorResponse, ApiStatusCode};

struct TokenSecurity;

impl Modify for TokenSecurity {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "token",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .description(Some(
                            "The token is retrieved while creating drones in the server.",
                        ))
                        .build(),
                ),
            )
        }
    }
}

/// Helper struct for the drone api openapi definitions.
#[derive(OpenApi)]
#[openapi(
    paths(api::stats, api::error),
    components(schemas(
        ApiErrorResponse,
        ApiStatusCode,
        StatReport,
        CreateStats,
        HookStats,
        ErrorReport,
        State
    )),
    modifiers(&TokenSecurity)
)]
pub struct ApiDoc;

struct CookieSecurity;

impl Modify for CookieSecurity {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "session_cookie",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("id"))),
            )
        }
    }
}

/// Helper struct for the frontend openapi definitions.
#[derive(OpenApi)]
#[openapi(
    paths(
        frontend::test,
        frontend::login,
        frontend::logout,
        frontend::create_drone,
        frontend::get_all_drones,
        frontend::get_drone,
        frontend::delete_drone,
        frontend::get_key,
    ),
    components(schemas(
        ApiErrorResponse,
        ApiStatusCode,
        frontend::LoginRequest,
        frontend::CreateDroneRequest,
        frontend::CreateDroneResponse,
        frontend::GetAllDronesResponse,
        frontend::GetDroneResponse,
        frontend::GetKeyResponse,
    )),
    modifiers(&CookieSecurity)
)]
pub struct FrontendDoc;
