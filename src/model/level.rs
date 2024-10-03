#[derive(Debug, Clone)]
pub enum Level {
    Provinsi,
    Kabupaten,
    Kecamatan,
    Desa,
}

impl Level {
    pub fn as_str(&self) -> &'static str {
        match self {
            Level::Provinsi => "provinsi",
            Level::Kabupaten => "kabupaten",
            Level::Kecamatan => "kecamatan",
            Level::Desa => "desa",
        }
    }
}
