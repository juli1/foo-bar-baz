use std::path::PathBuf;

fn main() {
    let javascript_dir: PathBuf = ["tree-sitter-javascript", "src"].iter().collect();
    let python_dir: PathBuf = ["tree-sitter-python", "src"].iter().collect();

    cc::Build::new()
        .include(&javascript_dir)
        .file(javascript_dir.join("parser.c"))
        .file(javascript_dir.join("scanner.c"))
        .warnings(false)
        .compile("tree-sitter-python");
    cc::Build::new()
        .include(&python_dir)
        .file(python_dir.join("parser.c"))
        .file(python_dir.join("scanner.cc"))
        .warnings(false)
        .compile("tree-sitter-python");
}
