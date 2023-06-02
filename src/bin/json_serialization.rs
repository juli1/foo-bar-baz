use core::model;
pub use model::{Position, Rule, Violation};
fn main() {
    let violation = Violation {
        description: "foobar".to_string(),
        start: Position { line: 1, col: 1 },
        end: Position { line: 4, col: 42 },
    };

    let res = serde_json::to_string(&violation).expect("should serialize");

    println!("rules: {:#?}", res);
}
