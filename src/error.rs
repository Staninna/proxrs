use crate::config::{ConfigKey::*, ConfigStore};
use hyper::{Body, Response, StatusCode};
use tera::{Context, Tera};

pub async fn internal_error(
    conf: &ConfigStore,
    tera: Tera,
) -> Result<Response<Body>, hyper::Error> {
    // Get the internal error page path from the config
    let internal_error_page_path = conf.get(InternalErrorTemplate).await;

    // Create context for the internal error page
    let mut context = Context::new();

    // Using rand choose and random internet activity
    let activities = vec![
        (
            "https://twitter.com/",
            "Scroll through endless tweets and procrastinate on real work",
        ),
        (
            "https://youtube.com/watch?v=dQw4w9WgXcQ",
            "Experience the ultimate 80's nostalgia and get Rickrolled",
        ),
        (
            "https://www.reddit.com/r/funny/",
            "Get your daily dose of internet humor and memes",
        ),
        (
            "https://www.boredpanda.com/",
            "Find hilarious and ridiculous stories from around the world",
        ),
        (
            "https://www.netflix.com/",
            "Waste countless hours watching funny cat videos on Netflix",
        ),
        (
            "https://www.twitch.tv/",
            "Watch gamers rage quit and fail miserably at video games",
        ),
        (
            "https://www.goodreads.com/",
            "Discover funny and quirky books to add to your reading list",
        ),
        (
            "https://www.pinterest.com/",
            "Laugh at Pinterest fails and DIY disasters",
        ),
        (
            "https://www.spotify.com/",
            "Listen to comedy podcasts and humorous songs",
        ),
        (
            "https://www.etsy.com/",
            "Shop for ridiculous and hilarious gifts for your friends and family",
        ),
    ];
    let activity = activities[rand::random::<usize>() % activities.len()];
    context.insert("activity_url", &activity.0);
    context.insert("activity_description", &activity.1);

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
