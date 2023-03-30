use crate::{config::ConfigStore, session::SessionStore, utils::get_session_cookie, Session};
use hyper::{header::SET_COOKIE, http::HeaderValue, Body, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
}

// Main login that handles both GET and POST requests
pub async fn handler(
    req: Request<Body>,
    conf: ConfigStore,
    store: SessionStore,
) -> Result<Response<Body>, hyper::Error> {
    let auth_path = conf.get("auth_path").await;
    let login_path = format!("{}{}", auth_path, conf.get("login_path").await);
    let logout_path = format!("{}{}", auth_path, conf.get("logout_path").await);

    let path = req.uri().path();
    match (req.method(), path) {
        // Login
        (&hyper::Method::GET, path) if path == login_path => login_get(conf).await,
        (&hyper::Method::POST, path) if path == login_path => login_post(req, conf, store).await,

        // Logout
        (&hyper::Method::GET, path) if path == logout_path => logout(req, conf, store).await,

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
    conf: ConfigStore,
    store: SessionStore,
) -> Result<Response<Body>, hyper::Error> {
    // Extract the request body containing user credentials
    let body_bytes = hyper::body::to_bytes(req.into_body()).await?;

    let user: User = match serde_urlencoded::from_bytes(&body_bytes) {
        Ok(user) => user,
        Err(_) => {
            let mut response = Response::new(Body::from("Invalid credentials"));
            *response.status_mut() = StatusCode::BAD_REQUEST;

            return Ok(response);
        }
    };

    // Validate user credentials
    // TODO: Replace this with a database lookup
    if !user.username.is_empty() && !user.password.is_empty() {
        // Generate a session token
        let session_token = match Session::new(user.username, &conf, &store).await {
            Some(token) => {
                // Add the session to the sessions map
                store.add(token.clone()).await;
                token.token
            }
            None => {
                let mut response = Response::new(Body::from("User already logged in"));
                *response.status_mut() = StatusCode::BAD_REQUEST;

                return Ok(response);
            }
        };

        // Build the response
        let mut response = Response::new(Body::from("Logged in"));

        // Set an session cookie
        let cookie_name = conf.get("session_cookie_name").await;
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
async fn login_get(conf: ConfigStore) -> Result<Response<Body>, hyper::Error> {
    let static_dir = conf.get("static_path").await;
    let login_page_path = PathBuf::from(&static_dir).join(conf.get("login_page").await);

    // Read the login page from the file system
    let file = tokio::fs::read_to_string(login_page_path).await;

    let login_page = match file {
        Ok(login_page) => login_page,
        Err(_) => {
            let mut response = Response::new(Body::from("Internal server error"));
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

            return Ok(response);
        }
    };

    // Set the content type header
    let mut response = Response::new(Body::from(login_page));
    response
        .headers_mut()
        .insert("content-type", HeaderValue::from_static("text/html"));

    // Return the response
    Ok(response)
}

async fn logout(
    req: Request<Body>,
    conf: ConfigStore,
    store: SessionStore,
) -> Result<Response<Body>, hyper::Error> {
    // use get_session_cookie function to extract the session token from the request
    let session_token = match get_session_cookie(&req, &conf).await {
        Some(session_token) => session_token,
        None => {
            let mut response = Response::new(Body::from("Invalid session"));
            *response.status_mut() = StatusCode::UNAUTHORIZED;

            return Ok(response);
        }
    };

    // Check if the session token is valid
    if let None = store.get_token(&session_token).await {
        let mut response = Response::new(Body::from("Invalid session"));
        *response.status_mut() = StatusCode::UNAUTHORIZED;

        return Ok(response);
    }

    // Remove the session token from the sessions hashmap
    store.remove(&session_token).await;

    // Build the response
    let mut response = Response::new(Body::from("Logged out"));

    // Set the session cookie to an empty string
    let cookie_name = conf.get("session_cookie_name").await;
    let cookie = format!("{}=; HttpOnly; Path=/", cookie_name);
    response
        .headers_mut()
        .insert(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());

    // Return the response
    Ok(response)
}
