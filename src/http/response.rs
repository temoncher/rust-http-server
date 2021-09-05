use super::status_code::StatusCode;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::{Result as IoResult, Write};

#[derive(Debug)]
pub struct Response {
  status_code: StatusCode,
  body: Option<String>,
}

impl Response {
  pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
    Response { status_code, body }
  }

  pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
    let response_body = match &self.body {
      None => "",
      Some(body) => body,
    };

    write!(
      stream,
      "HTTP/1.1 {} {}\r\n\r\n{}",
      self.status_code,
      self.status_code.reason_phrase(),
      response_body
    )
  }
}

impl Display for Response {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    let response_body = match &self.body {
      None => "",
      Some(body) => body,
    };

    write!(
      f,
      "HTTP/1.1 {} {}\r\n\r\n{}",
      self.status_code,
      self.status_code.reason_phrase(),
      response_body
    )
  }
}
