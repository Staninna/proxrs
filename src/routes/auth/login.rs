use crate::{check_err, conf::*, AppState};

use axum::{
    extract::State,
    response::{Redirect, Response},
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
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
    // TODO: Check if user already logged in if yes say hello
    let msg = req.uri().query().unwrap_or("").replace("msg=", "");
    let msg = match msg.is_empty() {
        // Msg is empty
        true => "".to_string(),

        // Decode the msg // TODO: Make that the alert can have different colors based on the msg
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

pub async fn login_req(
    State(app_state): State<AppState>,
    jar: CookieJar,
    req: Request<Body>,
) -> Result<(CookieJar, Redirect), Redirect> {
    // Get data from the request using serde
    let body = match hyper::body::to_bytes(req.into_body()).await {
        Ok(body) => body,
        Err(_) => {
            let special_route = check_err!(app_state.conf.get(SpecialRoute));
            return Err(Redirect::to(&format!(
                "{}/login?msg={}",
                &special_route,
                urlencoding::encode("Oops! Something went wrong. Please give it another try.")
            )));
        }
    };
    let login_data = match serde_urlencoded::from_bytes::<LoginData>(&body) {
        Ok(data) => data,
        Err(_) => {
            let special_route = check_err!(app_state.conf.get(SpecialRoute));
            return Err(Redirect::to(&format!(
                "{}/login?msg={}",
                &special_route,
                urlencoding::encode("Oops! We couldn't process the information you provided. Can you please try again?")
            )));
        }
    };

    // Get the username and password
    let (username, password) = (login_data.username, login_data.password);

    // Check if the username and password are correct // TODO: Add database support
    if username.is_empty() || password.is_empty() {
        let special_route = check_err!(app_state.conf.get(SpecialRoute));
        return Err(Redirect::to(&format!(
            "{}/login?msg={}",
            &special_route,
            urlencoding::encode("Sorry, either your username or password is incorrect. Please double-check and try again.")
        )));
    }

    // Create a new session
    let session = match app_state.to_owned().sessions.new_session(username).await {
        // No session was found of the user
        Some(session) => session,

        // Session was found of the user
        // TODO: Check cookie to see if the user is already logged in
        None => {
            let special_route = check_err!(app_state.conf.get(SpecialRoute));
            return Err(Redirect::to(&format!(
                "{}/login?msg={}",
                &special_route,
                urlencoding::encode("You are already logged in. No need to log in again.")
            )));
        }
    };

    // Get cookie name
    let cookie_name = check_err!(app_state.conf.get(CookieName));

    // Create a new cookie
    let mut cookie = Cookie::new(cookie_name, session.token);
    cookie.set_path("/");
    // TODO: set expiration time

    // Redirect the user to the home page
    Ok((jar.add(cookie), Redirect::to("/")))
}
