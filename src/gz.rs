use flate2::read::GzDecoder;
use std::io::prelude::*;


pub fn read_gz(path:&str) -> Option<String>{

    let bytes = std::fs::read(path);
    let bytes = match bytes{
        Ok(_) => bytes.unwrap(),
        Err(_) => return None,
    };
    let mut decoder = GzDecoder::new(&bytes[..]);
    let mut ret_str = String::new();
    decoder.read_to_string(&mut ret_str).unwrap();

    Some(ret_str)
}


