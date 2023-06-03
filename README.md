To use before:

git clone https://github.com/tree-sitter/tree-sitter-python
git clone https://github.com/tree-sitter/tree-sitter-javascript


## Mini projects

### TODO

 - Make multiple execution of a function into multiple threads
 - Getting rules from an API and serializing it
 - Reading data from the environment (env variables)
 - Proving the sandboxing mechanisms for JavaScript

### Done

 - Reading values from a JSON file and deserialize into a class
 - Separate the project between one or multiple common libraries and build multiple binaries
 - Parse tree-sitter code
 - Execute JavaScript with Deno
 - Reading a YAML file
 - Have a webserver taking JSON as input and sending back JSON


## Binaries

 - `exec_js`: execute JavaScript using Deno
 - `tree_sitter`: tree sitter query

## Build a binary

Use

```shell
cargo build --bin <target>

```

For example

```shell

cargo build --bin tree_sitter

```

If you want to build for production, use

```shell

cargo build -r --bin <target> 
```


## Project layout

Separation of library in a separate directory. Look at `core` for example for the core library.
This directory has its own `Cargo.toml` file to build the library and its dependencies.

Then, we have each binary in `src/bin`. These binaries are what is built.


## Pre-commit/push hooks

Using [cargo-husky](https://lib.rs/crates/cargo-husky).
Look at `Cargo.toml`. It has the following content that add a pre-push hook and run clippy and cargo fmt.

```
[dev-dependencies.cargo-husky]
version = "1"
default-features = false # Disable features which are enabled by default
features = ["precommit-hook", "run-cargo-test", "run-cargo-clippy", "run-cargo-fmt"]
```



