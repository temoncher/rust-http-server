use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
  fn handle_request(&mut self, request: &Request) -> Response;
  fn handle_bad_request(&mut self, parse_error: &ParseError) -> Response {
    println!("Failed to parse request: {}", parse_error);
    Response::new(StatusCode::BadRequest, None)
  }
}

pub struct Server {
  addr: String,
}

impl Server {
  pub fn new(addr: String) -> Self {
    Self { addr }
  }

  pub fn listen(self, mut handler: impl Handler) {
    println!("Listening on {}", self.addr);

    let listener = TcpListener::bind(&self.addr).unwrap();

    loop {
      match listener.accept() {
        Err(connection_error) => {
          println!("Failed to establish a connection: {}", connection_error);
          continue;
        }
        Ok((mut stream, _)) => {
          let mut buffer = [0; 1024];
          match stream.read(&mut buffer) {
            Err(reading_error) => println!("Failed to read from connection: {}", reading_error),
            Ok(_buffer_size) => {
              println!("Received a request: {}", String::from_utf8_lossy(&buffer));

              let response = match Request::try_from(&buffer[..]) {
                Err(parsing_error) => handler.handle_bad_request(&parsing_error),
                Ok(request) => handler.handle_request(&request),
              };

              if let Err(send_error) = response.send(&mut stream) {
                println!("Failed to send a response: {}", send_error);
              }
            }
          };
        }
      }
    }
  }
}
