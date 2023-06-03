use core::model;
use rocket::serde::json::{json, Json, Value};

// Example to use curl -X POST http://localhost:8000/analyze -H "Content-Type: application/json" -d '{"filename": "myfile.rs", "rules": []}'

#[rocket::post("/analyze", format = "application/json", data = "<request>")]
fn analyze(request: Json<model::Request>) -> Value {
    println!("request: {:?}", &request);
    let response: model::Response = model::Response {
        violations: Vec::new(),
        error: Option::None,
    };
    json!(&response)
}

#[rocket::launch]
fn rocket_main() -> _ {
    rocket::build().mount("/", rocket::routes![analyze])
}
