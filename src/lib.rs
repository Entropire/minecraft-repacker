use std::fs;
use std::error::Error;
use std::path::PathBuf;
use serde::Deserialize;
use std::collections::HashMap;

const PACK_FORMATS_JSON: &str = include_str!("data/pack_formats.json");

pub fn repack(pack_path: &str, target_version: &str) -> Result<(), Box<dyn Error>>{
    let versions = PackVersions::from_json(PACK_FORMATS_JSON)?;

    let mut path = PathBuf::from(pack_path);
    path.push("pack.mcmeta");

    let pack_data = PackData::from_file(&path)?;


    Ok(())
}

#[derive(Debug, Deserialize)]
struct PackData {
    pub description: String,
    pub pack_format: Option<u32>,
    pub min_format: Option<u32>,
    pub max_format: Option<u32>,
}

impl PackData {
    fn from_file(path: &PathBuf)-> Result<PackData, Box<dyn Error>>{
        let json_str = fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json_str)?;
        let pack: PackData = serde_json::from_value(value["pack"].clone())?;
        Ok(pack)

    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct PackFormat {
    major: u32,
    minor: u32,
}

impl PackFormat {
    fn from_f64(v: f64) -> Self {
        Self {
            major: v.trunc() as u32,
            minor: ((v.fract() * 10.0).round()) as u32,
        }
    }
}

#[derive(Debug)]
struct PackVersions {
    version_map: HashMap<String, PackFormat>,
    pack_formats: Vec<PackFormat>,
}

impl PackVersions {
    fn from_json(json_str: &str) -> Result<Self, Box<dyn Error>> {
        let raw: HashMap<String, f64> = serde_json::from_str(json_str)?;

        let mut version_map = HashMap::new();
        let mut pack_formats: Vec<PackFormat> = Vec::new();

        for (version, value) in raw {
            let format = PackFormat::from_f64(value);
            version_map.insert(version, format);
            pack_formats.push(format);
        }

        pack_formats.sort_by(|a, b| {
            (a.major, a.minor).cmp(&(b.major, b.minor))
        });

        pack_formats.dedup_by(|a, b| {
            a.major == b.major && a.minor == b.minor
        });

        Ok(Self {
            version_map,
            pack_formats,
        })
    }

    fn get_pack_format(&self, version: &str) -> Option<PackFormat> {
        self.version_map.get(version).copied()
    }

    fn next_pack_format(&self, current: PackFormat) -> Option<PackFormat> {
        self.pack_formats
            .iter()
            .copied()
            .find(|p| (p.major, p.minor) > (current.major, current.minor))
    }
}