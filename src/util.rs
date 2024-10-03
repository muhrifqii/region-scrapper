pub mod client {
    use crate::model::level::Level;
    use log::error;

    pub fn handle_error(e: reqwest::Error, level: &Level, parent: &str) {
        error!(
            "Error fetching data for level: {:?}, parent: {}",
            level, parent
        );
        error!("{:?}", e);
    }
}
