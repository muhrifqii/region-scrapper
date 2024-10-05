use log::debug;
use model::config::Config;
use std::error::Error;

mod api;
mod model;
mod util;
mod writter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    debug!("Starting...");
    let config = Config::new().unwrap();
    debug!("Config: {:?}", config);
    let url = config.region_api.url;
    // let mapped_entries = api::get(&url).await;
    let mapped_entries = api::get_simple(&url).await;

    let write_json = writter::write_to_json(&mapped_entries, "data/json/simple.json");
    let write_yaml = writter::write_to_yaml(&mapped_entries, "data/yaml/simple.yaml");

    let (json_result, yaml_result) = tokio::join!(write_json, write_yaml);

    if let Err(e) = json_result {
        eprintln!("Failed to write JSON: {}", e);
    }
    if let Err(e) = yaml_result {
        eprintln!("Failed to write YAML: {}", e);
    }

    Result::Ok(())
}
