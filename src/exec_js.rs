use deno_core::op;
use deno_core::{Extension, RuntimeOptions, v8};
use deno_core::JsRuntime;

use serde_v8;

// execute a piece of JavaScript and return the result
pub fn eval(
    context: &mut JsRuntime,
    code: &'static str,
) -> Result<serde_json::Value, String> {
    let res = context.execute_script_static("<anon>", code);
    match res {
        Ok(global) => {
            let scope = &mut context.handle_scope();
            let local = v8::Local::new(scope, global);
            // Deserialize a `v8` object into a Rust type using `serde_v8`,
            // in this case deserialize to a JSON `Value`.
            let deserialized_value =
                serde_v8::from_v8::<serde_json::Value>(scope, local);

            match deserialized_value {
                Ok(value) => Ok(value),
                Err(err) => Err(format!("Cannot deserialize value: {err:?}")),
            }
        }
        Err(err) => Err(format!("Evaling error: {err:?}")),
    }
}

pub fn execute_rust_function() {

    #[op]
    fn op_panik(number: u64) {
        println!("from rust: {}", number);
    }

    let extensions = vec![Extension::builder("my_ext")
        .ops(vec![op_panik::decl()])
        .build()];
    let mut rt = JsRuntime::new(RuntimeOptions {
        extensions,
        ..Default::default()
    });
    rt.execute_script_static("panik", "Deno.core.ops.op_panik(42)")
        .unwrap();

    return;
}
