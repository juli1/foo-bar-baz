use core::model;
pub use model::Configfile;

fn main() {
    let data = r#"
rulesets:
  - python-security
    "#;
    let config_file: Configfile = serde_yaml::from_str(data).expect("config file parsed");
    println!("config file: {:#?}", config_file);
}
