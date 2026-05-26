use anyhow::{anyhow, bail, Context, Result};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct SchemaJson {
    name: String,
    description: String,
    url: String,
    #[serde(default)]
    aliases: Vec<String>,
    #[serde(default)]
    comments: Vec<String>,
    mapping: BTreeMap<String, String>,
    #[serde(default)]
    prev_mapping: Option<BTreeMap<String, String>>,
    #[serde(default)]
    next_mapping: Option<BTreeMap<String, String>>,
    #[serde(default)]
    ending_mapping: Option<BTreeMap<String, String>>,
    samples: Vec<(String, String)>,
}

#[derive(Debug)]
struct SchemaFile {
    module_name: String,
    type_name: String,
    schema: SchemaJson,
}

fn main() -> Result<()> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let schemas_dir = manifest_dir.join("../schemas");
    let output_dir = manifest_dir.join("../iuliia/src/schemas");

    let schemas = read_schemas(&schemas_dir)?;

    fs::create_dir_all(&output_dir)
        .with_context(|| format!("failed to create output directory {}", output_dir.display()))?;

    for schema_file in &schemas {
        let output_path = output_dir.join(format!("{}.rs", schema_file.module_name));
        fs::write(&output_path, generate_schema_file(schema_file)?)
            .with_context(|| format!("failed to write {}", output_path.display()))?;
    }

    let mod_path = output_dir.join("mod.rs");
    fs::write(&mod_path, generate_mod_file(&schemas))
        .with_context(|| format!("failed to write {}", mod_path.display()))?;

    println!(
        "Generated {} schemas in {}",
        schemas.len(),
        output_dir.display()
    );

    Ok(())
}

fn read_schemas(schemas_dir: &Path) -> Result<Vec<SchemaFile>> {
    let mut schemas = Vec::new();

    for entry in fs::read_dir(schemas_dir)
        .with_context(|| format!("failed to read schemas directory {}", schemas_dir.display()))?
    {
        let entry = entry
            .with_context(|| format!("failed to read an entry from {}", schemas_dir.display()))?;
        let path = entry.path();

        if path.extension().and_then(|extension| extension.to_str()) != Some("json") {
            continue;
        }

        let module_name = path
            .file_stem()
            .and_then(|file_stem| file_stem.to_str())
            .ok_or_else(|| anyhow!("schema file has invalid UTF-8 name: {}", path.display()))?
            .to_owned();

        let contents = fs::read_to_string(&path)
            .with_context(|| format!("failed to read schema file {}", path.display()))?;
        let schema: SchemaJson = serde_json::from_str(&contents)
            .with_context(|| format!("failed to parse schema file {}", path.display()))?;

        if schema.name != module_name {
            bail!(
                "schema name '{}' does not match file name '{}' in {}",
                schema.name,
                module_name,
                path.display()
            );
        }

        schemas.push(SchemaFile {
            type_name: to_pascal_case(&schema.name),
            module_name,
            schema,
        });
    }

    schemas.sort_by(|left, right| left.module_name.cmp(&right.module_name));
    Ok(schemas)
}

