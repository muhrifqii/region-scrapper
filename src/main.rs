use model::config::Config;

mod api;
mod model;
mod util;

fn main() {
    let config = Config::new().unwrap();
    let url = config.region_api.url;
}
