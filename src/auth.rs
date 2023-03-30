use crate::{
    config::{ConfigKey::*, ConfigStore},
    session::{SessionStore, User},
    utils::get_session_cookie,
    Session,
};
use hyper::{header::SET_COOKIE, http::HeaderValue, Body, Request, Response, StatusCode};
use std::path::PathBuf;

// Main login that handles both GET and POST requests
pub async fn handler(
    req: Request<Body>,
    conf: ConfigStore,
    store: SessionStore,
) -> Result<Response<Body>, hyper::Error> {
    let auth_path = conf.get(AuthPath).await;
    let login_path = format!("{}{}", auth_path, conf.get(LoginPath).await);
    let logout_path = format!("{}{}", auth_path, conf.get(LogoutPath).await);
    let renew_path = format!("{}{}", auth_path, conf.get(RenewPath).await);

    let path = req.uri().path();
    match (req.method(), path) {
        // Login
        (&hyper::Method::GET, path) if path == login_path => login_get(conf).await,
        (&hyper::Method::POST, path) if path == login_path => login_post(req, conf, store).await,

        // Logout
        (&hyper::Method::GET, path) if path == logout_path => logout(req, conf, store).await,

        // Renew session
        (&hyper::Method::GET, path) if path == renew_path => renew(req, conf, store).await,

        // Invalid request
        _ => {
            // list of valid routes
            // TODO: Add a way to dynamically generate this list
            let routes = vec![login_path, logout_path, renew_path];
            let methods = vec!["GET/POST", "GET", "GET"];

            // Merge the routes and methods into a single vector of strings for the response body
            let routes = routes
                .iter()
                .zip(methods.iter())
                .map(|(route, method)| format!("{} ({})", route, method))
                .collect::<Vec<String>>();

            // Build the response with a list of valid routes + Methods in the body
            let mut response = Response::new(Body::from(format!(
                "Invalid route. Valid routes: {}",
                routes.join(", ")
            )));

            // Set the status code to 404
            *response.status_mut() = StatusCode::NOT_FOUND;

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
        let session = match Session::new(user.username, &conf, &store).await {
            Some(token) => {
                // Add the session to the sessions map
                store.add(token.clone()).await;
                token
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
        let cookie_name = conf.get(SessionCookieName).await;
        let cookie = format!(
            "{}={}; HttpOnly; Path=/; Expires={}",
            cookie_name,
            session.token,
            session.expires.format("%a, %d %b %Y %T GMT")
        );

        dbg!(&cookie);

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

// Send the login page to the client
async fn login_get(conf: ConfigStore) -> Result<Response<Body>, hyper::Error> {
    let static_dir = conf.get(StaticPath).await;
    let login_page_path = PathBuf::from(&static_dir).join(conf.get(LoginPage).await);

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
    if store.get_token(&session_token).await.is_none() {
        let mut response = Response::new(Body::from("Invalid session"));
        *response.status_mut() = StatusCode::UNAUTHORIZED;

        return Ok(response);
    }

    // Remove the session token from the sessions hashmap
    store.remove(&session_token).await;

    // Build the response
    let mut response = Response::new(Body::from("Logged out"));

    // Set the session cookie to an empty string
    let cookie_name = conf.get(SessionCookieName).await;
    let cookie = format!("{}=; HttpOnly; Path=/", cookie_name);
    response
        .headers_mut()
        .insert(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());

    // Return the response
    Ok(response)
}

// Renew the session token
pub async fn renew(
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
    if store.get_token(&session_token).await.is_none() {
        let mut response = Response::new(Body::from("Invalid session"));
        *response.status_mut() = StatusCode::UNAUTHORIZED;

        return Ok(response);
    }

    // Renew the session token
    let mut session = store.get_token(&session_token).await.unwrap();
    session.renew(&conf).await;

    // Build the response
    let mut response = Response::new(Body::from("Session renewed"));

    // Set the session cookie to the new session token
    let cookie_name = conf.get(SessionCookieName).await;
    let cookie = format!("{}={}; HttpOnly; Path=/", cookie_name, session.token);
    response
        .headers_mut()
        .insert(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());

    // Return the response
    Ok(response)
}
