use hyper::{Chunk, Error};
use futures::{future::{self, FutureResult}};
use serde::Deserialize;

use crate::consts::ErrorCode;

pub fn parse_form<'de, T>(form_chunk: &'de Chunk) -> FutureResult<Result<T, ErrorCode>, Error>
  where T: Deserialize<'de> + std::fmt::Debug,
{
    let parsedData = if let Ok(data) = serde_json::from_slice(form_chunk.as_ref()) {
        info!("Parsed data: {:?}", data);
        Ok(data)
    } else {
        error!("Failed to parse data");
        Err(ErrorCode::BAD_REQUEST)
    };

    future::ok(parsedData)
}