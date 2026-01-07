use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn log_request(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let method = req.method().clone();
    let uri = req.uri().clone();

    println!("[{}] {}", method, uri);

    let response = next.run(req).await;

    println!("[{}] {} - Status: {}", method, uri, response.status());

    Ok(response)
}
