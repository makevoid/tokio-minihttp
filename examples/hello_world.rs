extern crate tokio_proto;
extern crate tokio_service;
extern crate tokio_minihttp as http;
extern crate futures;
extern crate env_logger;

use tokio_service::Service;
use futures::{Async, Finished};
use std::io;

#[derive(Clone)]
struct HelloWorld;

impl Service for HelloWorld {
    type Request = http::Request;
    type Response = http::Response;
    type Error = io::Error;
    type Future = Finished<http::Response, io::Error>;

    fn call(&self, _request: http::Request) -> Self::Future {
        let resp = http::Response::new();
        futures::finished(resp)
    }

    fn poll_ready(&self) -> Async<()> {
        Async::Ready(())
    }
}

pub fn main() {
    let _ = ::env_logger::init();

    let addr = "0.0.0.0:8080".parse().unwrap();

    http::serve(addr, HelloWorld);
}
