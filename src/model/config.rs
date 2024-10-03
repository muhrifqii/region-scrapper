use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Config {
    pub region_api: RegionApi,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct RegionApi {
    pub url: String,
}

impl Config {
    pub fn new() -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name("config"))
            .build()?;
        settings.try_deserialize()
    }
}
