use crate::{check_err, conf::*, AppState};

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
    // Extract the app state
    let conf = app_state.conf;
    let mut sessions = app_state.sessions;

    // Get special routes
    let special_route = check_err!(conf.get(SpecialRoute));

    // Get cookie name
    let cookie_name = check_err!(conf.get(CookieName));

    // Get cookie
    let cookie = jar.get(&cookie_name).cloned();

    // Check if the cookie exists
    if let Some(cookie) = cookie {
        // Get the session token
        let session_token = cookie.value();

        // Check if the session token is valid
        if sessions.validate_session_by_token(session_token).await {
            // Delete the session
            sessions.delete_session_by_token(session_token).await;

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
    } else {
        // Redirect to the home page
        Err(Redirect::to(&format!(
            "{}/login?msg={}&status=waring",
            special_route,
            encode("You are not logged in.")
        )))
    }
}
