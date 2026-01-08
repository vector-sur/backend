mod config;
mod handlers;
mod middleware;
mod models;
mod routes;

use crate::routes::{
    business::delete::__path_delete_business, business::list::__path_list_businesses,
    business::register::__path_register_business,
    business::set_location::__path_set_business_location, business::update::__path_update_business,
    drones::delete::__path_delete_drone, drones::list_drones::__path_list_drones,
    drones::register::__path_register_drone,
    product::delete::__path_delete_product,
    product::list_by_business::__path_list_products_by_business,
    product::register::__path_register_product, product::update::__path_update_product,
    stats::stats_::__path_get_stats, users::delete::__path_delete_user, users::login::__path_login,
    users::register::__path_register_handler, users::update::__path_update_user,
};
use axum::Router;
use routes::{
    business::delete::delete_business,
    business::list::list_businesses,
    business::register::register_business,
    business::set_location::set_business_location,
    business::update::update_business,
    drones::delete::delete_drone,
    drones::list_drones::list_drones,
    drones::register::register_drone,
    product::delete::delete_product,
    product::list_by_business::list_products_by_business,
    product::register::register_product,
    product::update::update_product,
    stats::stats_::get_stats,
    users::delete::delete_user,
    users::login::{AppState, login},
    users::register::register_handler,
    users::update::update_user,
};
use std::error::Error;
use utoipa::openapi::{
    security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    tag::TagBuilder,
};
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
        .routes(routes!(delete_drone))
        .routes(routes!(register_business))
        .routes(routes!(list_businesses))
        .routes(routes!(update_business))
        .routes(routes!(delete_business))
        .routes(routes!(set_business_location))
        .routes(routes!(register_product))
        .routes(routes!(update_product))
        .routes(routes!(delete_product))
        .routes(routes!(list_products_by_business))
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

    api.tags = Some(vec![
        TagBuilder::new()
            .name("Authentication")
            .description(Some("Authentication endpoints"))
            .build(),
        TagBuilder::new()
            .name("Users")
            .description(Some("User management endpoints"))
            .build(),
        TagBuilder::new()
            .name("Business")
            .description(Some("Business management endpoints"))
            .build(),
        TagBuilder::new()
            .name("Products")
            .description(Some("Product management endpoints"))
            .build(),
        TagBuilder::new()
            .name("Drones")
            .description(Some("Drone management endpoints"))
            .build(),
        TagBuilder::new()
            .name("Stats")
            .description(Some("Statistics endpoints"))
            .build(),
    ]);

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
