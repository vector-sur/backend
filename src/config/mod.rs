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