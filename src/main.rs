mod exec_js;
mod tree_sitter;

use deno_core::op;
use deno_core::v8;
use deno_core::Extension;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use exec_js::eval;

use serde_v8;
use crate::exec_js::execute_rust_function;
use crate::tree_sitter::test_tree_sitter;

fn return_str() -> String {
    let mut res = String::new();
    res.push_str("hello_world");
    res
}

fn js_test() {
    println!("{}", return_str());

    let code = "function foo(a) { return a + 10;} foo(32);";

    let mut runtime = JsRuntime::new(RuntimeOptions::default());
    let output: serde_json::Value =
        eval(&mut runtime, code).expect("Eval failed");

    println!("result: {}", output.to_string());

    execute_rust_function();
}

fn tree_sitter_test() {
    test_tree_sitter();
}


fn main() {

    tree_sitter_test();
}
