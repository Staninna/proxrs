use crate::config::{ConfigKey::*, ConfigStore};
use hyper::{Body, Response, StatusCode};
use serde::Deserialize;
use tera::{Context, Tera};

#[derive(Deserialize)]
struct Activity {
    url: String,
    description: String,
}

pub async fn internal_error(
    conf: &ConfigStore,
    tera: Tera,
) -> Result<Response<Body>, hyper::Error> {
    // Get the internal error page path from the config
    let internal_error_page_path = conf.get(InternalErrorTemplate).await;

    // Get the list of activities
    // TODO: Make this async
    // TODO: Make this not unwrap
    // TODO: Make this configurable in .env
    // TODO: Make this load on startup and not every time
    // TODO: Make this use an database
    let activities: Vec<Activity> = match serde_json::from_str(
        &std::fs::read_to_string("static/error_activities.json").unwrap(),
    ) {
        Ok(activities) => activities,
        Err(e) => {
            dbg!(e);

            let mut response = Response::new(Body::from("Internal error while loading activities"));
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Ok(response);
        }
    };

    // Get a random activity use %
    let activity = activities
        .get(rand::random::<usize>() % activities.len())
        .unwrap();

    // Create the context for the template
    let mut context = Context::new();
    context.insert("activity_url", &activity.url);
    context.insert("activity_description", &activity.description);

    // Load the internal error page using the tera template engine
    let mut response = match tera.render(&internal_error_page_path, &context) {
        Ok(internal_error_page) => Response::new(Body::from(internal_error_page)),
        Err(e) => {
            dbg!(e);

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
