#![allow(warnings)]
#![deny(unused_imports)]
extern crate hyper;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde;
extern crate url;

mod consts;
mod routes;
mod utils;
mod types;

use hyper::{service::{service_fn}, Server, Request, Body, Method, rt};
use std::net::SocketAddr;
use futures::future::*;

use routes::{book, general};
use types::response::ResponseFuture;

fn create_router(req: Request<Body>) -> ResponseFuture {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/api/v1/books") => book::get_all_books(req),
        (&Method::POST, "/api/v1/book") => book::create_book(req),
        _ => general::not_found_handler()
    }
}

fn main() {
    env_logger::init();
    
    let addr = "127.0.0.1:3000".parse::<SocketAddr>().unwrap();

    let new_service = || service_fn(move |req| create_router(req));

    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(move |e| eprintln!("Server error: {}", e));

    println!("Listening on http://{}", addr);
        
    rt::run(server);
}