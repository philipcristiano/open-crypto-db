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
struct CurrencyDefinition {
    id: uuid::Uuid,
    name: String,
    sources: std::collections::HashMap<String, CurrencySourceInfo>,
}

#[derive(Clone, Debug, Serialize)]
struct Currency<'a> {
    id: &'a uuid::Uuid,
    name: &'a String,
}

impl CurrencyDefinition {
    fn to_currency<'a>(self: &'a CurrencyDefinition) -> Currency {
        Currency {
            id: &self.id,
            name: &self.name,
        }
    }
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

struct SourceRegistry {
    data: std::collections::HashMap<String, Source>,
}

impl SourceRegistry {
    fn new() -> Self {
        SourceRegistry {
            data: std::collections::HashMap::new(),
        }
    }
    fn add_new(&mut self, s: Source) -> anyhow::Result<()> {
        let k = s.id.clone();

        if self.data.contains_key(&k) {
            return Err(anyhow::anyhow!("Source already set"));
        } else {
            self.data.insert(s.id.clone(), s);
            return Ok(());
        }
    }

    fn get_by_source_id(&self, sid: &String) -> Option<&Source> {
        self.data.get(sid)
    }
}

mod log;

fn main() {
    let args = Args::parse();
    log::setup(args.log_level);
    fs::create_dir_all(&args.output_path).expect("Creating output dir");
    let currencies_dir = format!("{}/currencies", &args.output_path);
    fs::create_dir_all(currencies_dir).expect("Creating output dir");
    tracing::info!("Reading data from: {}", args.data_path);

    let mut source_registry = SourceRegistry::new();

    for source in load_sources(&args.data_path) {
        println!("Name: {:?}", source);
        let m = format!("Source {} already defined", source.id);
        source_registry.add_new(source).expect(&m);
    }

    for currency_definition in load_currences(&args.data_path) {
        println!("Name: {:?}", currency_definition);
        let currency: Currency = currency_definition.to_currency();
        write_currency(&args.output_path, &currency);
        for (s, info) in currency_definition.sources.iter() {
            println!("Currenty for source: {:?}", s);
            let m = format!("Source should be defined: {}", s);
            let source = source_registry.get_by_source_id(&s).expect(&m);
            write_source_currency(&args.output_path, &source, &info, &currency)
        }
    }
}

fn load_currences(base_dir: &String) -> impl Iterator<Item = CurrencyDefinition> {
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

fn write_source_currency(path: &String, s: &Source, si: &CurrencySourceInfo, c: &Currency) {
    let currency_dir_path = format!("{path}/sources/{}/currencies/", s.id);
    let full_path = format!("{currency_dir_path}/{}", si.id);
    fs::create_dir_all(&currency_dir_path).expect("Creating output dir");

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
