use crate::*;

use axum::{extract::State, response::Redirect};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use hyper::{Body, Request};
use urlencoding::encode;

// Log user out
pub async fn logout(
    State(app_state): State<AppState>,
    jar: CookieJar,
    _req: Request<Body>,
) -> Result<(CookieJar, Redirect), Redirect> {
    // Initialize variables
    let (mut sessions, _, conf, _) = app_state.extract();
    let special_route = check_err!(conf.get(SpecialRoute));
    let cookie_name = check_err!(conf.get(CookieName));

    // Get cookie
    let cookie = match jar.get(&cookie_name) {
        Some(cookie) => cookie,
        None => {
            // Redirect to login page
            return Err(Redirect::to(&format!(
                "{}/login?msg={}&status=warning",
                special_route,
                encode("You are not logged in.")
            )));
        }
    };

    // Get session
    let session = match sessions.get(cookie.value()).await {
        Some(session) => session,
        None => {
            // Redirect to login page
            return Err(Redirect::to(&format!(
                "{}/login?msg={}&status=warning",
                special_route,
                encode("You are not logged in.")
            )));
        }
    };

    // Check if the session is expired
    if session.expired() {
        // Redirect to the login page
        return Err(Redirect::to(&format!(
            "{}/login?msg={}&status=warning",
            special_route,
            encode("You are not logged in.")
        )));
    }

    // Delete the session
    match sessions.delete(session).await {
        Ok(_) => (),
        Err(()) => {
            // Redirect to the login page
            return Err(Redirect::to(&format!(
                "{}/login?msg={}&status=warning",
                special_route,
                encode("You are not logged in.")
            )));
        }
    }

    // Unset the cookie
    let mut cookie = Cookie::new(cookie_name, "");
    cookie.set_path("/");

    // Redirect to the login page
    Ok((
        jar.add(cookie),
        Redirect::to(&format!(
            "{}/login?msg={}&status=success",
            special_route,
            encode("You have been logged out.")
        )),
    ))
}
