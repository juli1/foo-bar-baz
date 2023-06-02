use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub enum RuleType {
    TreeSitter,
    Pattern,
}

#[derive(Deserialize, Debug)]
pub enum Language {
    PYTHON,
    JAVASCRIPT,
}

#[derive(Deserialize, Debug)]
pub struct Rule {
    name: String,
    code: String,
    pattern: Option<String>,
    rule_type: RuleType,
    tree_sitter_query: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Violation {
    pub start: Position,
    pub end: Position,
    pub description: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Position {
    pub line: u32,
    pub col: u32,
}
