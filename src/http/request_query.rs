use std::collections::HashMap;

#[derive(Debug)]
pub struct RequestQuery<'buf>(HashMap<&'buf str, QueryValue<'buf>>);

impl<'buf> RequestQuery<'buf> {
  pub fn get(&self, key: &str) -> Option<&QueryValue> {
    let RequestQuery(map) = self;

    map.get(key)
  }
}

impl<'buf> From<&'buf str> for RequestQuery<'buf> {
  fn from(s: &'buf str) -> Self {
    let mut hash_data = HashMap::new();

    for sub_str in s.split('&') {
      let (key, value) = match sub_str.find('=') {
        None => (sub_str, ""),
        Some(i) => (&sub_str[..i], &sub_str[i + 1..]),
      };

      hash_data
        .entry(key)
        .and_modify(|existing| match existing {
          QueryValue::Single(prev_query_value) => {
            *existing = QueryValue::Multiple(vec![prev_query_value, value])
          }
          QueryValue::Multiple(vec) => vec.push(value),
        })
        .or_insert(QueryValue::Single(value));
    }

    Self(hash_data)
  }
}

#[derive(Debug)]
pub enum QueryValue<'buf> {
  Single(&'buf str),
  Multiple(Vec<&'buf str>),
}
