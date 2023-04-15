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
    let cookie = jar.get(&cookie_name);

    // Match the cookie
    let token = match cookie {
        Some(cookie) => cookie.value(),
        None => {
            // Redirect to the home page
            return Err(Redirect::to(&format!(
                "{}/login?msg={}&status=warning",
                special_route,
                encode("You are not logged in.")
            )));
        }
    };

    // Check if the session token is valid
    if sessions.validate_session_by_token(token, &conf).await {
        // Delete the session
        sessions.delete_session_by_token(token).await;

        // Unset the cookie
        let mut cookie = Cookie::new(cookie_name, "");
        cookie.set_path("/");

        // Redirect to the home page
        Ok((
            jar.add(cookie),
            Redirect::to(&format!(
                "{}/login?msg={}&status=success",
                special_route,
                encode("You have been logged out.")
            )),
        ))
    } else {
        // Redirect to the home page
        Err(Redirect::to(&format!(
            "{}/login?msg={}&status=warning",
            special_route,
            encode("You are not logged in.")
        )))
    }
}
