use std::time::Instant;

use crate::{db::SmollDB, util::DataType};
use rand::{thread_rng, distributions::{Alphanumeric, DistString}};


#[test]
fn set_and_get() {
    let mut db = SmollDB::default();
    db.set("Nome", "Mario".to_string());
    db.set("Eta", 34_i16);
    db.set("Stinky", true);
    db.set("Height", 23.3_f32);
    db.set("CF", vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    
    assert_eq!(DataType::STRING("Mario".to_string()),*(db.get("Nome").unwrap()));
    assert_eq!(DataType::INT16(34_i16),*(db.get("Eta").unwrap()));
    assert_eq!(DataType::BOOL(true),*(db.get("Stinky").unwrap()));
    assert_eq!(DataType::FLOAT32(23.3_f32),*(db.get("Height").unwrap()));
    assert_eq!(DataType::BYTES(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),*(db.get("CF").unwrap()));
}

#[test]
fn backup_and_load_empty(){
    let db = SmollDB::default();
    db.backup(&"database").unwrap();
    let db_copy = SmollDB::load(&"database").unwrap();
    assert_eq!(db,db_copy);
}

#[test]
fn backup_and_load_bool(){
    let mut db = SmollDB::default();
    db.set("element", true);
    db.backup(&"database").unwrap();
    let db_copy = SmollDB::load(&"database").unwrap();
    assert_eq!(db,db_copy);
}

#[test]
fn backup_and_load_int8(){
    let mut db = SmollDB::default();
    db.set("element", 1_i8);
    db.backup(&"database").unwrap();
    let db_copy = SmollDB::load(&"database").unwrap();
    assert_eq!(db,db_copy);
}

#[test]
fn backup_and_load_int16(){
    let mut db = SmollDB::default();
    db.set("element", 1_i16);
    db.backup(&"database").unwrap();
    let db_copy = SmollDB::load(&"database").unwrap();
    assert_eq!(db,db_copy);
}

#[test]
fn backup_and_load_int32(){
    let mut db = SmollDB::default();
    db.set("element", 1_i32);
    db.backup(&"database").unwrap();
    let db_copy = SmollDB::load(&"database").unwrap();
    assert_eq!(db,db_copy);
}

#[test]
fn backup_and_load_int64(){
    let mut db = SmollDB::default();
    db.set("element", 1_i64);
    db.backup(&"database").unwrap();
    let db_copy = SmollDB::load(&"database").unwrap();
    assert_eq!(db,db_copy);
}

#[test]
fn backup_and_load_float32(){
    let mut db = SmollDB::default();
    db.set("element", 1_f32);
    db.backup(&"database").unwrap();
    let db_copy = SmollDB::load(&"database").unwrap();
    assert_eq!(db,db_copy);
}

#[test]
fn backup_and_load_float64(){
    let mut db = SmollDB::default();
    db.set("element", 1_f64);
    db.backup(&"database").unwrap();
    let db_copy = SmollDB::load(&"database").unwrap();
    assert_eq!(db,db_copy);
}

#[test]
fn backup_and_load_string(){
    let mut db = SmollDB::default();
    db.set("element", String::from("S.M.O.L.L."));
    db.backup(&"database").unwrap();
    let db_copy = SmollDB::load(&"database").unwrap();
    assert_eq!(db,db_copy);
}

#[test]
fn backup_and_load_bytes(){
    let mut db = SmollDB::default();
    db.set("element", vec![0_u8, 1,2,3,4,5,6,7,8,9,10,12]);
    db.backup(&"database").unwrap();
    let db_copy = SmollDB::load(&"database").unwrap();
    assert_eq!(db,db_copy);
}

#[test]
fn backup_and_load_complete(){
    let mut db = SmollDB::default();
    db.set("bool", false);
    db.set("int8", 8_i8);
    db.set("int16", 8_i16);
    db.set("int32", 8_i32);
    db.set("int64", 8_i64);
    db.set("float32", 4_f32);
    db.set("float64", 4_f64);
    db.set("string", String::from("8_i8"));
    db.set("bytes", vec![1,2,3,4,5,6,7,8,243,123,46,11,123,65,2,3,5,7,2]);    
    db.backup(&"database").unwrap();
    let db_copy = SmollDB::load(&"database").unwrap();
    assert_eq!(db,db_copy);
}

#[test]
fn example_db_default(){
    let mut database = SmollDB::default();
    let data = String::from("data");
    database.set("example",data.clone());
    match database.get("example"){
        Some(result) => {
            assert_eq!(*result, DataType::STRING(data))
        },
        None => todo!(),
    };
}

#[test]
fn example_db_load(){
    let mut database = SmollDB::default();
    let data = String::from("data");
    database.set("example",data.clone());
    database.backup("myfile").unwrap();
    let database = SmollDB::load("myfile").unwrap();
    match database.get("example"){
        Some(result) => {
            assert_eq!(*result, DataType::STRING(data))
        },
        None => todo!(),
    };
}