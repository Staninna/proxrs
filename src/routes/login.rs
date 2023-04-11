use crate::{check_err, conf::*, AppState};

use axum::{extract::State, response::Response};
use hyper::{Body, Request};

// Send the login page to the user
pub async fn login(State(app_state): State<AppState>, _req: Request<Body>) -> Response<Body> {
    // Get the login page
    let static_dir = check_err!(app_state.conf.get(StaticDir));
    let login_page = static_dir + "/login.html";

    // Read the login page
    let mut login_page = check_err!(tokio::fs::read_to_string(login_page).await);

    // Get special routes
    let special_route = check_err!(app_state.conf.get(SpecialRoute));
    let login_route = special_route.to_owned() + "/login";
    let logout_route = special_route.to_owned() + "/logout";

    // Replace the special routes in the login page
    login_page = login_page.replace("{{login_route}}", &login_route);
    login_page = login_page.replace("{{logout_route}}", &logout_route);

    // Send the login page
    Response::builder()
        .status(200)
        .body(Body::from(login_page))
        .unwrap()
}
