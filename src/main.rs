use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::ops::Deref;
use std::str;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value = "data")]
    data_path: String,
    #[arg(short, long, default_value = "output")]
    output_path: String,
    #[arg(short, long, value_enum, default_value = "DEBUG")]
    log_level: tracing::Level,
    #[arg(long, action)]
    log_json: bool,
}

#[derive(Clone, Debug, Deserialize)]
struct Currency {
    id: uuid::Uuid,
    name: String,
}

#[derive(Clone, Debug, Deserialize)]
struct Source {
    id: String,
    name: String,
    homepage: url::Url,
}

mod log;

fn main() {
    let args = Args::parse();
    log::setup(args.log_level);
    tracing::info!("Reading data from: {}", args.data_path);

    for currency in load_currences(&args.data_path) {
        println!("Name: {:?}", currency);
    }
    for source in load_sources(&args.data_path) {
        println!("Name: {:?}", source);
    }
}

fn load_currences(base_dir: &String) -> impl Iterator<Item = Currency> {
    let dir = format!("{}/currencies", base_dir);
    let msg = format!("Expected directory: {}", dir);
    let paths = fs::read_dir(dir).expect(&msg);
    paths.map(|path| {
        let p = path.unwrap().path().deref().to_owned();
        let m = format!("Parsing file {:?}", p);
        let d = fs::read_to_string(&p).expect("reading file");
        toml::from_str(&d).expect(&m)
    })
}

fn load_sources(base_dir: &String) -> impl Iterator<Item = Source> {
    let dir = format!("{}/sources", base_dir);
    let msg = format!("Expected directory: {}", dir);
    let paths = fs::read_dir(dir).expect(&msg);
    paths.map(|path| {
        let p = path.unwrap().path().deref().to_owned();
        let m = format!("Parsing file {:?}", p);
        let d = fs::read_to_string(&p).expect("reading file");
        toml::from_str(&d).expect(&m)
    })
}
