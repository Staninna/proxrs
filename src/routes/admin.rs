use crate::*;

use axum::{extract::State, response::Response};
use axum_extra::extract::CookieJar;
use hyper::{Request, StatusCode};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    username: String,
    role: String,
    id: i64,
}

#[derive(Serialize)]
struct Proxy {
    name: String,
    ip: String,
    port: i64,
    id: i64,
}

// Send the login page to the user
pub async fn admin_page(
    State(app_state): State<AppState>,
    _jar: CookieJar,
    _req: Request<Body>,
) -> Response<Body> {
    // Initialize variables
    let (_sessions, _, _conf, tera, _) = app_state.extract();

    // Create the context
    let mut context = tera::Context::new();
    context.insert("title", "Admin");

    context.insert("update_route", "update");
    context.insert("delete_route", "delete");
    context.insert("invite_route", "invite");
    let users = vec![
        User {
            username: "user1".to_string(),
            role: "admin".to_string(),
            id: 1,
        },
        User {
            username: "user2".to_string(),
            role: "user".to_string(),
            id: 2,
        },
    ];
    context.insert("users", &users);
    let proxies = vec![
        Proxy {
            name: "proxy1".to_string(),
            ip: "127.0.0.1".to_string(),
            port: 8080,
            id: 1,
        },
        Proxy {
            name: "proxy2".to_string(),
            ip: "739.293.293.293".to_string(),
            port: 8080,
            id: 2,
        },
    ];
    context.insert("proxies", &proxies);

    // Render the login page
    dbg!(&context);
    let login_page = check_err!(tera.render("admin.tera.html", &context));

    // Send the login page
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(login_page))
        .unwrap()
}
