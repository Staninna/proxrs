use crate::*;

use axum::{
    extract::State,
    response::{Redirect, Response},
};
use axum_extra::extract::CookieJar;
use hyper::{Body, Request, StatusCode, Uri};

pub async fn proxy(
    State(app_state): State<AppState>,
    jar: CookieJar,
    mut req: Request<Body>,
) -> Result<Response<Body>, Redirect> {
    // Initlize variables
    let (sessions, client, conf, _) = app_state.extract();
    let special_route = check_err!(conf.get(SpecialRoute));
    let cookie_name = check_err!(conf.get(CookieName));

    // Get cookie
    let cookie = match jar.get(&cookie_name) {
        Some(cookie) => cookie,
        None => {
            // Redirect to login page
            return Err(Redirect::to(&format!("{}/login", special_route)));
        }
    };

    // Get session
    let mut session = match sessions.get(cookie.value()).await {
        Some(session) => session,
        None => {
            // Redirect to login page
            return Err(Redirect::to(&format!("{}/login", special_route)));
        }
    };

    // Validate session
    if !session.expired() {
        // Redirect to login page
        return Err(Redirect::to(&format!("{}/login", special_route)));
    }

    // Renew session
    session.renew();

    // Get the path
    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    // Create uri
    let uri = format!("https://example.com{}", path_query);

    // Make the Host header match the new uri
    // IDK: if this is necessary when using this for local network requests but it is for the requests to external websites
    let host = uri.replace("https://", "").replace("http://", "");
    let host = host.split('/').next().unwrap();
    req.headers_mut().insert("Host", host.parse().unwrap());

    // Set the new uri
    *req.uri_mut() = uri.parse::<Uri>().unwrap();

    // Do the request
    let res = client.request(req).await;

    // Return the response
    match res {
        Ok(res) => Ok(res),
        Err(err) => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(format!("Error: {}", err)))
            .unwrap()),
    }
}
