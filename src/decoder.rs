#![allow(warnings)]
use std::io::Read;
use std::io::Cursor;
use std::collections::HashMap;
use byteorder::{BigEndian, ReadBytesExt};
  
  struct meta {
    class: u8,
    structured: bool,
    tag: u8,
  }

  struct frog_tip {
    tip_id: u64,
    tip: String,
  }

  pub fn decode(mut buffer: &mut Vec<u8>) -> HashMap<u64, String> {
    let mut cache = HashMap::new();

    // Strip of and verify our first wrapper sequence 
    handle_sequence(&mut buffer);

    // for all remaining sequences
    while !buffer.is_empty() {
      handle_sequence(&mut buffer);
      let id = handle_int(&mut buffer);
      let t = handle_utf8_string(&mut buffer);
      cache.insert(id, t);
    }

    return cache;
  }

  // Nothing but the best for frog.tips clients, the following
  // are artisanal handcrafted der decoding functions 
  fn handle_meta(byte:u8) -> meta {
    let c = byte >> 6;
    let s = byte & 0x20 == 0x20; // false if primative
    let t = byte & 0x1f;
    return meta { class: c, structured: s, tag: t };
  }

  fn handle_sequence(buffer: &mut Vec<u8>) -> () {
    let m = handle_meta(buffer.remove(0));
    assert!(m.tag == 16, "not a SEQUENCE tag");
    let mut first_length_byte = buffer.remove(0);
    if first_length_byte > 128 {
      let mut length = first_length_byte - 128;
      while length > 0 {
        buffer.remove(0);
        length = length - 1;
      }
    } 
  }

  fn handle_int(buffer: &mut Vec<u8>) -> u64 () {
    let m = handle_meta(buffer.remove(0));
    assert!(m.tag == 2, "not an INTEGER tag");
    let mut first_length_byte = buffer.remove(0);
    let mut length = 0;
    let mut tip_id:u64 = 0;
    let mut int_buff:Vec<u8> = Vec::new();
    if first_length_byte < 128 {
      length = first_length_byte;
    } else {
      length = first_length_byte - 128;
    }

    while length > 0 {
      int_buff.push(buffer.remove(0));
      length = length - 1;
    }

    match int_buff.len() {
      1 => tip_id = int_buff.remove(0) as u64,
      2 => tip_id = Cursor::new(int_buff).read_u16::<BigEndian>().unwrap() as u64,
      4 => tip_id = Cursor::new(int_buff).read_u32::<BigEndian>().unwrap() as u64, 
      8 => tip_id = Cursor::new(int_buff).read_u64::<BigEndian>().unwrap(), 
      _ => panic!("illegal length"), 
    }

    return tip_id;
  }

  fn handle_utf8_string(buffer: &mut Vec<u8>) -> String {
    let m = handle_meta(buffer.remove(0));
    assert!(m.tag == 12, "not an utf8String tag");  
    let mut first_length_byte = buffer.remove(0);
    let mut length:u16 = 0;
    if first_length_byte < 128 {
      length = first_length_byte as u16;
    } else {
      let mut bytes_for_length = first_length_byte - 128;
      let mut int_buff = Vec::new();
      while bytes_for_length > 0 {
        int_buff.push(buffer.remove(0));
        bytes_for_length = bytes_for_length - 1;
      }

      if int_buff.len() > 1 {
        let mut rdr = Cursor::new(int_buff);
        length = rdr.read_u16::<BigEndian>().unwrap();
      } else {
        length = int_buff.remove(0) as u16;
      }
    }

    let mut string_buff = Vec::new();
    while length > 0 {
      string_buff.push(buffer.remove(0));
      length = length -1;
    }

    let s = String::from_utf8(string_buff).unwrap();   
    return s;
  }

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs::File;
  use std::io::Read;

  #[test]
  fn it_works() {
    let mut f = File::open("./frog.data").unwrap();
    let mut buffer = Vec::new();
    let bytes_read = f.read_to_end(&mut buffer);
    println!("bytes read {:?}", bytes_read);
    let cache = super::decode(&mut buffer);
    assert_eq!(cache.len(), 50);
    let x = cache.get(&27);
    assert_eq!(x.unwrap(), "DO NOT USE FROG IN PLACE OF PRESSURE COOKER.");
  }
}
