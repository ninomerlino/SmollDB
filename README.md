# SmollDB
## Small in-memory key value database for rust

This is a small in-memory key-value database, which can be easly backed up in a file and later loaded from it

---

### **It ain't much but it's honest work**

This database is nothing but an hashmap but it already comes with function to easly load and save the hashmap on file, it also compress it in a [Zlib](https://en.wikipedia.org/wiki/Zlib) compatible format

### **Inspired by Pickles**

This crate was inspired by [pickleDB](https://github.com/patx/pickledb) and it works in similars use cases

---

## Some examples
### Basic use
```Rust
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
```
### Loading from file
```Rust
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
```