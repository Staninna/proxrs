use hashbrown::HashMap;
use hyper::{Body, Request, Response};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn proxy_handler(
    _req: Request<Body>,
    _sessions: Arc<Mutex<HashMap<String, String>>>,
) -> Result<Response<Body>, hyper::Error> {
    todo!()
}
