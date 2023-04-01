use crate::{
    auth::{login, login_page, logout},
    config::{ConfigKey::*, ConfigStore},
    session::{get_session_cookie, SessionStore},
};
use hyper::{Body, Client, Method, Request, Response};
use tera::Tera;

// Handles incoming requests
pub async fn proxy(
    req: Request<Body>,
    conf: ConfigStore,
    tera: Tera,
    store: SessionStore,
) -> Result<Response<Body>, hyper::Error> {
    // Check for special routes
    let special_endpoint = conf.get(SpecialRouteEndpoint).await;
    let login_endpoint = special_endpoint.clone() + "/login";
    let logout_endpoint = special_endpoint.clone() + "/logout";
    match (req.method(), req.uri().path()) {
        // Login page
        (&Method::GET, path) if path == &login_endpoint => return login_page(&conf, &tera).await,

        // Login request
        (&Method::POST, path) if path == &login_endpoint => {
            return login(req, conf, &tera, store).await
        }

        // Logout request
        (&Method::POST, path) if path == &logout_endpoint => return logout(req, conf, store).await,

        // TODO: Session debug page /proxrs/session

        // Ignore all other requests
        _ => (),
    }

    // Check if the request has an session cookie
    let session_token = match get_session_cookie(&req, &conf).await {
        Some(session_token) => session_token,
        None => return login_page(&conf, &tera).await,
    };

    // Check if the session token is valid
    let mut token = match store.get_token(&session_token).await {
        Some(token) => match token.is_valid() {
            true => token,
            false => {
                store.remove(&session_token).await;
                return login_page(&conf, &tera).await;
            }
        },
        None => return login_page(&conf, &tera).await,
    };

    // Renew the session token
    token.renew(&conf).await;

    // Build the request to the proxied site
    let method = req.method().clone();
    let headers = req.headers().clone();
    let uri = format!("http://81.173.114.61:8237{}", req.uri());
    let mut new_req = Request::new(req.into_body());
    *new_req.uri_mut() = uri.parse().unwrap();
    *new_req.method_mut() = method;
    *new_req.headers_mut() = headers;

    // Send the request to the proxied site
    let res = Client::new().request(new_req).await?;

    // Send the response back to the client
    Ok(res)
}
