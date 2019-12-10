use hyper::{StatusCode};
use futures::future;

use crate::consts::ErrorCode;
use crate::types::response::ResponseFuture;
use crate::utils::response::{create_response, create_error_message};

pub fn not_found_handler() -> ResponseFuture {
    Box::new(future::ok(create_response(StatusCode::NOT_FOUND, create_error_message(ErrorCode::NOT_FOUND))))
}