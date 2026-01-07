mod config;
mod middleware;
mod models;
mod routes;

use crate::routes::users::delete::__path_delete_user;
use crate::routes::users::login::__path_login;
use crate::routes::users::register::__path_register_handler;
use crate::routes::users::update::__path_update_user;
use axum::{Router, routing::get};
use routes::protected::protected;
use routes::users::delete::delete_user;
use routes::users::login::{AppState, login};
use routes::users::register::register_handler;
use routes::users::update::update_user;
use std::error::Error;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    config::load_env();
    let pool = config::database::create_pool()
        .await
        .expect("Failed to connect to the database");
    let state = AppState { db: pool };

    let (api_router, mut api) = OpenApiRouter::new()
        .routes(routes!(login))
        .routes(routes!(register_handler))
        .routes(routes!(delete_user))
        .routes(routes!(update_user))
        .split_for_parts();

    api.components
        .get_or_insert_with(Default::default)
        .add_security_scheme(
            "jwt",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );

    let app = Router::new()
        .merge(api_router)
        .route("/protected", get(protected))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api))
        .with_state(state);

    let addr = config::get_server_addr();
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
