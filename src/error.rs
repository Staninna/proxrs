use crate::config::{ConfigKey::*, ConfigStore};
use hyper::{Body, Response, StatusCode};
use tera::Tera;

pub async fn internal_error(
    conf: &ConfigStore,
    tera: Tera,
) -> Result<Response<Body>, hyper::Error> {
    // Get the internal error page path from the config
    let internal_error_page_path = conf.get(InternalErrorTemplate).await;

    // Load the internal error page using the tera template engine
    let mut response = match tera.render(&internal_error_page_path, &tera::Context::new()) {
        Ok(internal_error_page) => Response::new(Body::from(internal_error_page)),
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
