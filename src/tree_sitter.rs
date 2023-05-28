use tree_sitter::{Parser, Language, Query, QueryCursor};


pub fn test_tree_sitter() {

    extern "C" { fn tree_sitter_python() -> Language; }
    let mut parser = Parser::new();
    let language = unsafe { tree_sitter_python() };
    parser.set_language(language).unwrap();

    let source_code = r#"
arr = ["foo", "bar"];

def func():
   pass;"#;
    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();

    println!("root node: {}", root_node.kind());
    println!("s-expression: {}", root_node.to_sexp());

    let query = r#"
(function_definition
  name: (identifier) @class_name
  body: (block) @the_body

  (#eq? @class_name "func")

) @functiondef
    "#;

    let q = Query::new(language, query).expect("woeijf");

    let mut cursor = QueryCursor::new();
    let res = cursor.matches(&q, root_node, source_code.as_bytes());

    for m in res {

        for c in m.captures.iter() {
            let name = q.capture_names().get(usize::try_from(c.index).unwrap());
            let range = c.node.range();
            let related_code = &source_code[range.start_byte..range.end_byte];
            println!(
                "[{}:{}-{}:{} {}] Offending source code: `{}`",
                range.start_point.row, range.start_point.column,
                range.end_point.row, range.end_point.column,
                name.unwrap(), related_code,
            );
        }
    }
}
