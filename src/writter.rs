use crate::model::{entry::MappedEntry, level::Level};
use serde_json::to_string_pretty;
use std::{collections::HashMap, io::Result};
use tokio::{fs::File, io::AsyncWriteExt};

pub async fn write_to_json(
    mapped_entries: &HashMap<Level, Vec<MappedEntry>>,
    file_path: &str,
) -> Result<()> {
    let mut file = File::create(file_path).await?;
    let json_data = to_string_pretty(mapped_entries).expect("Failed to serialize JSON");
    file.write_all(json_data.as_bytes()).await?;
    file.flush().await?;
    Ok(())
}

pub async fn write_to_yaml(
    mapped_entries: &HashMap<Level, Vec<MappedEntry>>,
    file_path: &str,
) -> std::io::Result<()> {
    let yaml_data = serde_yaml::to_string(mapped_entries).expect("Failed to serialize YAML");
    let mut file = File::create(file_path).await?;
    file.write_all(yaml_data.as_bytes()).await?;
    file.flush().await?;
    Ok(())
}
