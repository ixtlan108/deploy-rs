use clap::Parser;
use std::error::Error;
use std::fs;
//use std::marker::PhantomData;
use yaml_rust2::YamlLoader;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about)]
/// Deployment Spago and Css
struct Args {
    /// Yaml Project File
    #[arg(index = 1)]
    yaml: String,

    /// Spago Bundle
    #[arg(short, long, default_value_t = false)]
    spago: bool,

    /// Spago build
    #[arg(short, long, default_value_t = false)]
    build: bool,

    /// Css
    #[arg(short, long, default_value_t = false)]
    css: bool,

    /// Thymeleaf
    #[arg(short, long, default_value_t = false)]
    thymeleaf: bool,

    /// Janet
    #[arg(short, long, default_value_t = false)]
    janet: bool,
}

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
pub struct Tpl(String);

#[derive(Debug)]
pub struct TplPath(String);

#[derive(Debug)]
pub struct TplTarget(String);

#[derive(Debug)]
pub struct ScssHome(String);

#[derive(Debug)]
struct Config {
    pkg: Pkg,
    ps_home: PsHome,
    stem: Stem,
    java_res_home: JavaResourceHome,
    spago: spagom::Spago,
    tpl: Tpl,
    tpl_path: TplPath,
    tpl_target: TplTarget,
    scss_home: ScssHome,
}

impl Config {
    pub fn new(
        pkg: Pkg,
        ps_home: PsHome,
        stem: Stem,
        java_res_home: JavaResourceHome,
        spago: spagom::Spago,
        tpl: Tpl,
        tpl_path: TplPath,
        tpl_target: TplTarget,
        scss_home: ScssHome,
    ) -> Self {
        Config {
            pkg: pkg,
            ps_home: ps_home,
            stem: stem,
            java_res_home: java_res_home,
            spago: spago,
            tpl: tpl,
            tpl_path: tpl_path,
            tpl_target: tpl_target,
            scss_home: scss_home,
        }
    }
}

mod spagom {
    use super::{Args, Config, Module};
    use std::error::Error;
    use std::process::Command;
    use yaml_rust2::Yaml;

    pub fn out_file(cfg: &Config) -> String {
        format!("{}/{}/dist/{}.js", &cfg.ps_home.0, &cfg.pkg.0, &cfg.stem.0)
    }

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
    pub fn build(cfg: &Config) -> Result<(), Box<dyn Error>> {
        Command::new("spago")
            .arg("build")
            .arg("--package")
            .arg(&cfg.pkg.0)
            .current_dir(&cfg.ps_home.0)
            .status()?;
        Ok(())
    }
    pub fn run(cfg: &Config, args: &Args) -> Result<(), Box<dyn Error>> {
        if args.build == true {
            build(cfg)?;
        } else if args.spago == true {
            bundle(cfg)?;
        }
        Ok(())
    }
}

mod css {
    use super::{Args, Config};
    use std::error::Error;
    use std::fs::File;
    use std::io::Write;
    use std::io::{BufRead, BufReader};

    pub fn out_file(cfg: &Config) -> String {
        format!("{}/{}/dist/{}.css", &cfg.ps_home.0, &cfg.pkg.0, &cfg.stem.0)
    }

    fn scss_file_name(cfg: &Config) -> String {
        let stem = &cfg.stem.0;
        format!("{}/{}/{}.scss", &cfg.scss_home.0, stem, stem)
    }

    fn import_file_name(cfg: &Config, file_name: &str) -> String {
        format!("{}/{}.css", &cfg.scss_home.0, file_name)
    }

    fn local_import_file_name(cfg: &Config, file_name: &str) -> String {
        format!("{}/{}/{}.css", &cfg.scss_home.0, &cfg.stem.0, file_name)
    }
    // fn first_word_is_import(s: &str) -> bool {
    //     s.split_whitespace().next() == Some("import")
    // }

