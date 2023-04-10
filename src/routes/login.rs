use crate::{conf::*, err, AppState};

use axum::{extract::State, response::Response};
use hyper::{Body, Request};

// Send the login page to the user
pub async fn login(State(app_state): State<AppState>, _req: Request<Body>) -> Response<Body> {
    // Get the login page
    let static_dir = err!(app_state.conf.get(StaticDir));
    let login_page = static_dir + "/login.html";

    // Read the login page
    let login_page = err!(tokio::fs::read_to_string(login_page).await);

    // Send the login page
    Response::builder()
        .status(200)
        .body(Body::from(login_page))
        .unwrap()
}
