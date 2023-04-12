use crate::{check_err, conf::*, AppState};

use axum::{extract::State, response::Redirect};
use axum_extra::extract::cookie::CookieJar;
use hyper::{Body, Request};

// Log user out
pub async fn logout(
    State(app_state): State<AppState>,
    mut jar: CookieJar,
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

            // Delete the cookie
            jar = jar.remove(cookie);

            // Redirect to the home page
            let msg = "You have been logged out";
            Ok((
                jar,
                Redirect::to(&format!("{}/?msg={}", special_route, msg)),
            ))
        } else {
            // Redirect to the home page
            let msg = "You are not logged in";
            Err(Redirect::to(&format!(
                "{}/login?msg={}",
                special_route, msg
            )))
        }
    } else {
        // Redirect to the home page
        let msg = "You are not logged in";
        Err(Redirect::temporary(&format!(
            "{}/login?msg={}",
            special_route, msg
        )))
    }
}
