extern crate serde_yaml;

use serde_yaml::Value;
use std::fs::File;
use std::io::Read;

macro_rules! make_wrapper {
    ($doc_path:ident) => {{
        let doc: serde_yaml::Mapping = fetch_data!($doc_path);

        match doc.get(&Value::String(String::from("components"))) {
            None => (),
            Some(components) => match components.get(&Value::String(String::from("schemas"))) {
                None => (),
                Some(raw_schemas) => make_schemas!(raw_schemas),
            },
        };

        // for (key, value) in doc.iter() {
        //     match key {
        //         Value::String(key) => println!("{key}"),
        //         _ => (),
        //     }
        // }
    }};
}

macro_rules! fetch_data {
    ($doc_path:ident) => {{
        let doc_path = $doc_path;
        let doc_raw = match File::open(&doc_path) {
            Ok(mut file) => {
                let mut contents = String::new();
                let _ = file.read_to_string(&mut contents);
                contents
            }
            Err(_) => match reqwest::blocking::get(&doc_path) {
                Err(_) => panic!("Could not fetch Swagger docs!"),
                Ok(response) => response.text().expect("Could not fetch Swagger docs!"),
            },
        };
        let doc = serde_yaml::from_str(&doc_raw).unwrap();
        doc
    }};
}

macro_rules! make_schemas {
    ($schemas:ident) => {{
        // println!("{:?}", $schemas);
        if let Value::Mapping(schemas) = $schemas {
            for (key, value) in schemas.iter() {
                if let Value::String(type_name) = key {
                    println!("{type_name}");
                    struct type_name {}
                }
            }
        }
    }};
}

struct Order {
    testa: String,
}

impl Order {
    fn new() -> Self {
        Order {
            testa: String::new(),
        }
    }
}

fn main() {
    let doc_file_path = String::from("swagger_example.yml");
    let client = make_wrapper!(doc_file_path);

    let a = Order::new();
    a.test;
}
