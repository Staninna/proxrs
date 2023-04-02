use crate::{
    auth::root,
    config::{ConfigKey::*, ConfigStore},
    db::Db,
    error::internal_error,
    session::{get_session_cookie, SessionStore},
};
use hyper::{Body, Request, Response};
use tera::Tera;

pub async fn admin_page(
    db: Db,
    req: Request<Body>,
    conf: ConfigStore,
    tera: Tera,
    store: SessionStore,
) -> Result<Response<Body>, hyper::Error> {
    let session_token = match get_session_cookie(&req, &conf).await {
        Some(token) => token,
        None => return root(None),
    };

    let mut token = match store.get_token(&session_token).await {
        Some(token) => match token.is_valid() {
            true => token,
            false => {
                store.remove(&session_token).await;
                return root(None);
            }
        },
        None => return root(None),
    };

    // Renew the session token
    token.renew(&conf).await;

    // Render the admin page
    let add_user_endpoint = conf.get(SpecialRouteEndpoint).await + "/add_user";
    let remove_user_endpoint = conf.get(SpecialRouteEndpoint).await + "/remove_user";
    let users = db.get_users().await;
    let mut context = tera::Context::new();
    context.insert("add_user_endpoint", &add_user_endpoint);
    context.insert("remove_user_endpoint", &remove_user_endpoint);
    context.insert("users", &users);
    match tera.render("admin.html", &context) {
        Ok(body) => Ok(Response::new(Body::from(body))),
        Err(_) => internal_error(&tera).await,
    }
}
