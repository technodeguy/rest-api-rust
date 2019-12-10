use hyper::{Body, Request, StatusCode, Chunk};
use futures::{future, Future, Stream};

use crate::types::response::*;
use crate::utils::{response::{create_response, create_error_message}, validator::parse_form};
use crate::consts::ErrorCode;

#[derive(Serialize, Deserialize, Debug,)]
struct NewBook {
    name: String,
    author: String,
    is_published: Option<bool>,
}

pub fn get_all_books(req: Request<Body>) -> ResponseFuture {
    let books = vec!["Anna Karenina", "Kobsar"];

    let response = if let Ok(json) = serde_json::to_string(&books) {
        create_response(StatusCode::OK, json)
    } else {
        create_response(StatusCode::INTERNAL_SERVER_ERROR, create_error_message(ErrorCode::INTERNAL))
    };

    Box::new(future::ok(response))
}

// impl std::convert::From<NewBook> for Body {
//     fn from(res: NewBook) -> Body {
//         Body::from(res.)
//     }
// }

pub fn create_book(req: Request<Body>) -> ResponseFuture {    
    let response = req.into_body()
        .concat2()
        .and_then(move |chunk: Chunk| parse_form::<NewBook>(&chunk))
        .and_then(move |result| {
            let response = match result {
                Ok(data) => {
                    info!("Preparing to succesfull server response");
                    // create_response::<NewBook>(StatusCode::CREATED, data)
                    unimplemented!()
                }
                Err(error) => {
                    error!("Preparing to send bad_request server response");
                    create_response::<String>(StatusCode::BAD_REQUEST, create_error_message(ErrorCode::BAD_REQUEST))
                }
            };
            
            Ok(response)
        });
    
   Box::new(response)
}