    fn handle_import<W: Write>(
        f: &mut W,
        yaml_line: &str,
        cfg: &Config,
    ) -> Result<(), Box<dyn Error>> {
        let file_name = yaml_line.split_whitespace().nth(1).unwrap();
        let file_name = import_file_name(cfg, file_name);
        writeln!(f, "/* {} */", file_name)?;
        let import = File::open(file_name)?;
        let lines: Vec<String> = BufReader::new(import)
            .lines()
            .map(|l| l.expect("could not read line"))
            .collect();

        for line in &lines {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }

    fn handle_local_import<W: Write>(
        f: &mut W,
        yaml_line: &str,
        cfg: &Config,
    ) -> Result<(), Box<dyn Error>> {
        let file_name = yaml_line.split_whitespace().nth(1).unwrap();
        let file_name = local_import_file_name(cfg, file_name);
        writeln!(f, "/* {} */", file_name)?;
        let import = File::open(file_name)?;
        let lines: Vec<String> = BufReader::new(import)
            .lines()
            .map(|l| l.expect("could not read line"))
            .collect();

        for line in &lines {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }

    fn generate_css(cfg: &Config) -> Result<(), Box<dyn Error>> {
        let scss = scss_file_name(cfg);
        println!("scss file: {}", &scss);
        let scss = File::open(&scss)?; //.expect("file not found");

        let scss_lines: Vec<String> = BufReader::new(scss)
            .lines()
            .map(|l| l.expect("could not read line"))
            .collect();

        let mut out = File::create(&out_file(cfg))?;

        for line in &scss_lines {
            println!("{}", line);
            if line.trim().starts_with("import") {
                handle_import(&mut out, &line, cfg)?;
            } else if line.trim().starts_with("local-import") {
                handle_local_import(&mut out, &line, cfg)?;
            } else {
                writeln!(out, "{}", line)?;
            }
        }

        // let lines: Vec<String> = fs::read_to_string("file.css")
        //     .expect("file not found")
        //     .lines()
        //     .map(String::from)
        //     .collect();

        Ok(())
    }
    pub fn run(cfg: &Config, args: &Args) -> Result<(), Box<dyn Error>> {
        if args.css == true {
            generate_css(cfg)?;
        }
        Ok(())
    }
}

mod thymeleaf {
    use super::{Args, Config, css, spagom};
    //use md5::Digest;
    use std::error::Error;
    use std::fs;
    use std::fs::File;

    use minijinja::{Environment, context};

    fn calc_md5(path: &str) -> Result<String, Box<dyn Error>> {
        let contents = fs::read(path)?; //.expect("Failed to read file");
        let digest = md5::compute(&contents);
        let short = format!("{:x}", digest)[..8].to_string();
        Ok(short)

        //println!("short hash: {}", &format!("{:x}", digest)[..8]);
        //println!("{:x}\t{}", digest, path);
        // let digest = md5::compute(b"hello");
        // let short = format!("{:x}", digest)[..8].to_string();
    }

    fn tpl_target_file_name(cfg: &Config) -> String {
        format!("{}/index.html", &cfg.tpl_target.0)
    }

    pub fn js_target_file_name(cfg: &Config, md5: &str) -> String {
        let stem = &cfg.stem.0;
        format!(
            "{}/static/js/{}/{}-{}.js",
            &cfg.java_res_home.0, stem, stem, md5
        )
    }

    pub fn css_target_file_name(cfg: &Config, md5: &str) -> String {
        let stem = &cfg.stem.0;
        format!(
            "{}/static/css/{}/{}-{}.js",
            &cfg.java_res_home.0, stem, stem, md5
        )
    }

    pub fn run(cfg: &Config, args: &Args) -> Result<(), Box<dyn Error>> {
        if args.thymeleaf == true {
            let mut env = Environment::new();

            env.set_loader(minijinja::path_loader(&cfg.tpl_path.0));

            let spago_out = spagom::out_file(cfg);
            let css_out = css::out_file(cfg);

            let md5_js = calc_md5(&spago_out)?;
            let md5_css = calc_md5(&css_out)?;

            let template = env.get_template(&cfg.tpl.0).unwrap();

            let index_html = File::create(tpl_target_file_name(cfg))?;

            template.render_captured_to(
                context! { md5_js => &md5_js, md5_css => &md5_css },
                index_html,
            )?;

            let target_js = js_target_file_name(cfg, &md5_js);

            let _ = fs::copy(&spago_out, &target_js);

            let target_css = css_target_file_name(cfg, &md5_js);

            let _ = fs::copy(&css_out, &target_css);

            //template.render_captured_to(context!(name => "World"), index_html)?;
            // println!(
            //     "{}",
            //     template.render(context! { md5_js => &md5_js }).unwrap()
            // );
        }
        Ok(())
    }
}

fn parse_yaml(yaml: &str) -> Result<Config, Box<dyn Error>> {
    let yaml_content = fs::read_to_string(yaml).expect("Failed to read yaml file");

    let docs = YamlLoader::load_from_str(&yaml_content)?;
    let cfg_doc = &docs[0];
    let spago_doc = &docs[1];
    let css_doc = &docs[2];
    let tpl_doc = &docs[3];

    let spago = spagom::parse(spago_doc)?;

    let ps_home = cfg_doc["ps-home"].as_str().unwrap();
    let pkg = cfg_doc["pkg"].as_str().unwrap();
    let stem = cfg_doc["stem"].as_str().unwrap();
    let java_res = cfg_doc["java-resources"].as_str().unwrap();
    let tpl = tpl_doc["tpl"].as_str().unwrap();
    let tpl_path = tpl_doc["tpl-path"].as_str().unwrap();
    let tpl_target = tpl_doc["tpl-target"].as_str().unwrap();
    let scss_home = css_doc["scss-home"].as_str().unwrap();

    let result = Config::new(
        Pkg(String::from(pkg)),
        PsHome(String::from(ps_home)),
        Stem(String::from(stem)),
        JavaResourceHome(String::from(java_res)),
        spago,
        Tpl(String::from(tpl)),
        TplPath(String::from(tpl_path)),
        TplTarget(String::from(tpl_target)),
        ScssHome(String::from(scss_home)),
    );

    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let config = parse_yaml(&args.yaml)?;
    spagom::run(&config, &args)?;
    thymeleaf::run(&config, &args)?;
    css::run(&config, &args)?;

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
    fn test_js_target_file_name() -> Result<(), Box<dyn Error>> {
        let cfg = test_config();
        let result = thymeleaf::js_target_file_name(&cfg, "12345678");
        assert_eq!(
            "/home/rcs/opt/klaxton/PhotoAppMVC/src/main/resources/static/js/report/report-12345678.js",
            result
        );
        Ok(())
    }

    #[test]
    fn test_parse_input() -> Result<(), Box<dyn Error>> {
        let config = parse_yaml("tests/resources/reports.yaml")?;
        let spago = config.spago;

        assert_eq!(
            "/home/rcs/opt/klaxton/PhotoAppMVC/src/main/resources",
            config.java_res_home.0
        );
        assert_eq!("report-app", config.pkg.0);
        assert_eq!("report", config.stem.0);
        assert_eq!("ReportMain", spago.module.0);
        assert_eq!(
            "/home/rcs/opt/klaxton/PhotoAppMVC/Purescript",
            config.ps_home.0
        );
        assert_eq!("index.html.tpl", config.tpl.0);
        assert_eq!(
            "/home/rcs/opt/klaxton/PhotoAppMVC/Purescript/report-app/tpl",
            config.tpl_path.0
        );
        assert_eq!(
            "/home/rcs/opt/klaxton/PhotoAppMVC/src/main/resources/templates/report",
            config.tpl_target.0
        );

        assert_eq!(
            "/home/rcs/opt/klaxton/PhotoAppMVC/sass-src",
            config.scss_home.0
        );

        Ok(())
    }

    fn test_config() -> Config {
        let spago = spagom::Spago {
            module: Module(String::from("module")),
        };
        Config::new(
            Pkg(String::from("pkg")),
            PsHome(String::from("ps_home")),
            Stem(String::from("report")),
            JavaResourceHome(String::from(
                "/home/rcs/opt/klaxton/PhotoAppMVC/src/main/resources",
            )),
            spago,
            Tpl(String::from("tpl")),
            TplPath(String::from("tpl_path")),
            TplTarget(String::from("tpl_target")),
            ScssHome(String::from("scss_home")),
        )
    }
}
/*
let line = "import global/colors";

// Option 1: split on whitespace, take the second token
let path = line.split_whitespace().nth(1).unwrap(); // "global/colors"

// Option 2: split on the first space only
let path = line.split_once(' ').map(|(_, rest)| rest).unwrap(); // "global/colors"

// Option 3: strip a known prefix
let path = line.strip_prefix("import ").unwrap(); // "global/colors"
*/
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
