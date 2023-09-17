use std::fs;
use std::error::Error;

mod encrypt;
mod compress;
use encrypt::{encrypt,decrypt};
use compress::{compress,decompress};


pub fn write(file_name: &str, data: Vec<u8>, key: &str) ->Result<(), Box<dyn Error>>{
    let data = compress(data)?;
    let data = encrypt(data,key)?;
    fs::write(file_name, &data)?;
    Ok(())
}

pub fn read(file_name: &str, key: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let data = fs::read(file_name)?;
    let data = decrypt(data,key)?;
    let data = decompress(data)?;
    Ok(data)
}


