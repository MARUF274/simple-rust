use std::str::FromStr;

use serde_json::{Map, Value};

//* String
#[allow(dead_code)]
pub trait StringParser {
    fn to_vec<T: FromStr>(&self, delimiter: &str) -> Vec<T>;
}

impl StringParser for str {
    fn to_vec<T: FromStr>(&self, delimiter: &str) -> Vec<T> {
        self.split(delimiter)
            .filter_map(|s| s.trim().parse::<T>().ok())
            .collect()
    }
}

//* Value
#[allow(dead_code)]
pub trait ValueParser {
    fn as_str_or_default(&self) -> String;
    fn as_f64_or_default(&self) -> f64;
    fn as_i64_or_default(&self) -> i64;
    fn as_u64_or_default(&self) -> u64;
    fn as_bool_or_default(&self) -> bool;
    fn as_object_or_default(&self) -> Map<String, Value>;
    fn as_array_or_default(&self) -> Vec<Value>;
    fn get_or_null(&self, key: &str) -> Value;
}

impl ValueParser for Value {
    fn as_str_or_default(&self) -> String {
        self.as_str().unwrap_or_default().to_owned()
    }

    fn as_f64_or_default(&self) -> f64 {
        self.as_f64().unwrap_or_default().to_owned()
    }

    fn as_i64_or_default(&self) -> i64 {
        self.as_i64().unwrap_or_default().to_owned()
    }

    fn as_u64_or_default(&self) -> u64 {
        self.as_u64().unwrap_or_default().to_owned()
    }

    fn as_bool_or_default(&self) -> bool {
        self.as_bool().unwrap_or_default().to_owned()
    }

    fn as_object_or_default(&self) -> Map<String, Value> {
        self.as_object().unwrap_or(&Map::new()).to_owned()
    }

    fn as_array_or_default(&self) -> Vec<Value> {
        self.as_array().unwrap_or(&Vec::new()).to_owned()
    }

    fn get_or_null(&self, key: &str) -> Value {
        self.get(key).unwrap_or(&Value::Null).to_owned()
    }
}

pub fn to_array(map: Value) -> Vec<Value> {
    map.as_array().unwrap_or(&Vec::new()).to_owned()
}


pub fn map_get(key: &str, value: Value) -> Value {
    value
        .as_object()
        .unwrap_or(&Map::new())
        .get(key)
        .unwrap_or(&Value::Null)
        .to_owned()
}

#[allow(dead_code)]
pub fn to_f64(map: Value) -> f64 {
    map.as_f64().unwrap_or(0.0).to_owned()
}

pub fn to_i64(map: Value) -> i64 {
    map.as_i64().unwrap_or(0).to_owned()
}

pub fn to_str(map: Value) -> String {
    map.as_str().unwrap_or("").to_owned()
}



