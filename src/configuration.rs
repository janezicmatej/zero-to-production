use envconfig::Envconfig;

#[derive(Debug, Envconfig)]
pub struct Settings {
    #[envconfig(nested = true)]
    pub database: DatabaseSettings,
    // pub server_port: u16,
}

#[derive(Debug, Envconfig)]
pub struct DatabaseSettings {
    #[envconfig(from = "DB_USER")]
    pub user: String,
    #[envconfig(from = "DB_PASSWORD")]
    pub password: String,
    #[envconfig(from = "DB_HOST")]
    pub host: String,
    #[envconfig(from = "DB_PORT")]
    pub port: u16,
    #[envconfig(from = "DB_NAME")]
    pub name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.name
        )
    }
}

pub fn get_configuration() -> Result<Settings, envconfig::Error> {
    Settings::init_from_env()
}
