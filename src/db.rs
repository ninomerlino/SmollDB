/*
MIT License

Copyright (c) 2022 ninomerlino

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE. 
*/
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{Read, Write},
    path::{Path},
};
use yazi::{compress, decompress, CompressionLevel, Format};
use crate::util::*;

#[derive(Debug,PartialEq)]

///Object to represent the in memory database
pub struct SmollDB {
    inner: HashMap<String, DataType>,
}

impl Default for SmollDB {
    ///Create an empty database
    /// # Example
    /// ```
    ///let mut database = SmollDB::default();
    ///let data = String::from("data");
    ///database.set("example",data.clone());
    ///match database.get("example"){
    ///    Some(result) => {
    ///        assert_eq!(*result, DataType::STRING(data))
    ///    },
    ///    None => todo!(),
    ///};
    /// 
    /// ```
    fn default() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
}

impl SmollDB {
    fn save_file(&self, filename: impl AsRef<Path>, data: &Vec<u8>) -> Result<(), SmollError> {
        let mut filename = filename.as_ref().to_owned();
        filename.set_extension("smoll");

        match File::create(filename) {
            Ok(mut file) => {
                file.write(&data).map_err(|_| SmollError::SaveFileError)?;
                Ok(())
            }
            Err(_) => Err(SmollError::SaveFileError),
        }
    }

    fn read_file(filename: impl AsRef<Path>) -> Result<Vec<u8>, SmollError> {
        let mut filename = filename.as_ref().to_owned();
        filename.set_extension("smoll");

        match File::open(filename) {
            Ok(mut file) => {
                let mut buffer: Vec<u8> = Vec::new();
                file.read_to_end(&mut buffer)
                    .map_err(|_| SmollError::ReadFileError)?;
                Ok(buffer)
            }
            Err(_) => Err(SmollError::ReadFileError),
        }
    }

    fn encode(&self) -> Vec<u8> {
        let mut encoded_data = Vec::<u8>::new();
        for (key, value) in self.inner.iter() {
            encoded_data.extend(key.as_bytes());
            encoded_data.push(0); //make string null terminated
            encoded_data.push(value.id());
            match value {
                DataType::BOOL(value) => encoded_data.push(*value as u8),
                DataType::INT8(value) => encoded_data.extend(value.to_be_bytes()),
                DataType::INT16(value) => encoded_data.extend(value.to_be_bytes()),
                DataType::INT32(value) => encoded_data.extend(value.to_be_bytes()),
                DataType::INT64(value) => encoded_data.extend(value.to_be_bytes()),
                DataType::FLOAT32(value) => encoded_data.extend(value.to_be_bytes()),
                DataType::FLOAT64(value) => encoded_data.extend(value.to_be_bytes()),
                DataType::STRING(value) => {
                    encoded_data.extend(value.as_bytes());
                    encoded_data.push(0); //null terminate str
                }
                DataType::BYTES(value) => {
                    encoded_data.extend(value.len().to_be_bytes());
                    encoded_data.extend(value);
                }
            }
        }
        encoded_data
    }

