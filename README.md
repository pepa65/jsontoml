[![version](https://img.shields.io/crates/v/jsontoml.svg)](https://crates.io/crates/jsontoml)
[![build](https://github.com/pepa65/jsontoml/actions/workflows/ci.yml/badge.svg)](https://github.com/pepa65/jsontoml/actions/workflows/ci.yml)
[![dependencies](https://deps.rs/repo/github/pepa65/jsontoml/status.svg)](https://deps.rs/repo/github/pepa65/jsontoml)
[![docs](https://img.shields.io/badge/docs-jsontoml-blue.svg)](https://docs.rs/crate/jsontoml/latest)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/pepa65/jsontoml/blob/master/LICENSE)
[![downloads](https://img.shields.io/crates/d/jsontoml.svg)](https://crates.io/crates/jsontoml)

# jsontoml 0.2.2
**Convert a JSON file to TOML on the CLI**
* Repo: https://github.com/pepa65/jsontoml
* After [json2toml](https://github.com/voidei/json2toml)
* Inspired heavily by [toml2json](https://github.com/woodruffw/toml2json/)

## Usage
```
jsontoml 0.2.2 - Convert a JSON file to TOML on the CLI
Usage: jsontoml [input]
Arguments:
  [input]  JSON file to convert to TOML

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Example
* Input: `input.json`:
```json
{"key": "value", "key2": "value2"}
```
* Command: `jsontoml input.json >output.toml`
* Output: `output.toml`:
```toml
"key" = "value"
"key2" = "value2"
```
