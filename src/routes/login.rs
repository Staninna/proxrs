use crate::{check_err, conf::*, AppState};

use axum::{extract::State, response::Response};
use hyper::{Body, Request, StatusCode};
use serde::Deserialize;

// Send the login page to the user
pub async fn login_page(State(app_state): State<AppState>, req: Request<Body>) -> Response<Body> {
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

    // Get msg from the query
    let msg = req.uri().query().unwrap_or("").replace("msg=", "");
    let msg = match msg.is_empty() {
        // Msg is empty
        true => "".to_string(),

        // Decode the msg
        false => {
            let msg = urlencoding::decode(&msg).unwrap_or(std::borrow::Cow::Borrowed(""));
            format!(
                r#"
                <div class="alert">
                    <span class="closebtn" onclick="this.parentElement.style.display='none';">&times;</span>
                    <p>{}</p>
                </div>"#,
                msg
            )
        }
    };

    // Replace the msg in the login page
    login_page = login_page.replace("{{msg}}", &msg);

    // Send the login page
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(login_page))
        .unwrap()
}

#[derive(Deserialize)]
struct LoginData {
    username: String,
    password: String,
}

pub async fn login_req(State(app_state): State<AppState>, req: Request<Body>) -> Response<Body> {
    // Get data from the request using serde
    let body = match hyper::body::to_bytes(req.into_body()).await {
        Ok(body) => body,
        Err(_) => {
            let special_route = check_err!(app_state.conf.get(SpecialRoute));
            return redirect_to_login(&special_route, "An error occurred"); // TODO: Better error message
        }
    };
    let login_data = match serde_urlencoded::from_bytes::<LoginData>(&body) {
        Ok(data) => data,
        Err(_) => {
            let special_route = check_err!(app_state.conf.get(SpecialRoute));
            return redirect_to_login(&special_route, "An error occurred"); // TODO: Better error message
        }
    };

    // Get the username and password
    let username = login_data.username;
    let password = login_data.password;

    // Check if the username and password are correct // TODO: Add database support
    if username.is_empty() || password.is_empty() {
        let special_route = check_err!(app_state.conf.get(SpecialRoute));
        return redirect_to_login(&special_route, "Username or password is empty");
    }

    // TODO: Tokens and cookies

    // TODO: Redirect to root
    redirect(State(app_state), "Logged in")
}

// Redirect to the login page with a message
fn redirect(State(app_state): State<AppState>, msg: &str) -> Response<Body> {
    // Get special route
    let special_route = check_err!(app_state.conf.get(SpecialRoute));
    let login_route = special_route.to_owned() + "/login";

    // Add message to the query
    let login_route = format!("{}?msg={}", login_route, msg);

// Redirect the user to the login page
fn redirect_to_login(special_route: &str, msg: &str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::FOUND)
        .header(
            "Location",
            format!(
                "{}?msg={}",
                special_route.to_owned() + "/login",
                urlencoding::encode(msg)
            ),
        )
        .body(Body::empty())
        .unwrap()
}
