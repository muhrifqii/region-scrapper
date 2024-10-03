use std::borrow::Borrow;

use serde::Deserialize;

use super::level::Level;

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub kode_bps: String,
    pub nama_bps: String,
    pub kode_pos: String,
}

#[derive(Debug)]
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
}
