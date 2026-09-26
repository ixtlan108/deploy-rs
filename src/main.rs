use clap::Parser;
use std::error::Error;
use std::fs;
use std::marker::PhantomData;
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
    /// Yaml Project File
    #[arg(index = 1)]
    yaml: String,

    /// Spago
    #[arg(short, long, default_value_t = false)]
    spago: bool,

    /// Css
    #[arg(short, long, default_value_t = false)]
    css: bool,

    /// Janet
    #[arg(short, long, default_value_t = false)]
    janet: bool,
}

mod css {}

#[derive(Debug)]
struct Pkg(String);

#[derive(Debug)]
struct Module(String);

#[derive(Debug)]
pub struct PsHome(String);

#[derive(Debug)]
pub struct JavaResourceHome(String);

#[derive(Debug)]
pub struct Stem(String);

#[derive(Debug)]
pub struct Janet;

#[derive(Debug)]
pub struct Java;

#[derive(Debug)]
struct Config<State = Java> {
    pkg: Pkg,
    ps_home: PsHome,
    stem: Stem,
    java_res_home: JavaResourceHome,
    spago: spagom::Spago,
    state: PhantomData<State>,
}

impl Config {
    pub fn new(
        is_java: bool,
        pkg: Pkg,
        ps_home: PsHome,
        stem: Stem,
        java_res_home: JavaResourceHome,
        spago: spagom::Spago,
    ) -> Self {
        Config {
            pkg: pkg,
            ps_home: ps_home,
            stem: stem,
            java_res_home: java_res_home,
            spago: spago,
            state: if is_java { Java } else { Janet }, //Default::default(),
        }
    }
}
mod spagom {
    use super::{Config, Module};
    use std::error::Error;
    use std::process::Command;
    use yaml_rust2::Yaml;

    #[derive(Debug)]
    pub struct Spago {
        pub module: Module,
    }
    pub fn parse(doc: &Yaml) -> Result<Spago, Box<dyn Error>> {
        let module = doc["module"].as_str().unwrap();
        let result = Spago {
            module: Module(String::from(module)),
        };
        Ok(result)
    }
    fn out_file(cfg: &Config) -> String {
        format!("{}/{}/dist/{}.js", &cfg.ps_home.0, &cfg.pkg.0, &cfg.stem.0)
    }

    pub fn bundle(cfg: &Config) -> Result<(), Box<dyn Error>> {
        let s_cfg = &cfg.spago;
        let out = out_file(cfg);
        Command::new("spago")
            .arg("bundle")
            .arg("--quiet")
            .arg("--package")
            .arg(&cfg.pkg.0)
            .arg("--module")
            .arg(&s_cfg.module.0)
            .arg("--outfile")
            .arg(&out)
            .current_dir(&cfg.ps_home.0)
            .status()?;
        Ok(())
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
}

fn parse_yaml(yaml: &str) -> Result<Config<Java>, Box<dyn Error>> {
    let yaml_content = fs::read_to_string(yaml).expect("Failed to read yaml file");

    let docs = YamlLoader::load_from_str(&yaml_content)?;
    let cfg_doc = &docs[0];
    let spago_doc = &docs[1];

    let spago = spagom::parse(spago_doc)?;

    let ps_home = cfg_doc["ps-home"].as_str().unwrap();
    let pkg = cfg_doc["pkg"].as_str().unwrap();
    let stem = cfg_doc["stem"].as_str().unwrap();
    let java_res = cfg_doc["java-resources"].as_str().unwrap();

    let result = Config {
        pkg: Pkg(String::from(pkg)),
        ps_home: PsHome(String::from(ps_home)),
        stem: Stem(String::from(stem)),
        java_res_home: JavaResourceHome(String::from(java_res)),
        spago: spago,
        state: PhantomData::<Java>,
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
