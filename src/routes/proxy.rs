use crate::AppState;

use axum::{extract::State, response::Response};
use hyper::{Body, Request, StatusCode, Uri};

pub async fn proxy(State(app_state): State<AppState>, mut req: Request<Body>) -> Response<Body> {
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
        Ok(res) => res,
        Err(err) => {
            eprintln!("Error: {}", err);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal Server Error"))
                .unwrap()
        }
    }
}
