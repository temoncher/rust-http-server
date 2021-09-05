use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;

pub struct Server {
  addr: String,
}

impl Server {
  pub fn new(addr: String) -> Self {
    Self { addr }
  }

  pub fn listen(self) {
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
                Err(parsing_error) => {
                  println!("Failed to parse a request: {}", parsing_error);

                  Response::new(StatusCode::BadRequest, None)
                }
                Ok(_request) => Response::new(StatusCode::Ok, None),
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
