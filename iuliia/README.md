# iuliia

![Rust](https://github.com/ahanoff/iuliia-rs/workflows/Rust/badge.svg)
[![crates.io](https://img.shields.io/crates/v/iuliia.svg)](https://crates.io/crates/iuliia)
[![docs.rs](https://docs.rs/iuliia/badge.svg)](https://docs.rs/iuliia)

Transliterate Cyrillic to Latin in every possible way.

`iuliia` is a Rust port of the excellent Python library [iuliia](https://github.com/nalgeon/iuliia-py) by [@nalgeon](https://github.com/nalgeon). It supports standardized transliteration schemas used for international passports, visas, green cards, driving licenses, mail, goods delivery, maps, and other text processing tasks.

The crate has zero dependencies. Schema data is code generated, so typed schema calls compile down to plain Rust code. It is also compatible with `wasm32-unknown-unknown`, which makes it a good fit for browser, worker, and embedded WASM use cases. The public API uses `String`, so the crate targets `std` rather than `no_std`.

## Installation

```sh
cargo add iuliia
```

## Usage

### Transliterate by schema type

Use a generated schema type when the schema is known at compile time. This path is monomorphized and avoids name lookup.

```rust
use iuliia::schemas::wikipedia::Wikipedia;
use iuliia::Schema;

let result = Wikipedia::transliterate("Юлия Щеглова");
assert_eq!(result, "Yuliya Shcheglova");
```

Schema names map to PascalCase type names:

```rust
use iuliia::schemas::icao_doc_9303::IcaoDoc9303;
use iuliia::Schema;

let result = IcaoDoc9303::transliterate("Москва");
assert_eq!(result, "Moskva");
```

### Transliterate by schema name

Use name based lookup when the schema comes from user input or configuration.

```rust
use iuliia::schemas;

let result = schemas::transliterate("Юлия Щеглова", "wikipedia").unwrap();
assert_eq!(result, "Yuliya Shcheglova");

let result = iuliia::transliterate("Санкт-Петербург", "bgn_pcgn").unwrap();
assert_eq!(result, "Sankt-Peterburg");
```

Aliases work too:

```rust
let result = iuliia::transliterate("Юлия", "iso_9_1995").unwrap();
// Same schema as gost_779.
```

List all available schemas:

```rust
for name in iuliia::schemas::schema_names() {
    println!("{name}");
}
```

### Handle unknown schemas

```rust
match iuliia::transliterate("Юлия", "unknown_schema") {
    Ok(result) => println!("{result}"),
    Err(err) => eprintln!("{err}"), // unknown schema: unknown_schema
}
```

## Available schemas

| Schema | Notes |
| --- | --- |
| `ala_lc` | |
| `ala_lc_alt` | |
| `bgn_pcgn` | |
| `bgn_pcgn_alt` | |
| `bs_2979` | |
| `bs_2979_alt` | |
| `gost_16876` | |
| `gost_16876_alt` | |
| `gost_52290` | |
| `gost_52535` | |
| `gost_7034` | |
| `gost_779` | Alias: `iso_9_1995` |
| `gost_779_alt` | |
| `icao_doc_9303` | |
| `iso_9_1954` | |
| `iso_9_1968` | |
| `iso_9_1968_alt` | |
| `mosmetro` | |
| `mvd_310` | |
| `mvd_310_fr` | |
| `mvd_782` | |
| `scientific` | |
| `telegram` | |
| `ungegn_1987` | |
| `wikipedia` | |
| `yandex_maps` | |
| `yandex_money` | |

## WASM support

`iuliia` has no external dependencies and compiles for `wasm32-unknown-unknown`:

```sh
cargo build -p iuliia --target wasm32-unknown-unknown
```

## Maintainers

Schema tables are generated from source data. After changing schemas or code generation logic, regenerate the Rust modules with:

```sh
cargo run -p iuliia-codegen
```

## Contributing

Issues and pull requests are welcome. Please keep changes focused, add tests for behavior changes, and run the crate checks before opening a pull request.

## License

MIT
