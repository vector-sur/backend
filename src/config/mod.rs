pub mod database;
use std::net::SocketAddr;
/// Loads environment variables from the [.env](cci:7://file:///home/mateo/dev/backend/.env:0:0-0:0) file.
pub fn load_env() {
    dotenvy::dotenv().ok();
}
/// Returns the server address from the environment variables.
pub fn get_server_addr() -> SocketAddr {
    std::env::var("SOCKET_ADDR")
        .expect("SOCKET_ADDR must be defined in the .env file")
        .parse()
        .expect("SOCKET_ADDR must be a valid address (e.g., 127.0.0.1:3000)")
}

/// Returns the JWT secret key from the environment variables.
pub fn get_jwt_secret() -> String {
    std::env::var("JWT_SECRET").expect("JWT_SECRET must be defined in the .env file")
}

/// Returns the JWT expiration time in hours from the environment variables.
pub fn get_jwt_expiration_hours() -> i64 {
    std::env::var("JWT_EXPIRATION_HOURS")
        .expect("JWT_EXPIRATION_HOURS must be defined in the .env file")
        .parse()
        .expect("JWT_EXPIRATION_HOURS must be a valid number")
}
