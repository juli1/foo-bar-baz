use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub enum RuleType {
    TreeSitter,
    Pattern,
}

#[derive(Deserialize, Debug, Serialize)]
pub enum Language {
    PYTHON,
    JAVASCRIPT,
}

#[derive(Deserialize, Debug, Serialize)]
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

#[derive(Deserialize, Debug, Serialize)]
pub struct Configfile {
    pub rulesets: Vec<String>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Request {
    pub filename: String,
    pub rules: Vec<Rule>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Response {
    pub error: Option<String>,
    pub violations: Vec<Violation>,
}
