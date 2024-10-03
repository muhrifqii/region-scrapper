use log::{debug, error, info};
use serde::de::DeserializeOwned;
use std::borrow::Borrow;
use tokio::sync::watch::error;

use crate::{
    model::{
        entry::{Entry, MappedEntry},
        level::Level,
    },
    util::client::handle_error,
};

pub async fn get(api_url: &str) {
    let mut level = Level::Provinsi;
    let mut parent = 0;
}

async fn loop_get(api_url: &str, level: &Level, parent: &str) -> Option<MappedEntry> {
    debug!("Fetching data for level: {:?}, parent: {}", level, parent);
    let result = fetch(api_url, level, parent).await;
    match result {
        Ok(data) => {
            debug!("Fetched data for level: {:?}, parent: {}", level, parent);

            let (entries, fetches): (Vec<MappedEntry>, Vec<Option<FetchRegion>>) = data
                .iter()
                .filter(|entry| !entry.kode_bps.is_empty())
                .map(|entry| MappedEntry::from_entry(entry, parent, level))
                .map(|entry| {
                    let fetch_param = match level {
                        Level::Provinsi => Some(Level::Kabupaten),
                        Level::Kabupaten => Some(Level::Kecamatan),
                        Level::Kecamatan => Some(Level::Desa),
                        Level::Desa => None,
                    }
                    .map(|next_level| FetchRegion::new(api_url, &next_level, &entry.code.as_str()));
                    (entry, fetch_param)
                })
                .unzip();
            let fetches: Vec<FetchRegion> = fetches
                .into_iter()
                .filter(|f| f.is_some())
                .map(|f| f.unwrap())
                .collect();
            let fetches = tokio::spawn(async move {
                for fetch in fetches {
                    let result = fetch.fetch().await;
                }
            });
        }
        Err(e) => handle_error(e, &level, parent),
    }
    None
}

pub struct FetchRegion {
    api_url: String,
    level: Level,
    parent: String,
}

impl FetchRegion {
    pub fn new(api_url: &str, level: &Level, parent: &str) -> Self {
        Self {
            api_url: api_url.to_string(),
            level: level.clone(),
            parent: parent.to_string(),
        }
    }

    pub async fn fetch(&self) -> reqwest::Result<Vec<Entry>> {
        let url = format!(
            "{}/?level={}&parent={}",
            self.api_url,
            self.level.as_str(),
            self.parent
        );
        reqwest::get(&url).await?.json::<Vec<Entry>>().await
    }
}

async fn fetch(api_url: &str, level: &Level, parent: &str) -> reqwest::Result<Vec<Entry>> {
    FetchRegion::new(api_url, level, parent).fetch().await
}
