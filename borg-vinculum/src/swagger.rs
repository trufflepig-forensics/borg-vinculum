use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

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
                            "The token is set in the configuration file in the server.",
                        ))
                        .build(),
                ),
            )
        }
    }
}

/// Helper struct for the admin openapi definitions.
#[derive(OpenApi)]
#[openapi(modifiers(&TokenSecurity))]
pub struct ApiDoc;