fn generate_schema_file(schema_file: &SchemaFile) -> Result<String> {
    let schema = &schema_file.schema;
    let type_name = &schema_file.type_name;
    let mut output = String::new();

    output.push_str(&format!("//! {}\n", schema.description));
    output.push_str("//!\n");
    output.push_str(&format!("//! See: <{}>\n", schema.url));
    if !schema.comments.is_empty() {
        output.push_str("//!\n");
        for comment in &schema.comments {
            // Escape lines that could be misinterpreted as markdown list items or blockquotes
            let escaped = if comment.trim_start().starts_with('-')
                || comment.trim_start().starts_with('>')
            {
                format!("\\{}", comment)
            } else {
                comment.clone()
            };
            output.push_str(&format!("//! {escaped}\n"));
        }
    }
    output.push('\n');
    output.push_str("use crate::Schema;\n\n");
    output.push_str(&format!("/// {}.\n", schema.description));
    output.push_str(&format!("pub struct {};\n\n", type_name));
    output.push_str(&format!("impl Schema for {} {{\n", type_name));
    output.push_str(&format!(
        "    const NAME: &'static str = {};\n",
        rust_string(&schema.name)
    ));

    if !schema.aliases.is_empty() {
        output.push_str("    const ALIASES: &'static [&'static str] = &[\n");
        for alias in &schema.aliases {
            output.push_str(&format!("        {},\n", rust_string(alias)));
        }
        output.push_str("    ];\n");
    }

    output.push('\n');
    output.push_str("    fn mapping(c: char) -> Option<&'static str> {\n");
    output.push_str("        match c {\n");
    for (key, value) in &schema.mapping {
        output.push_str(&format!(
            "            {} => Some({}),\n",
            rust_char_key(key)?,
            rust_string(value)
        ));
    }
    output.push_str("            _ => None,\n");
    output.push_str("        }\n");
    output.push_str("    }\n\n");

    output.push_str(&generate_prev_mapping(schema.prev_mapping.as_ref())?);
    output.push('\n');
    output.push_str(&generate_next_mapping(schema.next_mapping.as_ref())?);
    output.push('\n');
    output.push_str(&generate_ending_mapping(schema.ending_mapping.as_ref())?);
    output.push_str("}\n\n");

    output.push_str("#[cfg(test)]\n");
    output.push_str("mod tests {\n");
    output.push_str("    use super::*;\n\n");
    for (index, (input, expected)) in schema.samples.iter().enumerate() {
        output.push_str("    #[test]\n");
        output.push_str(&format!("    fn test_sample_{}() {{\n", index));
        output.push_str("        assert_eq!(\n");
        output.push_str(&format!(
            "            {}::transliterate({}),\n",
            type_name,
            rust_string(input)
        ));
        output.push_str(&format!("            {}\n", rust_string(expected)));
        output.push_str("        );\n");
        output.push_str("    }\n");
        if index + 1 != schema.samples.len() {
            output.push('\n');
        }
    }
    output.push_str("}\n");

    Ok(output)
}

fn generate_prev_mapping(mapping: Option<&BTreeMap<String, String>>) -> Result<String> {
    let Some(mapping) = mapping else {
        return Ok("    fn prev_mapping(_prev: Option<char>, _curr: char) -> Option<&'static str> {\n        None\n    }\n".to_owned());
    };

    let mut output = String::new();
    output.push_str(
        "    fn prev_mapping(prev: Option<char>, curr: char) -> Option<&'static str> {\n",
    );
    output.push_str("        match (prev, curr) {\n");
    for (key, value) in mapping {
        let chars = chars_for_key(key);
        let arm = match chars.as_slice() {
            [curr] => format!("(None, {})", rust_char(*curr)),
            [prev, curr] => format!("(Some({}), {})", rust_char(*prev), rust_char(*curr)),
            _ => bail!("prev_mapping key must contain one or two chars: {key}"),
        };
        output.push_str(&format!(
            "            {} => Some({}),\n",
            arm,
            rust_string(value)
        ));
    }
    output.push_str("            _ => None,\n");
    output.push_str("        }\n");
    output.push_str("    }\n");
    Ok(output)
}

fn generate_next_mapping(mapping: Option<&BTreeMap<String, String>>) -> Result<String> {
    let Some(mapping) = mapping else {
        return Ok("    fn next_mapping(_curr: char, _next: char) -> Option<&'static str> {\n        None\n    }\n".to_owned());
    };

    let mut output = String::new();
    output.push_str("    fn next_mapping(curr: char, next: char) -> Option<&'static str> {\n");
    output.push_str("        match (curr, next) {\n");
    for (key, value) in mapping {
        let chars = chars_for_key(key);
        let [curr, next] = chars.as_slice() else {
            bail!("next_mapping key must contain two chars: {key}");
        };
        output.push_str(&format!(
            "            ({}, {}) => Some({}),\n",
            rust_char(*curr),
            rust_char(*next),
            rust_string(value)
        ));
    }
    output.push_str("            _ => None,\n");
    output.push_str("        }\n");
    output.push_str("    }\n");
    Ok(output)
}

