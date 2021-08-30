use super::method::{Method, MethodError};
use super::RequestQuery;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;

#[derive(Debug)]
pub struct Request<'buf> {
  path: &'buf str,
  query_string: Option<RequestQuery<'buf>>,
  method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
  type Error = ParseError;

  fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
    let request_string = str::from_utf8(buf)?;

    let (method_string, request_without_method) =
      get_next_word(request_string).ok_or(ParseError::InvalidRequest)?;
    let (path_string, request_without_path) =
      get_next_word(request_without_method).ok_or(ParseError::InvalidRequest)?;
    let (protocol_string, _) =
      get_next_word(request_without_path).ok_or(ParseError::InvalidRequest)?;

    if protocol_string != "HTTP/1.1" {
      return Err(ParseError::InvalidProtocol);
    }

    let method: Method = method_string.parse()?;

    // using string slice here is safe because question mark symbol always takes up 1 byte
    let (path, query_string) = match path_string.find('?') {
      None => (path_string, None),
      Some(i) => (
        &path_string[..i],
        Some(RequestQuery::from(&path_string[i + 1..])),
      ),
    };

    Ok(Self {
      path,
      query_string,
      method,
    })
  }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
  request
    .chars()
    .enumerate()
    .find(|(_i, c)| *c == ' ' || *c == '\r')
    .map(|(i, _)| (&request[..i], &request[i + 1..]))
}

pub enum ParseError {
  InvalidRequest,
  InvalidEncoding,
  InvalidProtocol,
  InvalidMethod,
}

impl ParseError {
  fn message(&self) -> &str {
    match self {
      Self::InvalidRequest => "Invalid Request",
      Self::InvalidEncoding => "Invalid Encoding",
      Self::InvalidProtocol => "Invalid Protocol",
      Self::InvalidMethod => "Invalid Method",
    }
  }
}

impl Display for ParseError {
  fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
    write!(formatter, "{}", self.message())
  }
}

impl Debug for ParseError {
  fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
    write!(formatter, "{}", self.message())
  }
}

impl From<MethodError> for ParseError {
  fn from(_method_error: MethodError) -> Self {
    Self::InvalidMethod
  }
}

impl From<str::Utf8Error> for ParseError {
  fn from(_utf8_error: str::Utf8Error) -> Self {
    Self::InvalidEncoding
  }
}

impl Error for ParseError {}
