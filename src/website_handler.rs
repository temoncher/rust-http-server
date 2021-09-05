use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
use std::fs;

pub struct WebsiteHandler {
  public_path: String,
}

impl WebsiteHandler {
  pub fn new(public_path: String) -> Self {
    Self { public_path }
  }

  fn read_file(&self, file_path: &str) -> Option<String> {
    let full_file_path = format!("{}/{}", self.public_path, file_path);
    fs::read_to_string(full_file_path).ok()
  }
}

impl Handler for WebsiteHandler {
  fn handle_request(&mut self, request: &Request) -> Response {
    match request.method() {
      Method::GET => match request.path() {
        "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
        "/hello" => Response::new(
          StatusCode::Ok,
          Some("<h1>Hello, stranger!</h1>".to_string()),
        ),
        _ => Response::new(StatusCode::NotFound, None),
      },
      _ => Response::new(StatusCode::MethodNotAllowed, None),
    }
  }
}
