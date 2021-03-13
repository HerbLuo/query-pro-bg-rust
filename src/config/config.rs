use crate::model::permissions::Permissions;

pub struct Config {
    pub with_logger: bool,
    pub port: u16,
    pub permissions: Vec<Permissions>,
}

impl  Config {
    pub fn build() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    pub fn clone_permissions(&self) -> Vec<Permissions> {
        self.permissions.to_vec()
    }
}

pub struct ConfigBuilder {
    config: Config,
}

impl  ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        let config = Config {
            with_logger: false,
            port: 28686,
            permissions: vec![],
        };
        ConfigBuilder {
            config
        }
    }

    pub fn permissions(mut self, permissions: Vec<Permissions>) -> Self {
        self.config.permissions = permissions;
        self
    }

    #[inline]
    pub fn with_logger(mut self, with_logger: bool) -> Self {
        self.config.with_logger = with_logger;
        self
    }

    #[inline]
    pub fn port(mut self, port: u16) -> Self {
        self.config.port = port;
        self
    }

    pub fn finalize(self) -> Config {
        self.config
    }
}
