use crate::*;

use axum::{extract::State, response::Redirect};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use hyper::{Body, Request, Response, StatusCode};
use serde::Deserialize;
use sha2::Digest;
use urlencoding::{decode, encode};

// Send the login page to the user
pub async fn login_page(
    State(app_state): State<AppState>,
    jar: CookieJar,
    req: Request<Body>,
) -> Response<Body> {
    // Initialize variables
    let (sessions, _, conf, tera, _) = app_state.extract();
    let special_route = check_err!(conf.get(SpecialRoute));
    let mut context = tera::Context::new();

    // Generate the routes
    context.insert("login_route", &(special_route.to_owned() + "/login"));
    context.insert("logout_route", &(special_route.to_owned() + "/logout"));
    context.insert("admin_route", &(special_route.to_owned() + "/admin"));

    // Get cookie
    let cookie_name = check_err!(conf.get(CookieName));
    let cookie = jar.get(&cookie_name);

    // Get session
    let session = match cookie {
        Some(cookie) => sessions.get(cookie.value()).await,
        None => None,
    };

    // Get the username, admin and logged in status from the session
    if let Some(session) = session {
        // Capitalize the first letter of the username
        let mut username = session.user.chars();
        let first = username.next().unwrap().to_uppercase().to_string();
        let rest = username.as_str();
        let username = first + rest;

        // Add the username, admin and logged in status to the context
        context.insert("admin", &session.admin);
        context.insert("logged_in", &true);
        context.insert("title", &format!("Welcome, {}!", username));
    } else {
        // Add the username, admin and logged in status to the context
        context.insert("admin", &false);
        context.insert("logged_in", &false);
        context.insert("title", &"Log in");
    }

    // Get the msg and color from the query
    if let Some(msg) = get_query_param(&req, "msg") {
        let msg_url_safe = match decode(&msg) {
            Ok(msg) => msg,
            Err(_) => todo!("Handle error when decoding msg"),
        };

        context.insert("msg", &msg_url_safe);
    }
    if let Some(status) = get_query_param(&req, "status") {
        context.insert("status", &status);
    }

    // Render the login page
    let login_page = check_err!(tera.render("login.tera.html", &context));

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
    // Initialize variables
    let (mut sessions, _, conf, _, db) = app_state.extract();
    let special_route = check_err!(conf.get(SpecialRoute));

    // Get data from the request using serde
    let body = match hyper::body::to_bytes(req.into_body()).await {
        Ok(body) => body,
        Err(_) => {
            return Err(Redirect::to(&format!(
                "{}/login?msg={}&status=error",
                &special_route,
                encode("Oops! Something went wrong. Please give it another try.")
            )));
        }
    };
    let login_data = match serde_urlencoded::from_bytes::<LoginData>(&body) {
        Ok(data) => data,
        Err(_) => {
            return Err(Redirect::to(&format!(
                "{}/login?msg={}&status=error",
                &special_route,
                encode("Oops! We couldn't process the information you provided. Can you please try again?")
            )));
        }
    };

    // Get the username and password
    let (username, password) = (login_data.username, login_data.password);

    // Hash the password
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    let password = hex::encode(hasher.finalize());

    // Validate the user
    let db_result = db.validate_user(&username, &password).await;
    let valid_user = match db_result {
        Ok(valid_user) => valid_user,
        Err(_) => {
            return Err(Redirect::to(&format!(
                "{}/login?msg={}&status=error",
                &special_route,
                encode("Oops! Something went wrong. Please give it another try.")
            )));
        }
    };

    // Give response if the user is not valid
    if !valid_user {
        return Err(Redirect::to(&format!(
            "{}/login?msg={}&status=warning",
            &special_route,
            encode("Sorry, either your username or password is incorrect. Please double-check and try again.")
        )));
    }

    // Get cookie name
    let cookie_name = check_err!(conf.get(CookieName));
    let cookie = jar.get(&cookie_name);

    // Get session
    let session = match cookie {
        Some(cookie) => sessions.get(cookie.value()).await,
        None => None,
    };

    // Get the username from the session
    let username_from_session = match session {
        Some(mut session) => {
            // Get variables from the session
            let username = session.user.clone();

            // Check if the session is expired
            if session.expired() {
                None
            } else {
                // Renew the session
                session.renew();

                Some(username)
            }
        }
        None => None,
    };

    // Check if the user is already logged in
    if username_from_session.is_some() {
        return Err(Redirect::to(&format!(
            "{}/login?msg={}&status=warning",
            &special_route,
            encode("You are already logged in.")
        )));
    }

    // Create a new session
    let session = sessions.new_session(username, &conf, &db).await;

    // Create a new cookie
    let mut cookie = Cookie::new(cookie_name, session.token);
    cookie.set_path("/");

    // Redirect the user to the home page
    Ok((
        jar.add(cookie),
        Redirect::to(&format!(
            "{}/login?msg={}&status=success",
            &special_route,
            encode("You have successfully logged in.")
        )),
    ))
}

fn get_query_param(req: &Request<Body>, param: &str) -> Option<String> {
    // Get the query
    let query = req.uri().query().unwrap_or("");

    // Get the param
    let param = query
        .split('&')
        .filter(|s| s.starts_with(&format!("{}=", param)))
        .map(|s| s.trim_start_matches(&format!("{}=", param)))
        .next()
        .unwrap_or("")
        .to_string();

    // Check if the param is empty
    if param.is_empty() {
        None
    } else {
        Some(param)
    }
}
