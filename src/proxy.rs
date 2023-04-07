// Import the necessary libraries and modules
use futures::future::BoxFuture;
use hyper::{Body, Client, Request, Response};
use hyper_tls::HttpsConnector;
use std::task::{Context, Poll};
use tower::Service;

// Define a struct to serve as the HTTP service
#[derive(Clone, Copy)]
pub struct Proxy;

// Implement the Service trait for the HelloWorld struct
impl Service<Request<Body>> for Proxy {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        // Get the request method and headers
        let method = req.method().to_owned();
        let headers = req.headers().to_owned();

        // Create uri
        let uri = format!("https://noelp.dev{}", req.uri());

        // Create a new request
        let mut new_req = Request::new(req.into_body());
        *new_req.uri_mut() = uri.parse().unwrap();
        *new_req.method_mut() = method;
        *new_req.headers_mut() = headers;

        // Make the Host header match the new uri
        // IDK:  if this is necessary when using this for local network requests but it is for the requests to external websites
        let host = uri.replace("https://", "").replace("http://", "");
        let host = host.split('/').next().unwrap();
        new_req.headers_mut().insert("Host", host.parse().unwrap());

        // Create a new client
        let https = HttpsConnector::new();
        let res = Client::builder()
            .build::<_, hyper::Body>(https)
            .request(new_req);

        Box::pin(async move {
            // Await the response
            let res = res.await;

            // Print the response
            let res = match res {
                Ok(res) => {
                    dbg!(&res); // DEBUG: Print the response
                    res
                }
                Err(err) => {
                    dbg!(&err); // DEBUG: Print the error
                    return Err(err);
                }
            };

            // Return the response
            Ok(res)
        })
    }
}