    fn decode(mut encoded_data: VecDeque<u8>) -> Result<HashMap<String, DataType>, SmollError> {
        let mut db_hashmap = HashMap::new();
        let mut key = String::default();
        while !encoded_data.is_empty() {
            key.clear();
            loop {
                if let Some(byte) = encoded_data.pop_front() {
                    if byte == 0 {
                        break;
                    }
                    key.push(byte as char);
                } else {
                    return Err(SmollError::DecodeError);
                }
            }
            match encoded_data.pop_front().ok_or(SmollError::DecodeError)? {
                0 => {
                    let data = encoded_data.pop_front().ok_or(SmollError::DecodeError)? != 0;
                    db_hashmap.insert(key.clone(), DataType::BOOL(data));
                }
                1 => {
                    let data = i8::from_be_bytes([encoded_data
                        .pop_front()
                        .ok_or(SmollError::DecodeError)?]);
                    db_hashmap.insert(key.clone(), DataType::INT8(data));
                }
                2 => {
                    let data = i16::from_be_bytes(
                        encoded_data
                            .drain(0..2)
                            .collect::<Vec<u8>>()
                            .try_into()
                            .map_err(|_| SmollError::DecodeError)?,
                    );
                    db_hashmap.insert(key.clone(), DataType::INT16(data));
                }
                3 => {
                    let data = i32::from_be_bytes(
                        encoded_data
                            .drain(0..4)
                            .collect::<Vec<u8>>()
                            .try_into()
                            .map_err(|_| SmollError::DecodeError)?,
                    );
                    db_hashmap.insert(key.clone(), DataType::INT32(data));
                }
                4 => {
                    let data = i64::from_be_bytes(
                        encoded_data
                            .drain(0..8)
                            .collect::<Vec<u8>>()
                            .try_into()
                            .map_err(|_| SmollError::DecodeError)?,
                    );
                    db_hashmap.insert(key.clone(), DataType::INT64(data));
                }
                5 => {
                    let data = f32::from_be_bytes(
                        encoded_data
                            .drain(0..4)
                            .collect::<Vec<u8>>()
                            .try_into()
                            .map_err(|_| SmollError::DecodeError)?,
                    );
                    db_hashmap.insert(key.clone(), DataType::FLOAT32(data));
                }
                6 => {
                    let data = f64::from_be_bytes(
                        encoded_data
                            .drain(0..8)
                            .collect::<Vec<u8>>()
                            .try_into()
                            .map_err(|_| SmollError::DecodeError)?,
                    );
                    db_hashmap.insert(key.clone(), DataType::FLOAT64(data));
                }
                7 => {
                    let mut data = String::default();
                    loop {
                        if let Some(byte) = encoded_data.pop_front() {
                            if byte == 0 {
                                break;
                            }
                            data.push(byte as char);
                        } else {
                            return Err(SmollError::DecodeError);
                        }
                    }
                    db_hashmap.insert(key.clone(), DataType::STRING(data));
                }
                8 => {
                    let size = usize::from_be_bytes(
                        encoded_data
                            .drain(0..8)
                            .collect::<Vec<u8>>()
                            .try_into()
                            .map_err(|_| SmollError::DecodeError)?,
                    );
                    let data = encoded_data
                        .drain(0..size)
                        .collect::<Vec<u8>>();
                    db_hashmap.insert(key.clone(), DataType::BYTES(data));
                }
                _ => {
                    return Err(SmollError::DecodeError);
                }
            }
        }
        Ok(db_hashmap)
    }
    ///Load database from file, path doesn't need the .smoll extention
    /// # Example
    /// ```
    ///let mut database = SmollDB::default();
    ///let data = String::from("data");
    ///database.set("example",data.clone());
    ///database.backup("myfile").unwrap();
    ///let database = SmollDB::load("mydatabase");
    ///let data = String::from("data");
    ///match database.get("example"){
    ///    Some(result) => {
    ///        assert_eq!(*result, DataType::STRING(data))
    ///    },
    ///    None => todo!(),
    ///};
    /// ```
    pub fn load(path: impl AsRef<Path>) -> Result<Self, SmollError> {
        let encoded_data = Self::read_file(path)?;
        let (encoded_data, _) = decompress(&encoded_data, Format::Zlib).map_err(|_|{SmollError::DecompressionError})?;
        let data = Self::decode(encoded_data.into())?;
        Ok(Self{inner:data})
    }
    ///Backup database on file, path doesn't need the .smoll extention
    /// # Example
    /// ```
    ///let mut database = SmollDB::default();
    ///let data = String::from("data");
    ///database.set("example",data.clone());
    ///database.backup("myfile").unwrap();
    ///let database = SmollDB::load("mydatabase");
    ///let data = String::from("data");
    ///match database.get("example"){
    ///    Some(result) => {
    ///        assert_eq!(*result, DataType::STRING(data))
    ///    },
    ///    None => todo!(),
    ///};
    /// ```
    pub fn backup(&self, file: impl AsRef<Path>) -> Result<(), SmollError> {
        let data = self.encode();
        let data = compress(&data, Format::Zlib, CompressionLevel::BestSpeed)
            .map_err(|_| SmollError::CompressionError)?;
        self.save_file(file, &data)
    }
    ///Save `value` in the database with the specified `key`
    /// # Example
    /// ```
    ///let mut database = SmollDB::default();
    ///let data = String::from("data");
    ///database.set("example",data.clone());
    ///match database.get("example"){
    ///    Some(result) => {
    ///        assert_eq!(*result, DataType::STRING(data))
    ///    },
    ///    None => todo!(),
    ///};
    /// 
    /// ```
    pub fn set(&mut self, key: impl ToString, value: impl Into<DataType>) -> Option<DataType> {
        self.inner.insert(key.to_string(), value.into())
    }
    ///Get `value` with the specified `key`, return a empty optional if `key` is not contained in the database
    /// # Example
    /// ```
    ///let mut database = SmollDB::default();
    ///let data = String::from("data");
    ///database.set("example",data.clone());
    ///match database.get("example"){
    ///    Some(result) => {
    ///        assert_eq!(*result, DataType::STRING(data))
    ///    },
    ///    None => todo!(),
    ///};
    /// 
    /// ```
    pub fn get(&self, key: impl ToString) -> Option<&DataType> {
        self.inner.get(&key.to_string())
    }
}
