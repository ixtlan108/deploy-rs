use yaml_rust2::YamlLoader;

use crate::spago::Spago;
use std::error::Error;
use std::fs;

pub fn parse_yaml(yaml: &str) -> Result<Spago, Box<dyn Error>> {
    let yaml_content = fs::read_to_string(yaml).expect("Failed to read yaml file");

    let docs = YamlLoader::load_from_str(&yaml_content)?;
    let doc = &docs[0];
    let module = doc["module"].as_str().unwrap();

    let result = Spago::new(module);

    Ok(result)
}
