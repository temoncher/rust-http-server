use super::status_code::StatusCode;

#[derive(Debug)]
pub struct Respone {
  status_code: StatusCode,
  body: Option<String>,
}

impl Respone {
  pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
    Self { status_code, body }
  }
}
