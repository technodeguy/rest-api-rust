#![allow(warnings)]
extern crate hyper;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate serde;
extern crate url;

use serde::{ Serialize, Deserialize };

use hyper::{Body, Request, Response, Server, rt, Method, StatusCode, Chunk, header};
use hyper::service::{service_fn};
use std::net::SocketAddr;
use std::io;
use futures::{Stream, Future, future::{self, FutureResult}};


type ResponseFuture = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

#[derive(Serialize, Deserialize, Debug,)]
enum ErrorCode {
    BAD_REQUEST,
    NOT_FOUND,
    INTERNAL
}

#[derive(Serialize, Deserialize, Debug,)]
struct Address {
    zip: u32,
    city: String
}

#[derive(Serialize, Deserialize, Debug,)]
struct NewBook {
    name: String,
    author: String,
    is_published: Option<bool>,
    address: Address
}

fn make_error_message(error_message: ErrorCode) -> String {
    json!({"errorCode": error_message}).to_string()
}

fn parse_form(form_chunk: Chunk) -> FutureResult<Result<NewBook, ErrorCode>, hyper::Error> {
    let json = String::from_utf8(form_chunk.to_vec()).unwrap();
    if let Ok(data) = serde_json::from_str::<NewBook>(&json) {
        info!("Parsed data: {:?}", data);
        future::ok(Ok(data))
    } else {
        error!("Failed to parse data");
        future::ok(Err(ErrorCode::BAD_REQUEST))
    }
}

fn make_post_response(result: Result<Result<NewBook, ErrorCode>, hyper::Error>) -> FutureResult<Response<Body>, hyper::Error> {
    if let Ok(data) = result {
        match data {
            Ok(newBook) => {
                info!("Preparing to succesfull server response");
                let response = Response::builder()
                    .header(header::CONTENT_TYPE, "application/json")
                    .status(StatusCode::CREATED)
                    .body(Body::from(serde_json::to_string(&newBook).unwrap()))
                    .unwrap();
                future::ok(response)
            }
            Err(errorCode) => {
                error!("Preparing to send bad_request server response");
                let response = Response::builder()
                    .header(header::CONTENT_TYPE, "application/json")
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from(make_error_message(errorCode)))
                    .unwrap();
                future::ok(response)
            }
        }
    } else {
        let response = Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(make_error_message(ErrorCode::INTERNAL)))
                .unwrap();
                       
        error!("Preparing to send internal server response");

        future::ok(response)
    }
}

// fn write_to_db(entry: NewMessage) -> FutureResult<i64, hyper::Error> {
//     futures::future::ok(0)
// }

fn hello_world(_req: Request<Body>) -> ResponseFuture {
    // unimplemented!()

    Box::new(future::ok(Response::new(Body::from("Fuck world!"))))
}

fn api_get_response(_req: Request<Body>) -> ResponseFuture {
   // let data = vec![NewBook{ name: String::from("Anna Karenina"), author: String::from("Tolstoi"), is_published: Some(true) }];
    let data = String::from("Default");
    let res = match serde_json::to_string(&data) {
        Ok(json) => {
            Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json))
                .unwrap()
        }
        Err(_) => {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal Server Error"))
                .unwrap()
        }
    };

    Box::new(future::ok(res))
}

fn create_book(req: Request<Body>) -> ResponseFuture {    
    let response = req.into_body()
        .concat2()
        .and_then(parse_form)
        .then(make_post_response);
        
    debug!("Preparing to send response");

    Box::new(response)
}

fn not_found_handler() -> ResponseFuture {
    let data = json!({ "errorCode": ErrorCode::NOT_FOUND }).to_string();
    let body = Body::from(data);
    Box::new(future::ok(
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(header::CONTENT_TYPE, "application/json")
            .body(body)
            .unwrap(),
    ))
}

fn router(req: Request<Body>) -> ResponseFuture {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => hello_world(req),
        (&Method::GET, "/api/v1/json_api") => api_get_response(req),
        (&Method::POST, "/api/v1/book") => create_book(req),
        _ => not_found_handler()
    }
}

fn main() {
    env_logger::init();
    
    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();

    let new_service = || service_fn(move |req| router(req));

    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(move |e| eprintln!("Server error: {}", e));

    println!("Listening on http://{}", addr);
        
    rt::run(server);
}