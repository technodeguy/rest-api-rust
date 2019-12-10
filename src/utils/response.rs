use hyper::{Body, Response, StatusCode, header};
use std::convert::From;

use crate::consts::ErrorCode;

pub fn create_error_message(error_message: ErrorCode) -> String {
    json!({"errorCode": error_message}).to_string()
}

pub fn create_response<T>(status_code: StatusCode, body: T) -> Response<Body>
  where Body: From<T>,
{
  Response::builder()
    .header(header::CONTENT_TYPE, "application/json")
    .status(status_code)
    .body(Body::from(body))
    .unwrap()
}