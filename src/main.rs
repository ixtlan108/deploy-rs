use clap::Parser;
use std::error::Error;
use std::fs;
use yaml_rust2::{Yaml, YamlLoader};

use md5::Digest;

fn calc_md5(path: &str) -> Result<Digest, Box<dyn Error>> {
    let contents = fs::read(path)?; //.expect("Failed to read file");
    let digest = md5::compute(&contents);
    //println!("{:x}\t{}", digest, path);
    Ok(digest)
    //println!("short hash: {}", &format!("{:x}", digest)[..8]);
}

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
    use std::process::Command;
    use yaml_rust2::Yaml;

    #[derive(Debug)]
    pub struct Spago {
        pub module: String,
        pub ps_home: String,
    }
    pub fn parse(doc: &Yaml) -> Result<Spago, Box<dyn Error>> {
        let module = doc["module"].as_str().unwrap();
        let ps_home = doc["ps-home"].as_str().unwrap();
        let result = Spago {
            module: String::from(module),
            ps_home: String::from(ps_home),
        };
        Ok(result)
    }
    pub fn build(cfg: &Spago) -> Result<(), Box<dyn Error>> {
        Command::new("spago")
            .arg("build")
            .arg("--package")
            .arg(&cfg.module)
            .current_dir(&cfg.ps_home)
            .status()?;
        Ok(())
    }
}

#[derive(Debug)]
struct Config {
    spago: spagom::Spago,
}

fn parse_yaml(yaml: &str) -> Result<Config, Box<dyn Error>> {
    let yaml_content = fs::read_to_string(yaml).expect("Failed to read yaml file");

    let docs = YamlLoader::load_from_str(&yaml_content)?;
    let spago_doc = &docs[0];

    let spago = spagom::parse(spago_doc)?;

    let result = Config { spago: spago };

    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let config = parse_yaml("tests/resources/reports.yaml")?;
    // Run a command inside a specific directory without changing your Rust app's global state
    /*
    Command::new("spago")
        .arg("build")
        .arg("--package")
        .arg("report-app")
        .current_dir("/Users/zeus/Projects/PhotoAppMVC/Purescript")
        .status()?;
        */

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_input() -> Result<(), Box<dyn Error>> {
        let config = parse_yaml("tests/resources/reports.yaml")?;
        let spago = config.spago;

        assert_eq!("report-app", spago.module);
        assert_eq!(
            "/home/rcs/opt/klaxton/PhotoAppMVC/Purescript",
            spago.ps_home
        );
        Ok(())
    }
}

/*
use yaml_rust2::{yamlloader, yaml};

fn main() -> result<(), box<dyn std::error::error>> {
    let yaml_str = r#"
name: myapp
servers:
  - host: a
  - host: b
"#;

    // parse into a vec of documents (yaml can have multiple docs)
    let docs = yamlloader::load_from_str(yaml_str)?;
    let doc = &docs[0];

    // navigate the yaml enum ast
    let name = doc["name"].as_str().unwrap();
    let servers = doc["servers"].as_vec().unwrap();

    for server in servers {
        let host = &server["host"];
        println!("server: {}", host.as_str().unwrap());
    }

    ok(())
}
*/
