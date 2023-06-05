use std::{env, vec};

fn main() {
    let path = env::var("PATH");
    let parts = match path {
        Ok(p) => p.split(':').map(|v| v.to_string()).collect(),
        Err(_) => vec::Vec::new(),
    };

    if parts.is_empty() {
        println!("no variable")
    } else {
        for p in &parts {
            println!("path element {:?}", p)
        }
    }
}
