use crate::{
    config::{ConfigKey::*, ConfigStore},
    db::Db,
    error::internal_error,
    session::SessionStore,
};
use hyper::{Body, Request, Response, StatusCode};
use tera::{Context, Tera};

pub async fn admin_page(conf: ConfigStore, tera: Tera) -> Result<Response<Body>, hyper::Error> {
    // Use tera to render the admin page
    let admin_endpoint = conf.get(SpecialRouteEndpoint).await + "/admin";
    let mut context = Context::new();
    context.insert("admin_endpoint", &admin_endpoint);

    match tera.render("admin.html", &context) {
        Ok(html) => {
            let mut response = Response::new(Body::from(html));
            *response.status_mut() = StatusCode::OK;
            response
                .headers_mut()
                .insert("Content-Type", "text/html".parse().unwrap());

            Ok(response)
        }
        Err(_) => return internal_error(&conf).await,
    }
}

pub async fn admin(
    req: Request<Body>,
    conf: ConfigStore,
    tera: Tera,
    store: SessionStore,
) -> Result<Response<Body>, hyper::Error> {
    // TODO: use separate admin token for admin stuff with shorter expiration time proxrs-x-admin
    // Check if request contains the admin token and if so render the admin page
    // Otherwise redirect to the admin login page
    todo!()
}

pub async fn add_user(
    db: Db,
    req: Request<Body>,
    conf: ConfigStore,
    tera: Tera,
    store: SessionStore,
) -> Result<Response<Body>, hyper::Error> {
    // TODO: use separate admin token for admin stuff with shorter expiration time proxrs-x-admin
    // Check if user is admin and if so add a new user to the database
    todo!()
}
