To use before:

git clone https://github.com/tree-sitter/tree-sitter-python
git clone https://github.com/tree-sitter/tree-sitter-javascript

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

Using cargo-husky. Look at `Cargo.toml`. It has the following content that add a pre-push hook
and run clippy and cargo fmt.

```
[dev-dependencies.cargo-husky]
version = "1"
default-features = false # Disable features which are enabled by default
features = ["precommit-hook", "run-cargo-test", "run-cargo-clippy", "run-cargo-fmt"]
```