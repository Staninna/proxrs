use crate::config::{ConfigKey::*, ConfigStore};
use hyper::{Body, Response, StatusCode};

pub async fn internal_error(conf: &ConfigStore) -> Result<Response<Body>, hyper::Error> {
    // Get the internal error page path from the config
    let static_dir = conf.get(StaticDir).await;
    let internal_error_page_path = conf.get(InternalErrorPage).await;
    let internal_error_page_path = format!("{}/{}", static_dir, internal_error_page_path);

    // Load the internal error page
    let mut response = match tokio::fs::read_to_string(internal_error_page_path).await {
        Ok(internal_error_page) => {
            // Create the response
            let mut response = Response::new(Body::from(internal_error_page));
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            response
        }
        // Error while loading the internal error page
        Err(_) => {
            let mut response = Response::new(Body::from(
                "Internal error while loading internal error page",
            ));
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            response
        }
    };

    // Set the content type header
    response
        .headers_mut()
        .insert("Content-Type", "text/html".parse().unwrap());

    Ok(response)
}
