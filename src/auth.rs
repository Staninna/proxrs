use hyper::{header::SET_COOKIE, http::HeaderValue, Body, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
}

// Main login that handles both GET and POST requests
pub async fn login_handler(
    req: Request<Body>,
    sessions: Arc<Mutex<HashMap<String, String>>>,
) -> Result<Response<Body>, hyper::Error> {
    match req.method() {
        &hyper::Method::POST => login_post(req, sessions).await,
        &hyper::Method::GET => login_get().await,
        _ => Ok(Response::new(Body::from("Invalid request"))),
    }
}

// Handles POST requests for login
async fn login_post(
    req: Request<Body>,
    sessions: Arc<Mutex<HashMap<String, String>>>,
) -> Result<Response<Body>, hyper::Error> {
    // Extract the request body containing user credentials
    let body_bytes = hyper::body::to_bytes(req.into_body()).await?;

    let user: User = match serde_urlencoded::from_bytes(&body_bytes) {
        Ok(user) => user,
        Err(_) => {
            let mut response = Response::new(Body::from("Invalid credentials"));
            *response.status_mut() = StatusCode::UNAUTHORIZED;

            return Ok(response);
        }
    };

    // Validate user credentials
    // TODO: Replace this with a database lookup
    if !user.username.is_empty() && !user.password.is_empty() {
        // Generate a session token
        let session_token = Uuid::new_v4().to_string();
        sessions
            .lock()
            .await
            .insert(session_token.clone(), user.username);

        // Build the response
        let mut response = Response::new(Body::from("Logged in"));
        // Set an session cookie
        let cookie = format!("proxrs-x={}; HttpOnly; Path=/", session_token);
        response
            .headers_mut()
            .insert(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());

        // Return the response
        Ok(response)
    }
    // If the user is not valid, return an unauthorized response
    else {
        let mut response = Response::new(Body::from("Invalid credentials"));
        *response.status_mut() = StatusCode::UNAUTHORIZED;

        Ok(response)
    }
}

// Handles GET requests for login
async fn login_get() -> Result<Response<Body>, hyper::Error> {
    // Build the response with the login.html content
    let mut response = Response::new(Body::from(include_str!("..\\static\\login.html")));
    // Set the content type header
    response
        .headers_mut()
        .insert("Content-Type", "text/html".parse().unwrap());

    // Return the response
    Ok(response)
}
