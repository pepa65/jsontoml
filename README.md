# jsontoml 0.2.1
**Convert a JSON file to TOML on the CLI**
* Repo: https://github.com/pepa65/jsontoml
* After [json2toml](https://github.com/voidei/json2toml)
* Inspired heavily by [toml2json](https://github.com/woodruffw/toml2json/)

## Usage
```
jsontoml 0.2.0 - Convert a JSON file to TOML on the CLI
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
