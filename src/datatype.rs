use std::fmt::Debug;

impl From<bool> for DataType {
    fn from(value: bool) -> Self {
        Self::BOOL(value)
    }
}

impl From<i8> for DataType {
    fn from(value: i8) -> Self {
        Self::INT8(value)
    }
}

impl From<i16> for DataType {
    fn from(value: i16) -> Self {
        Self::INT16(value)
    }
}

impl From<i32> for DataType {
    fn from(value: i32) -> Self {
        Self::INT32(value)
    }
}

impl From<i64> for DataType {
    fn from(value: i64) -> Self {
        Self::INT64(value)
    }
}

impl From<f32> for DataType {
    fn from(value: f32) -> Self {
        Self::FLOAT32(value)
    }
}

impl From<f64> for DataType {
    fn from(value: f64) -> Self {
        Self::FLOAT64(value)
    }
}

impl From<String> for DataType {
    fn from(value: String) -> Self {
        Self::STRING(value)
    }
}

impl From<Vec<u8>> for DataType {
    fn from(value: Vec<u8>) -> Self {
        Self::BYTES(value)
    }
}

///A union for all the types supported by smolldb
///
///Supported types are:
///* `bool`
///* `i8`
///* `i16`
///* `i32`
///* `i64`
///* `f32`
///* `f64`
///* `String`
///* `Vec<u8>`
#[derive(PartialEq, Debug)]
pub enum DataType {
    BOOL(bool),
    INT8(i8),
    INT16(i16),
    INT32(i32),
    INT64(i64),
    FLOAT32(f32),
    FLOAT64(f64),
    STRING(String),
    BYTES(Vec<u8>),
}

impl DataType {
    ///get the id of the current type stored in this Datatype obj
    ///*used for the encoding step*
    pub(crate) fn id(&self) -> u8 {
        match self {
            DataType::BOOL(_) => 0,
            DataType::INT8(_) => 1,
            DataType::INT16(_) => 2,
            DataType::INT32(_) => 3,
            DataType::INT64(_) => 4,
            DataType::FLOAT32(_) => 5,
            DataType::FLOAT64(_) => 6,
            DataType::STRING(_) => 7,
            DataType::BYTES(_) => 8,
        }
    }
}
