use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use std::io::prelude::*;
use std::error::Error;

pub fn compress(data:&str)-> Result<Vec<u8>,Box<dyn Error>>{
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    e.write_all(data.as_bytes())?;
    Ok(e.finish()?)
}

pub fn decompress(data:Vec<u8>)-> Result<String,Box<dyn Error>>{
    let data_slice: &[u8] = &data;
    let mut d = GzDecoder::new(data_slice);
    let mut s = String::new();
    d.read_to_string(&mut s)?;
    Ok(s)
}