use crate::{config::ConfigStore, session::SessionStore, utils::get_session_cookie};
use hyper::{Body, Request, Response, StatusCode};

pub async fn proxy_handler(
    req: Request<Body>,
    conf: ConfigStore,
    store: SessionStore,
) -> Result<Response<Body>, hyper::Error> {
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

    Ok(Response::new(Body::from("Proxy")))
}
