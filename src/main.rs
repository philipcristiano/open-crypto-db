use clap::Parser;
use serde::{Deserialize, Serialize};
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

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Currency {
    id: uuid::Uuid,
    name: String,
    sources: std::collections::HashMap<String, CurrencySourceInfo>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct CurrencySourceInfo {
    id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Source {
    id: String,
    name: String,
    homepage: url::Url,
}

mod log;

fn main() {
    let args = Args::parse();
    log::setup(args.log_level);
    fs::create_dir_all(&args.output_path).expect("Creating output dir");
    let currencies_dir = format!("{}/currencies", &args.output_path);
    fs::create_dir_all(currencies_dir).expect("Creating output dir");
    tracing::info!("Reading data from: {}", args.data_path);

    for source in load_sources(&args.data_path) {
        println!("Name: {:?}", source);
    }

    for currency in load_currences(&args.data_path) {
        println!("Name: {:?}", currency);
        write_currency(&args.output_path, &currency);
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

fn write_currency(path: &String, c: &Currency) {
    let full_path = format!("{path}/currencies/{}", c.id);
    let m = format!("Writing file {}", full_path);
    let file = fs::File::create(full_path).expect(&m);
    let file = std::io::BufWriter::new(file);
    serde_json::to_writer(file, c).unwrap();
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
