# iuliia-rs

![Rust](https://github.com/ahanoff/iuliia-rs/workflows/Rust/badge.svg)
[![crates.io](https://img.shields.io/crates/v/iuliia.svg)](https://crates.io/crates/iuliia)
[![docs.rs](https://docs.rs/iuliia/badge.svg)](https://docs.rs/iuliia)

Rust workspace for `iuliia`, a zero dependency, WASM compatible (`wasm32-unknown-unknown`) Cyrillic to Latin transliteration library using standardized schemas. See the [crate README](iuliia/README.md) for usage, API details, and the schema list.

`iuliia-rs` is a Rust port of [nalgeon/iuliia](https://github.com/nalgeon/iuliia). Credit to [@nalgeon](https://github.com/nalgeon) for the original project.

## Workspace structure

- `iuliia/`, library crate published as `iuliia`. It has zero dependencies.
- `iuliia-codegen/`, development tool that reads schema JSON and generates Rust schema modules.
- `schemas/`, git submodule pointing to `nalgeon/iuliia` for JSON schema definitions.

## Development

```sh
make build
make test
make lint
make fmt
```

## Regenerating schemas

```sh
make codegen
```

This runs `iuliia-codegen`, reads JSON definitions from the `schemas/` submodule, and writes generated `.rs` files into `iuliia/src/schemas/`.

## CI

```sh
make ci
```

CI runs `fmt-check`, `lint` (clippy), and `test` through `make ci`. The workflow runs on push and pull request events targeting `master`.

## Publishing

Publishing is handled by the GitHub Release workflow. A release triggers `make publish`, which publishes the `iuliia` crate with the `CARGO_REGISTRY_TOKEN` secret.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT


