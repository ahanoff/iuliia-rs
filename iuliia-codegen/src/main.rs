use codegen::{Scope, Type};
use std::fs::File;
use std::io::Write;
use clap::{App, Arg};
use iuliia_codegen::Schema;

fn main() -> anyhow::Result<()> {
    let matches = App::new("iuliia-codegen")
        .version("1.0")
        .author("Akhan Zhakiyanov <ahanoff@gmail.com>")
        .about("Generates boilerplate for transliteration schema")
        .arg(Arg::with_name("file").short("f").takes_value(true))
        .get_matches();

    match matches.value_of("file") {
        Some(filename) => {
            let json_file = File::open(filename)?;
            let schema: Schema = serde_json::from_reader(&json_file)?;
            let mut scope = Scope::new();
            let schema_name = schema.name.as_str();

            scope.raw("use std::collections::HashMap;");
            scope.raw("use crate::Transliterator;");

            let schema_struct = scope.new_struct(schema_name);
            schema_struct.field("mapping", Type::new("HashMap<String, String>"));
            schema_struct.field("prev_mapping", Type::new("Option<HashMap<String, String>>"));
            schema_struct.field("next_mapping", Type::new("Option<HashMap<String, String>>"));
            schema_struct.field("ending_mapping", Type::new("Option<HashMap<String, String>>"));


            let default_impl = scope.new_impl(schema_name);
            default_impl.impl_trait("Default");
            default_impl.new_fn("default")
                .line("let mut mapping = HashMap::new();")
                .line("let mut next_mapping = HashMap::new();")
                .line("let mut prev_mapping = HashMap::new();")
                .line("let mut ending_mapping = HashMap::new();")

                .ret(Type::new("Self"));

            let transliterator_impl = scope.new_impl(schema_name);
            transliterator_impl.impl_trait("Transliterator");
            transliterator_impl.new_fn("transliterate")
                .arg_ref_self()
                .arg("input", Type::new("&str"))
                .ret(Type::new("String"))
                .line("let mut output = String::from(\"\");");

            println!("{}", scope.to_string());

            let schema_filename =format!("/home/ahanoff/opsless/iuliia-rs/generated/{}.rs", schema.name.as_str());
            let mut f = File::create(schema_filename)?;
            f.write_all(scope.to_string().as_ref())?;
            println!("Some");
            Ok(())
        },
        None => {
            println!("No file provided");
            Ok(())
        }
    }
}
