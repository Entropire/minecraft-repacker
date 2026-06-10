use std::collections::HashMap;
use std::error::Error;
use std::collections::BTreeSet;

pub struct PackVersions{
    version_map: HashMap<String, u32>,
    pack_formats: Vec<u32>
}

impl PackVersions{
    pub fn from_json(json_str: &str) -> Result<PackVersions, Box<dyn Error>>{
        let version_map: HashMap<String, u32> = serde_json::from_str(json_str)?;
        let pack_formats: Vec<u32> = version_map
            .values()
            .copied()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        Ok(PackVersions { version_map: (version_map), pack_formats: (pack_formats) })
    }

    pub fn get_pack_format(&self, version: &str) -> Option<u32> {
        self.version_map.get(version).copied()
    }

    pub fn next_pack_format(&self, current: u32) -> Option<u32> {
        self.pack_formats
            .iter()
            .copied()
            .find(|&p| p > current)
    }
}