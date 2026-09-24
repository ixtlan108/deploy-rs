use clap::Parser;
use std::error::Error;
use std::fs;
use std::process::Command;
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

mod spagom {
    use std::error::Error;
    use yaml_rust2::Yaml;

    #[derive(Debug)]
    pub struct Spago {
        pub pkg: String,
    }
    pub fn parse(doc: &Yaml) -> Result<Spago, Box<dyn Error>> {
        let result = Spago {
            pkg: String::from("Demo"),
        };
        Ok(result)
    }
}

#[derive(Debug)]
struct Config {
    spago: spagom::Spago,
}

fn parse_yaml(yaml: &str) -> Result<Config, Box<dyn Error>> {
    let yaml_content = fs::read_to_string(yaml).expect("Failed to read yaml file");

    let docs = YamlLoader::load_from_str(&yaml_content)?;
    let doc = &docs[0];

    /*
    let spago = spago::Spago {
        pkg: String::from("reports"),
    };
    */
    let spago = spagom::parse(doc)?;

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

    // Run a command inside a specific directory without changing your Rust app's global state
    Command::new("spago")
        .arg("build")
        .arg("--package")
        .arg("report-app")
        .current_dir("/Users/zeus/Projects/PhotoAppMVC/Purescript")
        .status()?;

    Ok(())
}
