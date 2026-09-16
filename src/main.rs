#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

pub mod config;
pub mod spago;

//use clap::{Arg, ArgAction, Command};
use clap::Parser;

// use std::error::Error;
// use std::fs;
// use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about)]
/// Purescript and Css Deployment
struct Args {
    /// Yaml Deployment File
    #[arg(long, required(true))]
    yaml: String,
}

fn main() {
    let args = Args::parse();
    //dbg!(args.clone());

    let cfg = config::parse_yaml(&args.yaml).unwrap();
}
