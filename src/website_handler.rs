use super::http::{Request, Response, StatusCode};
use super::server::Handler;

pub struct WebpageHandler;

impl Handler for WebpageHandler {
  fn handle_request(&mut self, _request: &Request) -> Response {
    Response::new(StatusCode::Ok, Some("This website works!".to_string()))
  }
}