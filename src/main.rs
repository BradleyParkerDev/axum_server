mod controllers;
mod routes;
mod utils;

use axum::{
    Router,
    middleware,
    serve,
    http::Method
};
use tower_http::services::fs::ServeDir; 

use tower_http::{
    cors::{Any, CorsLayer},
    compression::CompressionLayer,
    trace::TraceLayer,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {

    // Middleware Definitions
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    let compression = CompressionLayer::new();
    let logging = TraceLayer::new_for_http();


    // App Creation
    let app = Router::new()
        // Mount the `public` directory for serving static files
        .nest_service("/public", ServeDir::new("public"))
        // Application Routes
        .merge(routes::web::web_routes()) // Merge the web routes
        .merge(Router::new().nest("/api/users", routes::users::user_routes()))
        // Middleware Layers
        .layer(cors)
        .layer(middleware::from_fn(utils::auth::authorize_user)) // Authorization Middleware
        .layer(compression)
        .layer(logging);

    // Bind the server to a TCP listener on port 3001
    let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();

    println!("Server running on http://127.0.0.1:3001");

    // Run the server using `serve`
    serve(listener, app).await.unwrap();
}
  