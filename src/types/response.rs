use hyper::{Error, Body, Response};
use futures::{Future};

pub type ResponseFuture = Box<dyn Future<Item = Response<Body>, Error = Error> + Send>;

pub type PostResponse<T> = Result<Result<T, String>, Error>;
