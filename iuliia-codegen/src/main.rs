use codegen::{Scope, Type};
use std::fs::File;
use serde::{Serialize, Deserialize};
use std::io::Write;
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize)]
struct Schema {
    name: String,
    description: String,
    url: String,
    comments: Vec<String>,
    mapping: Option<Map<String, Value>>,
    prev_mapping: Option<Map<String, Value>>,
    next_mapping: Option<Map<String, Value>>,
    ending_mapping: Option<Map<String, Value>>,
    samples: Vec<(String, String)>
}

fn main() {
    let filename = "/home/ahanoff/opsless/iuliia-rs/schemas/gost_7034.json";
    let json_file = File::open(filename).expect("File not found");
    let schema: Schema = serde_json::from_reader(&json_file).expect("error while reading json");

    let mut scope = Scope::new();
    let schemaName = schema.name.as_str();

    let schemaStruct = scope.new_struct(schemaName);

    let defaultImplementation = scope.new_impl(schemaName);
    defaultImplementation.impl_trait("Default");
    defaultImplementation.new_fn("default")
        .ret(Type::new("Self"));

    let transliteratorImpl = scope.new_impl(schemaName);
    transliteratorImpl.impl_trait("Transliterator");
    transliteratorImpl.new_fn("transliterate")
        .arg_ref_self()
        .arg("input", Type::new("&str"))
        .ret(Type::new("String"))
        .line("unimplemented!()");

    println!("{}", scope.to_string());

    // let schema_filename =format!("/home/ahanoff/opsless/iuliia-rs/generated/{}.rs", schema.name.as_str());
    // let mut f = File::create(schema_filename).expect("File not created");
    // f.write_all(scope.to_string().as_ref()).expect("couldn't write to file");
}
