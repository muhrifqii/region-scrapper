use crate::{
    model::{
        entry::{Entry, MappedEntry},
        level::Level,
    },
    util::client::handle_error,
};
use log::{debug, info};
use std::collections::{HashMap, VecDeque};

fn get_next_level(level: &Level) -> Option<Level> {
    match level {
        Level::Provinsi => Some(Level::Kabupaten),
        Level::Kabupaten => Some(Level::Kecamatan),
        Level::Kecamatan => Some(Level::Desa),
        Level::Desa => None,
    }
}

pub async fn loop_get(api_url: &str) -> HashMap<Level, Vec<MappedEntry>> {
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
                mapped_entries.insert(level.clone(), entries);
            }
            Err(e) => handle_error(e, &level, &parent),
        };
    }
    mapped_entries
}

async fn fetch(api_url: &str, level: &Level, parent: &str) -> reqwest::Result<Vec<Entry>> {
    let url = format!("{}/?level={}&parent={}", api_url, level.as_str(), parent);
    reqwest::get(&url).await?.json::<Vec<Entry>>().await
}
