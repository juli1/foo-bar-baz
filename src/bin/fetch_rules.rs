use core::model;
use serde::{Deserialize, Serialize};
use std::env;

pub use model::Configfile;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct ApiResponseRuleset {
    pub attributes: ApiResponseRulesetAttributes,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct ApiResponseRulesetAttributes {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct ApiResponse {
    pub data: ApiResponseRuleset,
}

fn main() {
    let site = env::var("DD_SITE").expect("specify DD_SITE variable");
    let app_key = env::var("DD_APP_KEY").expect("specify DD_APP_KEY variable");
    let api_key = env::var("DD_API_KEY").expect("specify DD_API_KEY variable");
    let url = format!(
        "https://api.{}/api/v2/static-analysis/rulesets/python-security",
        site
    );
    // jsonapi_model!(Rulesets; "rulesets");

    let client = reqwest::blocking::Client::new();
    let resp200 = client
        .get(url)
        .header("Content-Type", "application/json")
        .header("dd-api-key", api_key)
        .header("dd-application-key", app_key)
        .send()
        .unwrap()
        .text();
    let s = resp200.expect("should get content from server");
    println!("Value: {}", &s);
    let data: Result<ApiResponse, serde_json::Error> = serde_json::from_str(s.as_str());

    println!("Value: {:?}", data)
}
