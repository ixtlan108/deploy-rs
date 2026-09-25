use clap::Parser;
use std::error::Error;
use std::fs;
use yaml_rust2::YamlLoader;

use md5::Digest;

fn calc_md5(path: &str) -> Result<Digest, Box<dyn Error>> {
    let contents = fs::read(path)?; //.expect("Failed to read file");
    let digest = md5::compute(&contents);
    Ok(digest)

    //println!("short hash: {}", &format!("{:x}", digest)[..8]);
    //println!("{:x}\t{}", digest, path);
    // let digest = md5::compute(b"hello");
    // let short = format!("{:x}", digest)[..8].to_string();
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

mod css {}

#[derive(Debug)]
struct Pkg(String);

#[derive(Debug)]
struct Module(String);

#[derive(Debug)]
pub struct PsHome(String);

#[derive(Debug)]
struct Config {
    pkg: Pkg,
    ps_home: PsHome,
    spago: spagom::Spago,
}

mod spagom {
    use super::{Config, Module, Pkg, PsHome};
    use std::error::Error;
    use std::process::Command;
    use yaml_rust2::Yaml;

    #[derive(Debug)]
    pub struct Out(String);
    impl Out {
        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    // impl PsHome {
    //     pub fn as_str(&self) -> &str {
    //         &self.0
    //     }
    // }

    #[derive(Debug)]
    pub struct Spago {
        pub module: Module,
        pub out: Out,
    }
    pub fn parse(doc: &Yaml) -> Result<Spago, Box<dyn Error>> {
        let module = doc["module"].as_str().unwrap();
        let out = doc["out"].as_str().unwrap();
        let result = Spago {
            module: Module(String::from(module)),
            out: Out(String::from(out)),
        };
        Ok(result)
    }
    /*
    pub fn build(cfg: &Spago, ps_home: &PsHome) -> Result<(), Box<dyn Error>> {
        Command::new("spago")
            .arg("build")
            .arg("--package")
            .arg(&cfg.pkg.0)
            .current_dir(&ps_home.0)
            .status()?;
        Ok(())
    }
    */
    pub fn bundle(cfg: &Config) -> Result<(), Box<dyn Error>> {
        let s_cfg = &cfg.spago;
        Command::new("spago")
            .arg("bundle")
            .arg("--quiet")
            .arg("--package")
            .arg(&cfg.pkg.0)
            .arg("--module")
            .arg(&s_cfg.module.0)
            .arg("--outfile")
            .arg(&s_cfg.out.0)
            .current_dir(&cfg.ps_home.0)
            .status()?;
        Ok(())
    }
}

fn parse_yaml(yaml: &str) -> Result<Config, Box<dyn Error>> {
    let yaml_content = fs::read_to_string(yaml).expect("Failed to read yaml file");

    let docs = YamlLoader::load_from_str(&yaml_content)?;
    let spago_doc = &docs[0];

    let spago = spagom::parse(spago_doc)?;
    let ps_home = spago_doc["ps-home"].as_str().unwrap();

    let pkg = spago_doc["pkg"].as_str().unwrap();
    let result = Config {
        pkg: Pkg(String::from(pkg)),
        ps_home: PsHome(String::from(ps_home)),
        spago: spago,
    };

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

        assert_eq!("report-app", spago.module.0);
        assert_eq!(
            "/home/rcs/opt/klaxton/PhotoAppMVC/Purescript",
            config.ps_home.0
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
