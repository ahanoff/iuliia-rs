# Contributing to iuliia-rs

Thanks for your interest in contributing. This document covers the essentials for getting started.

## Prerequisites

- Rust stable toolchain
- `make`
- Git with submodule support

## Quick start

1. Fork the repository.
2. Clone your fork and initialize the schema submodule:

   ```sh
   git clone --recurse-submodules <your-fork-url>
   cd iuliia-rs
   ```

   If you already cloned without submodules:

   ```sh
   git submodule update --init
   ```

3. Build and test:

   ```sh
   make build
   make test
   ```

## Development workflow

Run the full check suite before opening a pull request:

```sh
make ci
```

This runs `fmt-check`, `lint` (clippy with warnings as errors), and `test` — the same commands the CI pipeline executes.

### Individual commands

| Command | What it does |
| --- | --- |
| `make build` | Compile the workspace |
| `make test` | Run all tests |
| `make lint` | Run clippy with warnings as errors |
| `make fmt` | Format all code |
| `make fmt-check` | Check formatting without writing |
| `make ci` | Run `fmt-check` + `lint` + `test` |

## Workspace structure

- `iuliia/` — The library crate (`iuliia` on crates.io). Zero dependencies.
- `iuliia-codegen/` — Dev tool that reads JSON schema definitions and generates Rust modules.
- `schemas/` — Git submodule pointing to [nalgeon/iuliia](https://github.com/nalgeon/iuliia) for upstream schema JSON files.

## Adding a new transliteration schema

Schema definitions come from the upstream [nalgeon/iuliia](https://github.com/nalgeon/iuliia) repository, tracked via the `schemas/` submodule. Do **not** hand-edit generated files in `iuliia/src/schemas/`.

1. If the schema already exists upstream, update the submodule pointer to a newer revision. If it does not exist yet, propose the schema upstream first, then update this repository once it is available.
2. Regenerate the Rust modules:

   ```sh
   make codegen
   ```

3. Run the test suite to verify the new schema:

   ```sh
   make test
   ```

4. Commit both the updated `schemas/` submodule pointer and the regenerated files under `iuliia/src/schemas/`.

## Pull request guidelines

- Keep changes focused. One concern per PR.
- Run `make ci` locally before pushing. CI failures slow down review.
- Add tests for any new behavior. The generated schemas already include sample-based tests from upstream JSON.
- Do not modify generated files under `iuliia/src/schemas/` directly. Run `make codegen` instead.
- Include regenerated files in your PR when changing schemas or codegen logic.
- If your PR changes the public API, update the crate README (`iuliia/README.md`) accordingly.

## Reporting issues

When filing a bug, please include:

- The input string.
- The schema name.
- The actual output.
- The expected output.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
