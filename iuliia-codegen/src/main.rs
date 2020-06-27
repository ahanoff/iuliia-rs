use codegen::{Scope, Type};
use std::fs::File;
use serde::{Serialize, Deserialize};
use std::io::Write;
use serde_json::{Map, Value};
use clap::{App, Arg};

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
    let matches = App::new("iuliia-codegen")
        .version("1.0")
        .author("Akhan Zhakiyanov <ahanoff@gmail.com>")
        .about("Generates boilerplate for transliteration schema")
        .arg(Arg::with_name("file").short("f").takes_value(true))
        .get_matches();

    match matches.value_of("file") {
        Some(filename) => {
            let json_file = File::open(filename).expect("File not found");
            let schema: Schema = serde_json::from_reader(&json_file).expect("error while reading json");
            let mut scope = Scope::new();
            let schema_name = schema.name.as_str();

            let _schema_struct = scope.new_struct(schema_name);

            let default_impl = scope.new_impl(schema_name);
            default_impl.impl_trait("Default");
            default_impl.new_fn("default")
                .ret(Type::new("Self"));

            let transliterator_impl = scope.new_impl(schema_name);
            transliterator_impl.impl_trait("Transliterator");
            transliterator_impl.new_fn("transliterate")
                .arg_ref_self()
                .arg("input", Type::new("&str"))
                .ret(Type::new("String"))
                .line("unimplemented!()");

            println!("{}", scope.to_string());

            let schema_filename =format!("/home/ahanoff/opsless/iuliia-rs/generated/{}.rs", schema.name.as_str());
            let mut f = File::create(schema_filename).expect("File not created");
            f.write_all(scope.to_string().as_ref()).expect("couldn't write to file");
            println!("Some")
        },
        None => {
            println!("None")
        }
    }
}
