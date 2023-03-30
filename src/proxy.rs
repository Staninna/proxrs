use crate::session::SessionStore;
use hyper::{Body, Request, Response};

pub async fn proxy_handler(
    _req: Request<Body>,
    _store: SessionStore,
) -> Result<Response<Body>, hyper::Error> {
    todo!()
}