fn generate_ending_mapping(mapping: Option<&BTreeMap<String, String>>) -> Result<String> {
    let Some(mapping) = mapping else {
        return Ok("    fn ending_mapping(_ending: [char; 2]) -> Option<&'static str> {\n        None\n    }\n".to_owned());
    };

    let mut output = String::new();
    output.push_str("    fn ending_mapping(ending: [char; 2]) -> Option<&'static str> {\n");
    output.push_str("        match ending {\n");
    for (key, value) in mapping {
        let chars = chars_for_key(key);
        let [first, second] = chars.as_slice() else {
            bail!("ending_mapping key must contain two chars: {key}");
        };
        output.push_str(&format!(
            "            [{}, {}] => Some({}),\n",
            rust_char(*first),
            rust_char(*second),
            rust_string(value)
        ));
    }
    output.push_str("            _ => None,\n");
    output.push_str("        }\n");
    output.push_str("    }\n");
    Ok(output)
}

fn generate_mod_file(schemas: &[SchemaFile]) -> String {
    let mut output = String::new();

    output.push_str("// Auto-generated by iuliia-codegen — DO NOT EDIT MANUALLY\n");
    output.push_str("// Run `cargo run -p iuliia-codegen` to regenerate\n\n");

    for schema_file in schemas {
        output.push_str(&format!("pub mod {};\n", schema_file.module_name));
    }

    output.push_str("\nuse crate::Schema;\n");
    output.push_str("use crate::UnknownSchemaError;\n\n");
    output.push_str("/// Transliterate input text using a named schema.\n");
    output.push_str("///\n");
    output.push_str("/// # Errors\n");
    output.push_str("/// Returns `UnknownSchemaError` if the schema name is not recognized.\n");
    output.push_str("pub fn transliterate(input: &str, schema_name: &str) -> Result<String, UnknownSchemaError> {\n");
    output.push_str("    match schema_name {\n");
    for schema_file in schemas {
        output.push_str("        ");
        output.push_str(&match_patterns(schema_file));
        output.push_str(&format!(
            " => Ok({}::{}::transliterate(input)),\n",
            schema_file.module_name, schema_file.type_name
        ));
    }
    output.push_str("        _ => Err(UnknownSchemaError::new(schema_name)),\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    output.push_str("/// Returns all available schema names (including aliases).\n");
    output.push_str("pub fn schema_names() -> &'static [&'static str] {\n");
    output.push_str("    &[\n");
    for name in schema_names(schemas) {
        output.push_str(&format!("        {},\n", rust_string(&name)));
    }
    output.push_str("    ]\n");
    output.push_str("}\n");

    output
}

fn match_patterns(schema_file: &SchemaFile) -> String {
    std::iter::once(schema_file.schema.name.as_str())
        .chain(schema_file.schema.aliases.iter().map(String::as_str))
        .map(rust_string)
        .collect::<Vec<_>>()
        .join(" | ")
}

fn schema_names(schemas: &[SchemaFile]) -> Vec<String> {
    let mut names = schemas
        .iter()
        .flat_map(|schema_file| {
            std::iter::once(schema_file.schema.name.clone())
                .chain(schema_file.schema.aliases.iter().cloned())
        })
        .collect::<Vec<_>>();
    names.sort();
    names
}

fn to_pascal_case(name: &str) -> String {
    name.split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect()
}

fn chars_for_key(key: &str) -> Vec<char> {
    key.chars().collect()
}

fn rust_string(value: &str) -> String {
    format!("{value:?}")
}

fn rust_char_key(value: &str) -> Result<String> {
    let mut chars = value.chars();
    let Some(ch) = chars.next() else {
        bail!("mapping key must contain one char");
    };
    if chars.next().is_some() {
        bail!("mapping key must contain one char: {value}");
    }
    Ok(rust_char(ch))
}

fn rust_char(value: char) -> String {
    format!("{value:?}")
}
