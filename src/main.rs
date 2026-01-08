mod config;
mod handlers;
mod middleware;
mod models;
mod routes;

use crate::routes::{
    drones::list_drones::__path_list_drones, drones::register::__path_register_drone,
    stats::stats_::__path_get_stats, users::delete::__path_delete_user, users::login::__path_login,
    users::register::__path_register_handler, users::update::__path_update_user,
};
use axum::Router;
use routes::{
    drones::list_drones::list_drones,
    drones::register::register_drone,
    stats::stats_::get_stats,
    users::delete::delete_user,
    users::login::{AppState, login},
    users::register::register_handler,
    users::update::update_user,
};
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
        .routes(routes!(get_stats))
        .routes(routes!(register_drone))
        .routes(routes!(list_drones))
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
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api))
        .with_state(state);

    let addr = config::get_server_addr();
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
