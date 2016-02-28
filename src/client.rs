#![allow(warnings)]
use std::io::Read;
use hyper::header::Headers;
use hyper::Client;
use std::collections::HashMap;
use decoder;

pub struct frog_tip {
  tip_id: u64,
  tip: String,
}

pub struct RIBBITClient {
  url: String,
  headers: Headers,
  client: Client,
  cache: Option<HashMap<u64, String>>,
}

impl RIBBITClient {
  pub fn new() -> RIBBITClient {
    let mut headers = Headers::new();
    headers.set_raw("Accept", vec![b"application/der-stream".to_vec()]);
    let client = Client::new();
    RIBBITClient {
      url: String::from("http://frog.tips/api/1/tips"),
      headers: headers,
      client: client,
      cache: None,
    }
  }

  pub fn croak(&mut self) -> () {
    let mut headers = Headers::new();
    headers.set_raw("Accept", vec![b"application/der-stream".to_vec()]);
    let client = super::hyper::Client::new();
    let mut res = client.get(&self.url).headers(headers).send().unwrap();
    let mut buffer = Vec::new();
    let bytes_read = res.read_to_end(&mut buffer);
    debug!("{:?} bytes read from {}", bytes_read, self.url);
    self.cache = Some(decoder::decode(&mut buffer));
  }

  pub fn frog_tip(&mut self) -> &String {
    if self.cache.is_none() {
      self.croak();
    }

    let mut say = None;
    match self.cache.as_mut() {
      Some(v) => say = v.values().next(),
      None => panic!("it appears frog has croacked its last"),
    }

    return say.unwrap();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let mut frog = super::RIBBITClient::new();
    let tip = frog.frog_tip();
    assert!(tip.len() > 0 && tip.contains("FROG")); 
  }
}
