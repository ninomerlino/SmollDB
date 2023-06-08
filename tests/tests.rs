use std::{fs::OpenOptions, io::Seek};

use serial_test::serial;
use smolldb::{DataType, SmollDB};

#[test]
fn set_and_get() {
    let mut db = SmollDB::default();
    db.set("Nome", "Mario".to_string());
    db.set("Eta", 34_i16);
    db.set("Stinky", true);
    db.set("Height", 23.3_f32);
    db.set("CF", vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    assert_eq!(
        DataType::STRING("Mario".to_string()),
        *(db.get(&"Nome").unwrap())
    );
    assert_eq!(DataType::INT16(34_i16), *(db.get(&"Eta").unwrap()));
    assert_eq!(DataType::BOOL(true), *(db.get(&"Stinky").unwrap()));
    assert_eq!(DataType::FLOAT32(23.3_f32), *(db.get(&"Height").unwrap()));
    assert_eq!(
        DataType::BYTES(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
        *(db.get(&"CF").unwrap())
    );
}

#[test]
#[serial]
fn backup_and_load_empty() {
    let db = SmollDB::default();
    db.backup(&"database").unwrap();
    let db_copy = SmollDB::load(&"database").unwrap();
    assert_eq!(db, db_copy);
}

#[test]
#[serial]
fn backup_and_load_bool() {
    let mut db = SmollDB::default();
    db.set("element", true);
    db.backup(&"database").unwrap();
    let db_copy = SmollDB::load(&"database").unwrap();
    assert_eq!(db, db_copy);
}

#[test]
#[serial]
fn backup_and_load_int8() {
    let mut db = SmollDB::default();
    db.set("element", 1_i8);
    db.backup(&"database").unwrap();
    let db_copy = SmollDB::load(&"database").unwrap();
    assert_eq!(db, db_copy);
}

#[test]
#[serial]
fn backup_and_load_int16() {
    let mut db = SmollDB::default();
    db.set("element", 1_i16);
    db.backup(&"database").unwrap();
    let db_copy = SmollDB::load(&"database").unwrap();
    assert_eq!(db, db_copy);
}

#[test]
#[serial]
fn backup_and_load_int32() {
    let mut db = SmollDB::default();
    db.set("element", 1_i32);
    db.backup(&"database").unwrap();
    let db_copy = SmollDB::load(&"database").unwrap();
    assert_eq!(db, db_copy);
}

#[test]
#[serial]
fn backup_and_load_int64() {
    let mut db = SmollDB::default();
    db.set("element", 1_i64);
    db.backup(&"database").unwrap();
    let db_copy = SmollDB::load(&"database").unwrap();
    assert_eq!(db, db_copy);
}

#[test]
#[serial]
fn backup_and_load_float32() {
    let mut db = SmollDB::default();
    db.set("element", 1_f32);
    db.backup(&"database").unwrap();
    let db_copy = SmollDB::load(&"database").unwrap();
    assert_eq!(db, db_copy);
}

#[test]
#[serial]
fn backup_and_load_float64() {
    let mut db = SmollDB::default();
    db.set("element", 1_f64);
    db.backup(&"database").unwrap();
    let db_copy = SmollDB::load(&"database").unwrap();
    assert_eq!(db, db_copy);
}

#[test]
#[serial]
fn backup_and_load_string() {
    let mut db = SmollDB::default();
    db.set("element", String::from("S.M.O.L.L."));
    db.backup(&"database").unwrap();
    let db_copy = SmollDB::load(&"database").unwrap();
    assert_eq!(db, db_copy);
}

#[test]
#[serial]
fn backup_and_load_bytes() {
    let mut db = SmollDB::default();
    db.set("element", vec![0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12]);
    db.backup(&"database").unwrap();
    let db_copy = SmollDB::load(&"database").unwrap();
    assert_eq!(db, db_copy);
}

#[test]
#[serial]
fn backup_and_load_complete() {
    let mut db = SmollDB::default();
    db.set("bool", false);
    db.set("int8", 8_i8);
    db.set("int16", 8_i16);
    db.set("int32", 8_i32);
    db.set("int64", 8_i64);
    db.set("float32", 4_f32);
    db.set("float64", 4_f64);
    db.set("string", String::from("8_i8"));
    db.set(
        "bytes",
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 243, 123, 46, 11, 123, 65, 2, 3, 5, 7, 2,
        ],
    );
    db.backup(&"database").unwrap();
    let db_copy = SmollDB::load(&"database").unwrap();
    assert_eq!(db, db_copy);
}

#[test]
fn example_db_default() {
    let mut database = SmollDB::default();
    let data = String::from("data");
    let key = String::from("example");
    database.set(key.clone(), data.clone());
    let result = database.get(&key).unwrap();
    assert_eq!(*result, DataType::STRING(data));
}

#[test]
#[serial]
fn example_db_load() {
    let mut database = SmollDB::default();
    let data = String::from("data");
    let key = String::from("example");
    database.set(key.clone(), data.clone());
    database.backup("myfile").unwrap();
    let database = SmollDB::load("myfile").unwrap();
    let result = database.get(&key).unwrap();
    assert_eq!(*result, DataType::STRING(data));
}

#[test]
fn example_db_remove_and_check() {
    let mut database = SmollDB::default();
    let data = String::from("data");
    let key = String::from("example");
    database.set(key.clone(), data.clone());
    assert!(database.contains_key(&key));
    database.remove(&key);
    assert!(!database.contains_key(&key));
}

#[test]
#[serial]
fn example_db_load_from_stream() {
    let mut database = SmollDB::default();
    let mut stream = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open("myfile.smoll")
        .unwrap();
    let data = String::from("data");
    let key = String::from("example");
    database.set(key.clone(), data.clone());
    database.backup_to_stream(&mut stream).unwrap();
    stream.seek(std::io::SeekFrom::Start(0)).unwrap();
    let database = SmollDB::load_from_stream(&mut stream).unwrap();
    let result = database.get(&key).unwrap();
    assert_eq!(*result, DataType::STRING(data));
}

#[test]
fn example_db_iteration() {
    let mut database = SmollDB::default();
    let keys = vec!["k1", "k2", "k3", "k4"];
    let values = vec![1, 2, 3, 4];
    for (pos, key) in keys.clone().into_iter().enumerate() {
        database.set(key.clone(), values[pos].clone());
    }
    for (k, v) in database {
        assert_eq!(v, values[keys.iter().position(|&x| x == k).unwrap()].into())
    }
}

#[test]
fn datetype_try_from_conversion() {
    let bool_datatype = DataType::from(true);
    let int8_datatype = DataType::from(8_i8);
    let int16_datatype = DataType::from(8_i16);
    let int32_datatype = DataType::from(8_i32);
    let int64_datatype = DataType::from(8_i64);
    let float32_datatype = DataType::from(4_f32);
    let float64_datatype = DataType::from(4_f64);
    let string_datatype = DataType::from(String::from("this is my string"));
    let bytes_datatype = DataType::from(vec![1_u8, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(bool::try_from(&bool_datatype).unwrap(), true);
    assert_eq!(i8::try_from(&int8_datatype).unwrap(), 8_i8);
    assert_eq!(i16::try_from(&int16_datatype).unwrap(), 8_i16);
    assert_eq!(i32::try_from(&int32_datatype).unwrap(), 8_i32);
    assert_eq!(i64::try_from(&int64_datatype).unwrap(), 8_i64);
    assert_eq!(f32::try_from(&float32_datatype).unwrap(), 4_f32);
    assert_eq!(f64::try_from(&float64_datatype).unwrap(), 4_f64);
    assert_eq!(
        String::try_from(&string_datatype).unwrap(),
        String::from("this is my string")
    );
    assert_eq!(
        Vec::<u8>::try_from(&bytes_datatype).unwrap(),
        vec![1_u8, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    );
}

#[test]
fn test_db_extract(){
    let mut database = SmollDB::default();
    let bool_data = true;
    let int8_data = 8_i8;
    let int16_data = 8_i16;
    let int32_data = 8_i32;
    let int64_data = 8_i64;
    let float32_data = 4_f32;
    let float64_data = 4_f64;
    let string_data = String::from("this is my string");
    let bytes_data = vec![1_u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    database.set("bool", bool_data);
    database.set("int8", int8_data);
    database.set("int16", int16_data);
    database.set("int32", int32_data);
    database.set("int64", int64_data);
    database.set("float32", float32_data);
    database.set("float64", float64_data);
    database.set("string", string_data.clone());
    database.set("bytes", bytes_data.clone());

    let bool_data_extracted: &bool = database.extract(&"bool").unwrap().unwrap();
    let int8_data_extracted: &i8 = database.extract(&"int8").unwrap().unwrap();
    let int16_data_extracted: &i16 = database.extract(&"int16").unwrap().unwrap();
    let int32_data_extracted: &i32 = database.extract(&"int32").unwrap().unwrap();
    let int64_data_extracted: &i64 = database.extract(&"int64").unwrap().unwrap();
    let float32_data_extracted: &f32 = database.extract(&"float32").unwrap().unwrap();
    let float64_data_extracted: &f64 = database.extract(&"float64").unwrap().unwrap();
    let string_data_extracted: &String = database.extract(&"string").unwrap().unwrap();
    let bytes_data_extracted: &Vec<u8> = database.extract(&"bytes").unwrap().unwrap();

    assert_eq!(bool_data, *bool_data_extracted);
    assert_eq!(int8_data, *int8_data_extracted);
    assert_eq!(int16_data, *int16_data_extracted);
    assert_eq!(int32_data, *int32_data_extracted);
    assert_eq!(int64_data, *int64_data_extracted);
    assert_eq!(float32_data, *float32_data_extracted);
    assert_eq!(float64_data, *float64_data_extracted);
    assert_eq!(string_data, *string_data_extracted);
    assert_eq!(bytes_data, *bytes_data_extracted);
}

#[test]
fn test_db_extract_clone(){
    let mut database = SmollDB::default();
    let bool_data = true;
    let int8_data = 8_i8;
    let int16_data = 8_i16;
    let int32_data = 8_i32;
    let int64_data = 8_i64;
    let float32_data = 4_f32;
    let float64_data = 4_f64;
    let string_data = String::from("this is my string");
    let bytes_data = vec![1_u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    database.set("bool", bool_data);
    database.set("int8", int8_data);
    database.set("int16", int16_data);
    database.set("int32", int32_data);
    database.set("int64", int64_data);
    database.set("float32", float32_data);
    database.set("float64", float64_data);
    database.set("string", string_data.clone());
    database.set("bytes", bytes_data.clone());

    let bool_data_extracted: bool = database.extract(&"bool").unwrap().unwrap();
    let int8_data_extracted: i8 = database.extract(&"int8").unwrap().unwrap();
    let int16_data_extracted: i16 = database.extract(&"int16").unwrap().unwrap();
    let int32_data_extracted: i32 = database.extract(&"int32").unwrap().unwrap();
    let int64_data_extracted: i64 = database.extract(&"int64").unwrap().unwrap();
    let float32_data_extracted: f32 = database.extract(&"float32").unwrap().unwrap();
    let float64_data_extracted: f64 = database.extract(&"float64").unwrap().unwrap();
    let string_data_extracted: String = database.extract(&"string").unwrap().unwrap();
    let bytes_data_extracted: Vec<u8> = database.extract(&"bytes").unwrap().unwrap();

    assert_eq!(bool_data, bool_data_extracted);
    assert_eq!(int8_data, int8_data_extracted);
    assert_eq!(int16_data, int16_data_extracted);
    assert_eq!(int32_data,  int32_data_extracted);
    assert_eq!(int64_data,  int64_data_extracted);
    assert_eq!(float32_data, float32_data_extracted);
    assert_eq!(float64_data, float64_data_extracted);
    assert_eq!(string_data, string_data_extracted);
    assert_eq!(bytes_data, bytes_data_extracted);
}