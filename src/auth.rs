use crate::config::get_value;
use hashbrown::HashMap;
use hyper::{header::SET_COOKIE, http::HeaderValue, Body, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
}

// Main login that handles both GET and POST requests
pub async fn handler(
    req: Request<Body>,
    conf: Arc<Mutex<HashMap<String, String>>>,
    sessions: Arc<Mutex<HashMap<String, String>>>,
) -> Result<Response<Body>, hyper::Error> {
    let auth_path = get_value(&conf, "auth_path").await;
    let path = req.uri().path();

    match (req.method(), path) {
        // Login
        (&hyper::Method::GET, path) if path == format!("{}/login", auth_path) => login_get().await,
        (&hyper::Method::POST, path) if path == format!("{}/login", auth_path) => {
            login_post(req, conf, sessions).await
        }

        // Logout
        (&hyper::Method::GET, path) if path == format!("{}/logout", auth_path) => {
            logout(req, conf, sessions).await
        }

        // Invalid request
        _ => {
            let mut response = Response::new(Body::from("Invalid request"));
            *response.status_mut() = StatusCode::BAD_REQUEST;

            Ok(response)
        }
    }
}

// Handles POST requests for login
async fn login_post(
    req: Request<Body>,
    conf: Arc<Mutex<HashMap<String, String>>>,
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
        let cookie_name = get_value(&conf, "session_cookie_name").await;
        let cookie = format!("{}={}; HttpOnly; Path=/", cookie_name, session_token);
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

async fn logout(
    req: Request<Body>,
    conf: Arc<Mutex<HashMap<String, String>>>,
    sessions: Arc<Mutex<HashMap<String, String>>>,
) -> Result<Response<Body>, hyper::Error> {
    // use get_session_cookie function to extract the session token from the request
    let session_token = match get_session_cookie(&req, conf.clone()).await {
        Some(session_token) => session_token,
        None => {
            let mut response = Response::new(Body::from("Invalid session"));
            *response.status_mut() = StatusCode::UNAUTHORIZED;

            return Ok(response);
        }
    };

    // Check if the session token is valid
    if !sessions.lock().await.contains_key(&session_token) {
        let mut response = Response::new(Body::from("Invalid session"));
        *response.status_mut() = StatusCode::UNAUTHORIZED;

        return Ok(response);
    }

    // Remove the session token from the sessions hashmap
    sessions.lock().await.remove(&session_token);

    // Build the response
    let mut response = Response::new(Body::from("Logged out"));

    // Set the session cookie to an empty string
    let cookie_name = get_value(&conf, "session_cookie_name").await;
    let cookie = format!("{}=; HttpOnly; Path=/", cookie_name);
    response
        .headers_mut()
        .insert(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());

    // Return the response
    Ok(response)
}

async fn get_session_cookie(
    req: &Request<Body>,
    conf: Arc<Mutex<HashMap<String, String>>>,
) -> Option<String> {
    // Get the session cookie name from the config
    let cookie_name = get_value(&conf, "session_cookie_name").await;

    // Get the cookie header from the request
    let cookie_header = match req.headers().get("Cookie") {
        Some(cookie_header) => cookie_header,
        None => return None,
    };

    // Get the session token from the cookie header
    let session_token = match cookie_header.to_str() {
        Ok(cookie_header) => {
            // Split the cookie header into individual cookies
            let cookies: Vec<&str> = cookie_header.split(";").collect();

            // Find the session cookie
            let session_cookie = cookies
                .iter()
                .find(|cookie| cookie.starts_with(&cookie_name));

            // Get the session token from the session cookie
            match session_cookie {
                Some(session_cookie) => {
                    let session_token = session_cookie.split('=').collect::<Vec<&str>>()[1];
                    session_token.to_string()
                }
                None => return None,
            }
        }
        Err(_) => return None,
    };

    Some(session_token)
}
