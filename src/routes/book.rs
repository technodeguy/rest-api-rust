use diesel::mysql::MysqlConnection;
use hyper::{Body, Request, StatusCode, Chunk};
use futures::{future, Future, Stream};

use crate::types::response::*;
use crate::utils::{response::{create_response, create_error_message}, validator::parse_form};
use crate::consts::ErrorCode;
use crate::models::book::{Book, NewBook};
use crate::dto::{IdDto, NewBookDto};

pub fn get_book_by_id(req: Request<Body>, db_conn: &MysqlConnection) -> ResponseFuture {
    let books = vec!["Anna Karenina", "Kobsar"];

    let response = req.into_body()
        .concat2()
        .and_then(move |chunk: Chunk| parse_form::<IdDto>(&chunk))
        .and_then(move |result| {
            let response = match result {
                Ok(data) => {
                    info!("Preparing to succesfull server response, {:#?}", data);
                    // create_response(StatusCode::CREATED, books.get(0).unwrap())
                    unimplemented!()
                }
                Err(error) => {
                    error!("Preparing to send bad_request server response");
                    create_response::<String>(StatusCode::BAD_REQUEST, create_error_message(error))
                }
            };
            
            Ok(response)
        });
    
    Box::new(response)
}


pub fn get_all_books(req: Request<Body>, db_conn: &MysqlConnection) -> ResponseFuture {
    let books = Book::find_all(db_conn);

    let response = if let Ok(json) = serde_json::to_string(&books) {
        create_response(StatusCode::OK, json)
    } else {
        create_response(StatusCode::INTERNAL_SERVER_ERROR, create_error_message(ErrorCode::INTERNAL))
    };

    Box::new(future::ok(response))
}

pub fn create_book(req: Request<Body>, db_conn: &MysqlConnection) -> ResponseFuture {    
    let response = req.into_body()
        .concat2()
        .and_then(move |chunk: Chunk| parse_form::<NewBookDto>(&chunk))
        .and_then(move |result| {
            let response = match result {
                Ok(data) => {
                    let book = Book::insert(&NewBook::from(data) , db_conn);
                    info!("Book inserted, {:#?}", book);

                    create_response(StatusCode::CREATED, vec!(book as u8))
                }
                Err(error) => {
                    error!("Preparing to send bad_request server response");
                    create_response::<String>(StatusCode::BAD_REQUEST, create_error_message(error))
                }
            };
            
            Ok(response)
        });
    
  Box::new(response)
}
