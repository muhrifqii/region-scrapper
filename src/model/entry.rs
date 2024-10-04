use serde::{Deserialize, Serialize};

use super::level::Level;

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub kode_bps: String,
    pub nama_bps: String,
    pub nama_pos: String,
    pub kode_pos: String,
}

#[derive(Debug, Deserialize)]
pub struct SimpleEntry {
    pub kode: String,
    pub nama: String,
}

#[derive(Debug, Serialize)]
pub struct MappedEntry {
    pub code: String,
    pub parent_code: String,
    pub name: String,
    pub postal_code: String,
    pub level: Level,
}

impl MappedEntry {
    pub fn from_entry(entry: &Entry, parent: &str, level: &Level) -> Self {
        Self {
            code: entry.kode_bps.clone(),
            parent_code: parent.to_string(),
            name: entry.nama_bps.clone(),
            postal_code: entry.kode_pos.clone(),
            level: level.clone(),
        }
    }

    pub fn from_simple_entry(entry: &SimpleEntry, parent: &str, level: &Level) -> Self {
        Self {
            code: entry.kode.clone(),
            parent_code: parent.to_string(),
            name: entry.nama.clone(),
            postal_code: "".to_string(),
            level: level.clone(),
        }
    }
}
