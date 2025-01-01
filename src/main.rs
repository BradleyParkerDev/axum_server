// mod controllers;
// mod routes;

// use axum::{Router, serve};
// use tokio::net::TcpListener;

// #[tokio::main]
// async fn main() {
//     // Merge the web routes and API routes into a single router
//     let app = routes::web::web_routes()
//         .merge(Router::new().nest("/api/users", routes::users::user_routes()));

//     // Bind the server to a TCP listener on port 3001
//     let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();

//     println!("Server running on http://127.0.0.1:3001");

//     // Run the server
//     serve(listener, app).await.unwrap();
// }

mod controllers;
mod routes;

use axum::{Router,serve};
use tower_http::services::fs::ServeDir; // Correct import path
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Create the app with your routes
    let app = Router::new()
        // Mount the `public` directory for serving static files
        .nest_service("/public", ServeDir::new("public"))
        .merge(routes::web::web_routes()) // Merge the web routes
        .merge(Router::new().nest("/api/users", routes::users::user_routes())); // Merge API user routes

    // Bind the server to a TCP listener on port 3001
    let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();

    println!("Server running on http://127.0.0.1:3001");

    // Run the server using `serve`
    serve(listener, app).await.unwrap();
}

