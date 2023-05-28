use std::path::PathBuf;

fn main() {
    let javaScriptDir: PathBuf = ["tree-sitter-javascript", "src"].iter().collect();
    let pythonDir: PathBuf = ["tree-sitter-python", "src"].iter().collect();

    cc::Build::new()
        .include(&javaScriptDir)
        .file(javaScriptDir.join("parser.c"))
        .file(javaScriptDir.join("scanner.c"))
        .compile("tree-sitter-python");


    cc::Build::new()
        .include(&pythonDir)
        .file(pythonDir.join("parser.c"))
        .file(pythonDir.join("scanner.cc"))
        .compile("tree-sitter-python");
}
