use model::config::Config;
use std::error::Error;

mod api;
mod model;
mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new().unwrap();
    let url = config.region_api.url;
    let result = api::loop_get(&url).await;

    Result::Ok(())
}
