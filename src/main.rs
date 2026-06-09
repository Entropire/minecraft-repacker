use clap::{Parser};
use std::{fs, process};
use std::error::Error;
use std::path::PathBuf;
use serde_json::{Value};
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    pack_path: String,
}

#[derive(Debug, Deserialize)]
struct PackData {
    pub description: String,
    pub pack_format: Option<u32>,
    pub min_format: Option<u32>,
    pub max_format: Option<u32>,
}

fn main() {
    let args = Args::parse();

    let pack_data = fetch_pack_data(&args.pack_path).unwrap_or_else(|err| {
        eprint!("{}", err);
        process::exit(1)
    });

    println!("{:?}", pack_data.pack_format)
}

fn fetch_pack_data(folder_path: &str) -> Result<PackData, Box<dyn Error>>{
    let mut path = PathBuf::from(folder_path);
    path.push("pack.mcmeta");
    let json_str = fs::read_to_string(path)?;
    let value: serde_json::Value = serde_json::from_str(&json_str)?;
    let pack: PackData = serde_json::from_value(value["pack"].clone())?;

    Ok(pack)
}