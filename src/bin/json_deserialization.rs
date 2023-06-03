use core::model;
pub use model::Rule;
fn main() {
    let data = r#"
    [
      {
        "name": "rule1",
        "code": "my-code",
        "pattern": "pattern",
        "rule_type": "PATTERN",
        "tree_sitter_query": "myquery"
      },
      {
        "name": "rule1",
        "code": "my-code",
        "pattern": null,
        "rule_type": "PATTERN",
        "tree_sitter_query": "myquery"
      }
    ]
    "#;
    let rules: Vec<Rule> = serde_json::from_str(data).expect("we should deserialize");
    println!("rules: {:#?}", rules);
}
