use crate::{
    config::{ConfigKey::*, ConfigStore},
    error::internal_error,
    session::{get_session_cookie, Session, SessionStore, User},
};
use hyper::{header::SET_COOKIE, http::HeaderValue, Body, Request, Response, StatusCode};
use tera::{Context, Tera};

// Handles login requests
pub async fn login(
    req: Request<Body>,
    conf: ConfigStore,
    store: SessionStore,
) -> Result<Response<Body>, hyper::Error> {
    // Extract the request body containing user credentials
    let body_bytes = hyper::body::to_bytes(req.into_body()).await?;

    // Deserialize the request body into a User struct
    let user: User = match serde_urlencoded::from_bytes(&body_bytes) {
        Ok(user) => user,
        Err(_) => return root(None),
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
            None => return root(None),
        };

        // Create the response
        let mut response = Response::new(Body::from(""));

        // Set an session cookie
        let cookie_name = conf.get(SessionCookieName).await;
        let cookie = format!(
            "{}={}; HttpOnly; Path=/; Expires={}",
            cookie_name,
            session.token,
            session.expires.format("%a, %d %b %Y %T GMT")
        );
        response
            .headers_mut()
            .insert(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());

        // Redirect to the home page
        return root(Some(response));
    }
    // Invalid credentials
    return root(None);
}

pub async fn login_page(conf: ConfigStore, tera: Tera) -> Result<Response<Body>, hyper::Error> {
    // Use tera to render the login page
    let login_endpoint = conf.get(SpecialRouteEndpoint).await + "/login";
    let logout_endpoint = conf.get(SpecialRouteEndpoint).await + "/logout";
    let mut context = Context::new();
    context.insert("login_endpoint", &login_endpoint);
    context.insert("logout_endpoint", &logout_endpoint);

    match tera.render("login.html", &context) {
        Ok(html) => {
            let mut response = Response::new(Body::from(html));
            *response.status_mut() = StatusCode::OK;
            response
                .headers_mut()
                .insert("Content-Type", "text/html".parse().unwrap());

            Ok(response)
        }
        Err(_) => return internal_error(&conf).await,
    }
}

pub async fn logout(
    req: Request<Body>,
    conf: ConfigStore,
    store: SessionStore,
) -> Result<Response<Body>, hyper::Error> {
    // Check if the request has an session cookie
    let session_token = match get_session_cookie(&req, &conf).await {
        Some(session_token) => session_token,
        None => return root(None),
    };

    // Remove the session from the sessions map
    store.remove(&session_token).await;

    // Create the response
    let mut response = Response::new(Body::from(""));

    // Unset the session cookie
    let cookie_name = conf.get(SessionCookieName).await;
    let cookie = format!(
        "{}=; HttpOnly; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT",
        cookie_name
    );
    response
        .headers_mut()
        .insert(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());

    // Redirect to the home page
    root(Some(response))
}

fn root(response: Option<Response<Body>>) -> Result<Response<Body>, hyper::Error> {
    let mut response = match response {
        Some(response) => response,
        None => Response::new(Body::from("")),
    };

    // Redirect / of the proxy to the home page
    *response.status_mut() = StatusCode::FOUND;
    response
        .headers_mut()
        .insert("Location", "/".parse().unwrap());

    Ok(response)
}
