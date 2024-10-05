use crate::{
    model::{
        entry::{Entry, MappedEntry, SimpleEntry},
        level::Level,
    },
    util::client::handle_error,
};
use itertools::Itertools;
use log::{debug, info};
use std::collections::{HashMap, VecDeque};
use strsim::jaro_winkler;

pub async fn get(api_url: &str) -> HashMap<Level, Vec<MappedEntry>> {
    let mut queue = VecDeque::new();
    let mut mapped_entries: HashMap<Level, Vec<MappedEntry>> = HashMap::new();

    queue.push_back((Level::Provinsi, "0".to_string()));
    while let Some((level, parent)) = queue.pop_front() {
        debug!("Fetching data for level: {:?}, parent: {}", &level, &parent);
        let result = fetch(api_url, &level, &parent).await;
        match result {
            Ok(data) => {
                info!("Fetched data for level: {:?}, parent: {}", &level, parent);
                let entries: Vec<MappedEntry> = data
                    .into_iter()
                    .filter(|entry| !entry.kode_bps.is_empty())
                    .map(|entry| MappedEntry::from_entry(&entry, &parent, &level))
                    .collect();
                if let Some(next_level) = get_next_level(&level) {
                    entries
                        .iter()
                        .map(|entry| entry.code.to_string())
                        .for_each(|parent_code| queue.push_back((next_level.clone(), parent_code)));
                }
                mapped_entries
                    .entry(level.clone())
                    .or_insert_with(Vec::new)
                    .extend(entries);
            }
            Err(e) => handle_error(e, &level, &parent),
        };
    }
    mapped_entries
}

pub async fn get_simple(api_url: &str) -> HashMap<Level, Vec<MappedEntry>> {
    let mut queue = VecDeque::new();
    let mut mapped_entries: HashMap<Level, Vec<MappedEntry>> = HashMap::new();

    queue.push_back((Level::Provinsi, "0".to_string()));
    while let Some((level, parent)) = queue.pop_front() {
        debug!("Fetching data for level: {:?}, parent: {}", &level, &parent);
        let result = fetch_simple(api_url, &level, &parent).await;
        match result {
            Ok(data) => {
                info!("Fetched data for level: {:?}, parent: {}", &level, parent);
                let entries: Vec<MappedEntry> = data
                    .into_iter()
                    .filter(|entry| !entry.kode.is_empty())
                    .map(|entry| MappedEntry::from_simple_entry(&entry, &parent, &level))
                    .collect();
                if let Some(next_level) = get_next_level(&level) {
                    entries
                        .iter()
                        .map(|entry| entry.code.to_string())
                        .for_each(|parent_code| queue.push_back((next_level.clone(), parent_code)));
                }
                mapped_entries
                    .entry(level.clone())
                    .or_insert_with(Vec::new)
                    .extend(entries);
            }
            Err(e) => handle_error(e, &level, &parent),
        };
    }
    mapped_entries
}

fn retain_unique_entries(entries: Vec<Entry>) -> Vec<Entry> {
    entries
        .into_iter()
        .filter(|entry| !entry.kode_bps.is_empty())
        .chunk_by(|entry| entry.kode_bps.clone())
        .into_iter()
        .map(|(_, group)| {
            let mut group: Vec<_> = group.collect();
            group.sort_by(|a, b| {
                let similarity_a =
                    jaro_winkler(&a.nama_bps.to_lowercase(), &a.nama_pos.to_lowercase());
                let similarity_b =
                    jaro_winkler(&b.nama_bps.to_lowercase(), &b.nama_pos.to_lowercase());
                similarity_b
                    .partial_cmp(&similarity_a)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            group.into_iter().next().unwrap()
        })
        // .unique_by(|entry| entry.nama_bps.to_string())
        .collect()
}

async fn fetch(api_url: &str, level: &Level, parent: &str) -> reqwest::Result<Vec<Entry>> {
    let url = format!("{}/?level={}&parent={}", api_url, level.as_str(), parent);
    let response = reqwest::get(&url).await?;
    debug!("Response: {:?}", &response);
    response.json::<Vec<Entry>>().await
}

async fn fetch_simple(
    api_url: &str,
    level: &Level,
    parent: &str,
) -> reqwest::Result<Vec<SimpleEntry>> {
    let url = format!("{}/?level={}&parent={}", api_url, level.as_str(), parent);
    let response = reqwest::get(&url).await?;
    debug!("Response: {:?}", &response);
    response.json::<Vec<SimpleEntry>>().await
}

fn get_next_level(level: &Level) -> Option<Level> {
    match level {
        Level::Provinsi => Some(Level::Kabupaten),
        Level::Kabupaten => Some(Level::Kecamatan),
        Level::Kecamatan => Some(Level::Desa),
        Level::Desa => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::level::Level;

    #[test]
    fn test_get_next_level() {
        assert_eq!(get_next_level(&Level::Provinsi), Some(Level::Kabupaten));
        assert_eq!(get_next_level(&Level::Kabupaten), Some(Level::Kecamatan));
        assert_eq!(get_next_level(&Level::Kecamatan), Some(Level::Desa));
        assert_eq!(get_next_level(&Level::Desa), None);
    }

    #[test]
    fn test_retain_unique_entries() {
        let entries = vec![
            Entry {
                kode_bps: "1".to_string(),
                nama_bps: "Biru-biru".to_string(),
                nama_pos: "Sibiru-biru".to_string(),
                kode_pos: "1111".to_string(),
            },
            Entry {
                kode_bps: "2".to_string(),
                nama_bps: "Namo Rambe".to_string(),
                nama_pos: "Deli Tua".to_string(),
                kode_pos: "1222".to_string(),
            },
            Entry {
                kode_bps: "2".to_string(),
                nama_bps: "Namo Rambe".to_string(),
                nama_pos: "Namorambe".to_string(),
                kode_pos: "1121".to_string(),
            },
        ];

        let result = retain_unique_entries(entries);

        // Check that only two entries remain after deduplication
        assert_eq!(result.len(), 2);

        // Check the retained entries are the best matches
        assert!(result.iter().any(|entry| {
            entry.kode_bps == "1"
                && entry.nama_bps == "Biru-biru"
                && entry.nama_pos == "Sibiru-biru"
                && entry.kode_pos == "1111"
        }));
        assert!(result.iter().any(|entry| {
            entry.kode_bps == "2"
                && entry.nama_bps == "Namo Rambe"
                && entry.nama_pos == "Namorambe"
                && entry.kode_pos == "1121"
        }));
    }
}
