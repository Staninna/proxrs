use crate::*;

use axum::{extract::State, response::Redirect};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use hyper::{Body, Request, Response, StatusCode};
use serde::Deserialize;
use urlencoding::{decode, encode};

// Send the login page to the user
pub async fn login_page(
    State(app_state): State<AppState>,
    jar: CookieJar,
    req: Request<Body>,
) -> Response<Body> {
    // Extract the app state
    let conf = app_state.conf;
    let sessions = app_state.sessions;

    // Get special routes
    let special_route = check_err!(conf.get(SpecialRoute));

    // Get the login page
    let static_dir = check_err!(conf.get(StaticDir));
    let login_page = static_dir + "/login.html";

    // Read the login page
    let mut login_page = check_err!(tokio::fs::read_to_string(login_page).await);

    // Get the login and logout routes
    let login_route = special_route.to_owned() + "/login";
    let logout_route = special_route.to_owned() + "/logout";

    // Replace the special routes in the login page
    login_page = login_page.replace("{{login_route}}", &login_route);
    login_page = login_page.replace("{{logout_route}}", &logout_route);

    // Get username from cookie
    let cookie_name = check_err!(conf.get(CookieName));
    let username = if let Some(cookie) = jar.get(&cookie_name).cloned() {
        // Get the session token
        let session_token = cookie.value();

        // Get user from the session token
        let user = sessions.get_user_by_token(session_token).await;

        // Check if the session token is valid
        if sessions
            .validate_session_by_token(session_token, &conf)
            .await
        {
            Some(user)
        } else {
            None
        }
    } else {
        None
    };

    // Get the title
    let mut login_enabled = "disabled";
    let title = match username {
        // User is logged in
        Some(username) => {
            // Capitalize the first letter of the username
            let mut username = username.chars();
            let first = username.next().unwrap().to_uppercase().to_string();
            let rest = username.as_str();

            // Enable the home button
            login_enabled = "enabled";

            // Return the title
            format!("Welcome, {}!", first + rest)
        }
        // User is not logged in
        None => "Log in".to_string(),
    };

    // Replace the title in the login page
    login_page = login_page.replace("{{title}}", &title);

    // Replace the enabled in the login page
    login_page = login_page.replace("{{login_enabled}}", login_enabled);

    // Get the msg and color from the query
    let msg = get_query_param(&req, "msg").unwrap_or("".to_string());
    let status = get_query_param(&req, "status").unwrap_or("".to_string());

    let msg = match msg.is_empty() {
        // Msg is empty
        true => "".to_string(),

        // Decode the msg
        false => {
            let msg = decode(&msg).unwrap();
            format!(
                r#"
                <div class="alert {}">
                    <span class="closebtn" onclick="closeAlert();">&times;</span>
                    <p>{}</p>
                </div>"#,
                status, msg
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
    // Extract the app state
    let mut sessions = app_state.sessions;
    let conf = app_state.conf;
    let db = app_state.db;

    // Get special routes
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

    // Check if the cookie exists
    if let Some(cookie) = jar.get(&cookie_name).cloned() {
        // Get the session token
        let session_token = cookie.value();

        // Get user from the session token
        let user = sessions.get_user_by_token(session_token).await;

        // Check if the session token is valid
        if sessions
            .validate_session_by_token(session_token, &conf)
            .await
            && user == username
        {
            return Err(Redirect::to(&format!(
                "{}/login?msg={}&status=success",
                &special_route,
                encode("You are already logged in. No need to log in again.",)
            )));
        } else {
            // Delete the session
            sessions.delete_session_by_token(session_token).await;
        }
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
