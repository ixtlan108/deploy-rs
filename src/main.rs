use clap::Parser;
use std::error::Error;
use std::fs;
use yaml_rust2::{Yaml, YamlLoader};

#[derive(Debug, Clone, Parser)]
#[command(author, version, about)]
/// Deployment Spago and Css
struct Args {
    /// Mac Os
    #[arg(short, long, default_value_t = false)]
    macos: bool,

    /// Yaml Project File
    #[arg(long, required(true))]
    yaml: String,
}

#[derive(Debug)]
struct Spago {
    pkg: String,
}

#[derive(Debug)]
struct Config {
    spago: Spago,
}

fn parse_spago(doc: &Yaml) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn parse_yaml(yaml: &str) -> Result<Config, Box<dyn Error>> {
    let yaml_content = fs::read_to_string(yaml).expect("Failed to read yaml file");

    let docs = YamlLoader::load_from_str(&yaml_content)?;
    let doc = &docs[0];

    let spago = Spago {
        pkg: String::from("reports"),
    };

    let result = Config { spago: spago };

    Ok(result)
    /*
    let main_action = doc["main-action"].as_str().unwrap();
    let module = doc["module"].as_str().unwrap();
    let html = doc["html"].as_str().unwrap();
    let src_path = doc["src-path"].as_str().unwrap();

    let result = Config {
        main_action: String::from(main_action),
        module: String::from(module),
        html: String::from(html),
        src_path: String::from(src_path),
    };
    */
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    Ok(())
}
