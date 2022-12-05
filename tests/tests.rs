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
