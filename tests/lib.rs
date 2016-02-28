extern crate rust_RIBBIT;
use rust_RIBBIT::client as client;

#[test]
fn it_works() {
  let mut frog = client::RIBBITClient::new();
  let tip = frog.frog_tip();
  assert!(tip.len() > 0 && tip.contains("FROG"));
}
