use crate::config::ConfigStore;
use hyper::{Body, Request};

pub async fn get_session_cookie(req: &Request<Body>, conf: &ConfigStore) -> Option<String> {
    // Get the session cookie name from the config
    let cookie_name = conf.get("session_cookie").await;

    // Get the cookie header from the request
    let cookie_header = match req.headers().get("Cookie") {
        Some(cookie_header) => cookie_header,
        None => return None,
    };

    // Get the session token from the cookie header
    match cookie_header.to_str() {
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
                    Some(session_token.to_string())
                }
                None => None,
            }
        }
        Err(_) => None,
    }
}
