mod config;
mod middleware;
mod models;
mod routes;

use axum::{
    Router,
    routing::{get, post, delete},
};
use routes::users::login::{AppState, login};
use routes::users::register::register;
use routes::users::delete::delete_user;
use routes::protected::protected;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    config::load_env();
    let pool = config::database::create_pool()
        .await
        .expect("Failed to connect to the database");
    let state = AppState { db: pool };

    let app = Router::new()
        // Public routes
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        // Protected routes
        .route("/protected", get(protected))
        .route("/users/{id}", delete(delete_user))
        .with_state(state);

    let addr = config::get_server_addr();
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
