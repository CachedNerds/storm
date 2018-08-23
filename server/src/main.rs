extern crate diesel;
extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate storm_server;

use diesel::pg::PgConnection;
use futures::{future, Stream};
use hyper::rt::{self, Future};
use hyper::service::service_fn;
use hyper::Chunk;
use hyper::{Body, Method, Request, Response, Server, StatusCode};

use serde::de;

fn deserialize<T>(body: Vec<u8>) -> serde_json::Result<T>
where
    for<'de> T: de::Deserialize<'de>,
{
    serde_json::from_slice(&body)
}

// Just a simple type alias
type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn service(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        }
//        (&Method::POST, "/users") => {
//            let (parts, body) = req.into_parts();
//            body.concat2().map(move |chunk| {
//                let body = chunk.iter().cloned().collect::<Vec<u8>>();
//                match deserialize(body) {
//                    Err(_) => {}
//                    Ok(json) => {
//
//                    }
//                }
//            });
//        }
        (&Method::GET, "/users") => {}
        (&Method::PUT, "/users") => {}
        (&Method::DELETE, "/users") => {}
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Box::new(future::ok(response))
}

fn main() {
    // This is our socket address...
    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn(service))
        .map_err(|e| println!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}
