use serde::{Deserialize, Serialize};
use serde_encrypt::{
    serialize::impls::BincodeSerializer, shared_key::SharedKey, traits::SerdeEncryptSharedKey,
    AsSharedKey, EncryptedMessage,
};
use sha2::{Sha256, Digest};
use std::error::Error;


#[derive(Serialize, Deserialize, Debug)]
struct MyData {
    data: Vec<u8>,
}

impl SerdeEncryptSharedKey for MyData {
    type S = BincodeSerializer<Self>;
}

pub fn encrypt(data:Vec<u8>,key:&str)-> Result<Vec<u8>,Box<dyn Error>> {
    let key: SharedKey = {
        let mut hasher: Sha256 = Sha256::new();
        hasher.update(key.as_bytes());
        let result = hasher.finalize();
        let result = result.as_slice().try_into()?;
        SharedKey::from_array(result)
    };

    let my_data = MyData {
        data: data,
    };

    let data = my_data.encrypt(&key)?;
    Ok(data.serialize())
}

pub fn decrypt(data:Vec<u8>,key:&str)-> Result<Vec<u8>,Box<dyn Error>> {
    let key: SharedKey = {
        let mut hasher: Sha256 = Sha256::new();
        hasher.update(key.as_bytes());
        let result = hasher.finalize();
        let result = result.as_slice().try_into().unwrap();
        SharedKey::from_array(result)
    };

    let data = EncryptedMessage::deserialize(data)?;
    let my_data = MyData::decrypt_owned(&data, &key)?;
    Ok(my_data.data)
}