use crate::model::permissions::Permissions;

pub struct Config<'p> {
    pub with_logger: bool,
    pub port: u16,
    pub permissions: &'p Vec<Permissions>,
}

impl <'p> Config<'p> {
    pub fn build() -> ConfigBuilder<'p> {
        ConfigBuilder::new()
    }
}

pub struct ConfigBuilder<'p> {
    config: Config<'p>,
}

const DEF_PERMISSIONS: &'static Vec<Permissions> = &vec![];

impl <'p> ConfigBuilder<'p> {
    pub fn new() -> ConfigBuilder<'p> {
        let config = Config {
            with_logger: false,
            port: 28686,
            permissions: DEF_PERMISSIONS,
        };
        ConfigBuilder {
            config
        }
    }

    pub fn permissions(mut self, permissions: &'p Vec<Permissions>) -> Self {
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

    pub fn finalize(self) -> Config<'p> {
        self.config
    }
}
