mod config;
mod handlers;
mod middleware;
mod models;
mod routes;

use std::error::Error;

use axum::Router;
use routes::{
    business::{
        delete::delete_business, list::list_businesses, register::register_business,
        set_location::set_business_location, update::update_business,
    },
    drones::{delete::delete_drone, list_drones::list_drones, register::register_drone},
    orders::register::register_order,
    product::{
        delete::delete_product, list_by_business::list_products_by_business,
        register::register_product, update::update_product,
    },
    stats::stats_::get_stats,
    trips::register::register_trip,
    users::{
        delete::delete_user,
        login::{AppState, login},
        register::register_handler,
        update::update_user,
    },
};
use utoipa::openapi::{
    security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    tag::TagBuilder,
};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

use crate::routes::{
    business::{
        delete::__path_delete_business, list::__path_list_businesses,
        register::__path_register_business, set_location::__path_set_business_location,
        update::__path_update_business,
    },
    drones::{
        delete::__path_delete_drone, list_drones::__path_list_drones,
        register::__path_register_drone,
    },
    orders::register::__path_register_order,
    product::{
        delete::__path_delete_product, list_by_business::__path_list_products_by_business,
        register::__path_register_product, update::__path_update_product,
    },
    stats::stats_::__path_get_stats,
    trips::register::__path_register_trip,
    users::{
        delete::__path_delete_user, login::__path_login, register::__path_register_handler,
        update::__path_update_user,
    },
};

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
        .routes(routes!(register_order))
        .routes(routes!(register_trip))
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
            .name("Orders")
            .description(Some("Order management endpoints"))
            .build(),
        TagBuilder::new()
            .name("Trips")
            .description(Some("Trip management endpoints"))
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
