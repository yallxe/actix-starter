// TODO: Rewrite it config, using macros or smth whatever will make it look better

#[derive(Clone, Debug)]
pub struct Configuration {
    pub bind_host: String,
    pub bind_port: u16,
    pub database_url: String,
}

impl Configuration {
    pub fn from_env() -> Self {
        let bind_host = std::env::var("BIND_HOST").expect("BIND_HOST must be set");
        let bind_port = std::env::var("BIND_PORT").expect("BIND_PORT must be set");
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        
        Configuration {
            bind_host,
            bind_port: bind_port.parse().expect("BIND_PORT must be a number"),
            database_url,
        }
    }
}