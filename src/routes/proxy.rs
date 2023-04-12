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
    // Get cookie
    let cookie_name = check_err!(app_state.conf.get(CookieName));
    let cookie = match jar.get(&cookie_name) {
        Some(cookie) => cookie,
        None => {
            // Redirect to login page
            let special_route = check_err!(app_state.conf.get(SpecialRoute));
            return Err(Redirect::to(&format!("{}/login", special_route)));
        }
    };

    // Validate cookie
    match app_state
        .sessions
        .validate_session_by_token(cookie.value())
        .await
    {
        true => (),
        false => {
            // Redirect to login page
            let special_route = check_err!(app_state.conf.get(SpecialRoute));
            return Err(Redirect::to(&format!("{}/login", special_route)));
        }
    }

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
    let res = app_state.client.request(req).await;

    // Return the response
    match res {
        Ok(res) => Ok(res),
        Err(err) => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(format!("Error: {}", err)))
            .unwrap()),
    }
}
