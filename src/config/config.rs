#[derive(Deserialize)]
pub struct Db {
    pub uri: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub db: Db,
}

lazy_static! {
    pub static ref CONFIG: Config = init();
}

pub fn init() -> Config {
    let config_str = include_bytes!("config.toml");
    return toml::from_slice(config_str).unwrap();
}
