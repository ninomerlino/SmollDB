use crate::*;
use std::{
    collections::{hash_map::IntoIter, HashMap, VecDeque},
    fs::File,
    io::{Read, Write},
    iter::IntoIterator,
    mem::size_of,
    path::Path,
};
use yazi::{compress, decompress, CompressionLevel, Format};

//TODO: use &[] instead of vecdequeque
//TODO: get and [] functions

macro_rules! from_be_bytes {
    ($type_name:ty, $data_buffer:ident) => {
        <$type_name>::from_be_bytes(
            $data_buffer
                .drain(0..size_of::<$type_name>())
                .collect::<Vec<u8>>()
                .try_into()
                .map_err(|_| Error::DecodeError)?,
        )
    };
}

///Object to represent the in memory database
#[derive(Debug, PartialEq, Default)]
pub struct SmollDB {
    inner: HashMap<String, DataType>,
}

impl SmollDB {
    fn save_file(&self, filename: impl AsRef<Path>, data: &[u8]) -> Result<()> {
        let mut filename = filename.as_ref().to_owned();
        filename.set_extension("smoll");
        File::create(filename)?.write_all(data)?;
        Ok(())
    }

    fn read_file(filename: impl AsRef<Path>) -> Result<Vec<u8>> {
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
                    encoded_data.extend(value.len().to_be_bytes());
                    encoded_data.extend(value.as_bytes());
                }
                DataType::BYTES(value) => {
                    encoded_data.extend(value.len().to_be_bytes());
                    encoded_data.extend(value);
                }
            }
        }
        encoded_data
    }

    fn decode(mut encoded_data: VecDeque<u8>) -> Result<HashMap<String, DataType>> {
        let mut db_hashmap = HashMap::new();
        while !encoded_data.is_empty() {
            let key_size = from_be_bytes!(usize, encoded_data);
            let key = String::from_utf8(encoded_data.drain(0..key_size).collect())
                .map_err(|_| Error::DecodeError)?;
            match encoded_data.pop_front().ok_or(Error::DecodeError)? {
                0 => {
                    let data = encoded_data.pop_front().ok_or(Error::DecodeError)? != 0;
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
                    let data_size = from_be_bytes!(usize, encoded_data);
                    let data = String::from_utf8(encoded_data.drain(0..data_size).collect())
                        .map_err(|_| Error::DecodeError)?;
                    db_hashmap.insert(key.clone(), DataType::STRING(data));
                }
                8 => {
                    let size = from_be_bytes!(usize, encoded_data);
                    let data = encoded_data.drain(0..size).collect::<Vec<u8>>();
                    db_hashmap.insert(key.clone(), DataType::BYTES(data));
                }
                _ => {
                    return Err(Error::DecodeError);
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
    ///let key = String::from("example");
    ///database.set(key.clone(), data.clone());
    ///database.backup("myfile").unwrap();
    ///let database = SmollDB::load("myfile").unwrap();
    ///let result = database.get(&key).unwrap();
    ///assert_eq!(*result, DataType::STRING(data));
    /// ```
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
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
    ///let key = String::from("example");
    ///database.set(key.clone(), data.clone());
    ///database.backup("myfile").unwrap();
    ///let database = SmollDB::load("myfile").unwrap();
    ///let result = database.get(&key).unwrap();
    ///assert_eq!(*result, DataType::STRING(data));
    /// ```
    pub fn backup(&self, path: impl AsRef<Path>) -> Result<()> {
        let data = self.encode();
        let data = compress(&data, Format::Zlib, CompressionLevel::BestSpeed)?;
        self.save_file(path, &data)
    }
    ///Load database from generic stream
    /// # Example
    /// ```no_run
    ///# use smolldb::{DataType, SmollDB};
    ///# use std::fs::{OpenOptions};
    ///# use std::io::{Seek};
    ///let mut database = SmollDB::default();
    ///let mut stream = OpenOptions::new().create(true).read(true).write(true).open("myfile.smoll").unwrap();
    ///let data = String::from("data");
    ///let key = String::from("example");
    ///database.set(key.clone(), data.clone());
    ///database.backup_to_stream(&mut stream).unwrap();
    ///stream.seek(std::io::SeekFrom::Start(0)).unwrap();
    ///let database = SmollDB::load_from_stream(&mut stream).unwrap();
    ///let result = database.get(&key).unwrap();
    ///assert_eq!(*result, DataType::STRING(data));
    /// ```
    pub fn load_from_stream(stream: &mut impl Read) -> Result<Self> {
        let mut encoded_data = Vec::new();
        stream.read_to_end(&mut encoded_data)?;
        let (encoded_data, _) = decompress(&encoded_data, Format::Zlib)?;
        let data = Self::decode(encoded_data.into())?;
        Ok(Self { inner: data })
    }
    ///Backup database onto generic stream
    /// # Example
    /// ```no_run
    ///# use smolldb::{DataType, SmollDB};
    ///# use std::fs::{OpenOptions};
    ///# use std::io::{Seek};
    ///let mut database = SmollDB::default();
    ///let mut stream = OpenOptions::new().create(true).read(true).write(true).open("myfile.smoll").unwrap();
    ///let data = String::from("data");
    ///let key = String::from("example");
    ///database.set(key.clone(), data.clone());
    ///database.backup_to_stream(&mut stream).unwrap();
    ///stream.seek(std::io::SeekFrom::Start(0)).unwrap();
    ///let database = SmollDB::load_from_stream(&mut stream).unwrap();
    ///let result = database.get(&key).unwrap();
    ///assert_eq!(*result, DataType::STRING(data));
    /// ```
    pub fn backup_to_stream(&self, stream: &mut impl Write) -> Result<()> {
        let data = self.encode();
        let data = compress(&data, Format::Zlib, CompressionLevel::BestSpeed)?;
        stream.write_all(&data)?;
        Ok(())
    }
    ///Save `value` in the database with the specified `key`
    /// # Example
    /// ```no_run
    ///# use smolldb::{DataType, SmollDB};
    ///let mut database = SmollDB::default();
    ///let data = String::from("data");
    ///let key = String::from("example");
    ///database.set(key.clone(), data.clone());
    ///let result = database.get(&key).unwrap();
    ///assert_eq!(*result, DataType::STRING(data));
    /// ```
    #[inline]
    pub fn set(&mut self, key: impl ToString, value: impl Into<DataType>) -> Option<DataType> {
        self.inner.insert(key.to_string(), value.into())
    }
    ///Get `value` with the specified `key`, return a empty optional if `key` is not contained in the database
    /// # Example
    /// ```no_run
    ///# use smolldb::{DataType, SmollDB};
    ///let mut database = SmollDB::default();
    ///let data = String::from("data");
    ///let key = String::from("example");
    ///database.set(key.clone(), data.clone());
    ///let result = database.get(&key).unwrap();
    ///assert_eq!(*result, DataType::STRING(data));
    /// ```
    #[inline]
    pub fn get(&self, key: &impl ToString) -> Option<&DataType> {
        self.inner.get(&key.to_string())
    }
    ///Check if database contains the specified key
    ///  # Example
    /// ```no_run
    ///# use smolldb::{DataType, SmollDB};
    ///let mut database = SmollDB::default();
    ///let data = String::from("data");
    ///let key = String::from("example");
    ///database.set(key.clone(), data.clone());
    ///assert!(database.contains_key(&key));
    ///database.remove(&key);
    ///assert!(!database.contains_key(&key));
    /// ```
    pub fn contains_key(&self, key: &impl ToString) -> bool {
        self.inner.contains_key(&key.to_string())
    }
    ///Remove element with the specified key from the database and returns it's value
    ///  # Example
    /// ```no_run
    ///# use smolldb::{DataType, SmollDB};
    ///let mut database = SmollDB::default();
    ///let data = String::from("data");
    ///let key = String::from("example");
    ///database.set(key.clone(), data.clone());
    ///assert!(database.contains_key(&key));
    ///database.remove(&key);
    ///assert!(!database.contains_key(&key));
    /// ```
    pub fn remove(&mut self, key: &impl ToString) -> Option<DataType> {
        self.inner.remove(&key.to_string())
    }
    ///Similar to [`get`](crate::SmollDB::get) but converts the [`DataType`](crate::DataType) to the specified type if possible.
    ///Returns `None` if the key is not contained in the database
    ///Returns [`ConversionError`](crate::errors::Error::ConversionError) if the conversion fails
    ///If T is a reference the reference lifetime is bound to the database lifetime
    ///  # Example
    /// ```no_run
    /// # use smolldb::{DataType, SmollDB};
    /// let mut database = SmollDB::default();
    /// let data = String::from("data");
    /// database.set(&"example", data.clone());
    /// let extracted : &String = database.extract(&"example").unwrap().unwrap();
    /// assert_eq!(extracted, &data);
    /// ```
    #[inline]
    pub fn extract<'c,T: TryFrom<&'c DataType, Error = Error>>(&'c self, key: &impl ToString) -> Option<Result<T>>{
        self.get(key).map(T::try_from)
    }
}

impl IntoIterator for SmollDB {
    type Item = (String, DataType);

    type IntoIter = IntoIter<String, DataType>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
