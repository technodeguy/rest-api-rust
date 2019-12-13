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
#[macro_use]
extern crate diesel;

mod schema;
mod consts;
mod models;
mod routes;
mod utils;
mod types;
mod db;
mod dto;

use hyper::{service::{service_fn}, Server, Request, Body, Method, rt};
use std::net::SocketAddr;
use futures::future::*;

use routes::{book, general};
use types::response::ResponseFuture;
use db::create_db_connection;

fn create_router(req: Request<Body>) -> ResponseFuture {
    let db_conn = create_db_connection();

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/api/v1/books") => book::get_all_books(req, &db_conn),
        (&Method::POST, "/api/v1/book/id") => book::get_book_by_id(req, &db_conn),
        (&Method::POST, "/api/v1/book") => book::create_book(req, &db_conn),
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