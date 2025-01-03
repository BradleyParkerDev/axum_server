use axum::{
    response::Response,
    middleware::Next,
    extract::Request,
};

pub async fn authorize_user(request: Request, next: Next) -> Response {
    println!("Authorization Middleware!!!");

    // Pass the request to the next handler/middleware
    let mut response = next.run(request).await;

    // Optionally, modify the response (e.g., adding a header)
    response.headers_mut().insert(
        "Set-Cookie",
        "axum_session_cookie=This is a session cookie!; HttpOnly".parse().unwrap(),
    );

    response
}
