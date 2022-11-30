use crate::*;
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{Read, Write},
    mem::size_of,
    path::Path,
};
use yazi::{compress, decompress, CompressionLevel, Format};

macro_rules! from_be_bytes {
    ($type_name:ty, $data_buffer:ident) => {
        <$type_name>::from_be_bytes(
            $data_buffer
                .drain(0..size_of::<$type_name>())
                .collect::<Vec<u8>>()
                .try_into()
                .map_err(|_| SmollError::DecodeError)?,
        )
    };
}

///Object to represent the in memory database
#[derive(Debug, PartialEq, Default)]
pub struct SmollDB {
    inner: HashMap<String, DataType>,
}

impl SmollDB {
    fn save_file(&self, filename: impl AsRef<Path>, data: &[u8]) -> SmollResult<()> {
        let mut filename = filename.as_ref().to_owned();
        filename.set_extension("smoll");
        File::create(filename)?.write_all(data)?;
        Ok(())
    }

    fn read_file(filename: impl AsRef<Path>) -> SmollResult<Vec<u8>> {
        let mut filename = filename.as_ref().to_owned();
        filename.set_extension("smoll");
        let mut buffer: Vec<u8> = Vec::new();
        File::open(filename)?.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    fn encode(&self) -> Vec<u8> {
        let mut encoded_data = Vec::<u8>::new();
        for (key, value) in self.inner.iter() {
            encoded_data.extend(key.len().to_be_bytes());
            encoded_data.extend(key.as_bytes());
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
                    encoded_data.extend(dbg!(value.len().to_be_bytes()));
                    encoded_data.extend(dbg!(value.as_bytes()));
                }
                DataType::BYTES(value) => {
                    encoded_data.extend(value.len().to_be_bytes());
                    encoded_data.extend(value);
                }
            }
        }
        encoded_data
    }

    fn decode(mut encoded_data: VecDeque<u8>) -> SmollResult<HashMap<String, DataType>> {
        let mut db_hashmap = HashMap::new();
        while !encoded_data.is_empty() {
            let key_size = from_be_bytes!(usize, encoded_data);
            let key = String::from_utf8(encoded_data.drain(0..key_size).collect())
                .map_err(|_| SmollError::DecodeError)?;
            match encoded_data.pop_front().ok_or(SmollError::DecodeError)? {
                0 => {
                    let data = encoded_data.pop_front().ok_or(SmollError::DecodeError)? != 0;
                    db_hashmap.insert(key.clone(), DataType::BOOL(data));
                }
                1 => {
                    let data = from_be_bytes!(i8, encoded_data);
                    db_hashmap.insert(key.clone(), DataType::INT8(data));
                }
                2 => {
                    let data = from_be_bytes!(i16, encoded_data);
                    db_hashmap.insert(key.clone(), DataType::INT16(data));
                }
                3 => {
                    let data = from_be_bytes!(i32, encoded_data);
                    db_hashmap.insert(key.clone(), DataType::INT32(data));
                }
                4 => {
                    let data = from_be_bytes!(i64, encoded_data);
                    db_hashmap.insert(key.clone(), DataType::INT64(data));
                }
                5 => {
                    let data = from_be_bytes!(f32, encoded_data);
                    db_hashmap.insert(key.clone(), DataType::FLOAT32(data));
                }
                6 => {
                    let data = from_be_bytes!(f64, encoded_data);
                    db_hashmap.insert(key.clone(), DataType::FLOAT64(data));
                }
                7 => {
                    let data_size = dbg!(from_be_bytes!(usize, encoded_data));
                    let data = String::from_utf8(encoded_data.drain(0..data_size).collect())
                        .map_err(|_| SmollError::DecodeError)?;
                    db_hashmap.insert(key.clone(), DataType::STRING(data));
                }
                8 => {
                    let size = from_be_bytes!(usize, encoded_data);
                    let data = encoded_data.drain(0..size).collect::<Vec<u8>>();
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
    /// ```no_run
    ///# use smolldb::{DataType, SmollDB};
    ///let mut database = SmollDB::default();
    ///let data = String::from("data");
    ///database.set("example",data.clone());
    ///database.backup("myfile").unwrap();
    ///let database = SmollDB::load("mydatabase").unwrap();
    ///let result = database.get("example").unwrap();
    ///assert_eq!(*result, DataType::STRING(data));
    /// ```
    pub fn load(path: impl AsRef<Path>) -> SmollResult<Self> {
        let encoded_data = Self::read_file(path)?;
        let (encoded_data, _) = decompress(&encoded_data, Format::Zlib)?;
        let data = Self::decode(encoded_data.into())?;
        Ok(Self { inner: data })
    }
    ///Backup database on file, path doesn't need the .smoll extention
    /// # Example
    /// ```no_run
    ///# use smolldb::{DataType, SmollDB};
    ///let mut database = SmollDB::default();
    ///let data = String::from("data");
    ///database.set("example",data.clone());
    ///database.backup("myfile").unwrap();
    ///let database = SmollDB::load("mydatabase").unwrap();
    ///let result = database.get("example").unwrap();
    ///assert_eq!(*result, DataType::STRING(data));
    /// ```
    pub fn backup(&self, file: impl AsRef<Path>) -> SmollResult<()> {
        let data = self.encode();
        let data = compress(&data, Format::Zlib, CompressionLevel::BestSpeed)?;
        self.save_file(file, &data)
    }
    ///Save `value` in the database with the specified `key`
    /// # Example
    /// ```no_run
    ///# use smolldb::{DataType, SmollDB};
    ///let mut database = SmollDB::default();
    ///let data = String::from("data");
    ///database.set("example",data.clone());
    ///let result = database.get("example").unwrap();
    ///assert_eq!(*result, DataType::STRING(data));
    /// ```
    pub fn set(&mut self, key: impl ToString, value: impl Into<DataType>) -> Option<DataType> {
        self.inner.insert(key.to_string(), value.into())
    }
    ///Get `value` with the specified `key`, return a empty optional if `key` is not contained in the database
    /// # Example
    /// ```no_run
    ///# use smolldb::{DataType, SmollDB};
    ///let mut database = SmollDB::default();
    ///let data = String::from("data");
    ///database.set("example",data.clone());
    ///let result = database.get("example").unwrap();
    ///assert_eq!(*result, DataType::STRING(data));
    /// ```
    pub fn get(&self, key: impl ToString) -> Option<&DataType> {
        self.inner.get(&key.to_string())
    }
}
