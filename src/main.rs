mod exec_js;
mod tree_sitter;

use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use exec_js::eval;

use crate::exec_js::execute_rust_function;
use crate::tree_sitter::test_tree_sitter;

#[allow(dead_code)]
fn return_str() -> String {
    let mut res = String::new();
    res.push_str("hello_world");
    res
}

#[allow(dead_code)]
fn js_test() {
    println!("{}", return_str());

    let code = "function foo(a) { return a + 10;} foo(32);";

    let mut runtime = JsRuntime::new(RuntimeOptions::default());
    let output: serde_json::Value = eval(&mut runtime, code).expect("Eval failed");

    println!("result: {}", output);

    execute_rust_function();
}

fn tree_sitter_test() {
    test_tree_sitter();
}

fn main() {
    tree_sitter_test();
}
