use hyper::{Body, Response, StatusCode};
use tera::{Context, Tera};

pub async fn internal_error(tera: &Tera) -> Result<Response<Body>, hyper::Error> {
    let mut response = match tera.render("internal_error.html", &Context::new()) {
        Ok(html) => Response::new(Body::from(html)),
        // Error while loading the internal error page
        Err(_) => Response::new(Body::from(
            "Internal error while loading internal error page",
        )),
    };

    // Set the status code
    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

    // Set the content type header
    response
        .headers_mut()
        .insert("Content-Type", "text/html".parse().unwrap());

    Ok(response)
}
