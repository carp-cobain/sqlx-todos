use std::env;

// DB related config
mod database;

// TCP related config
mod tcp;

/// Configuration settings
#[derive(Clone, Debug)]
pub struct Config {
    pub listen_addr: String,
    pub db_max_connections: u32,
    pub db_host: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_password: String,
    pub db_database: String,
    pub db_schema: String,
}

/// Default for config just calls basic constructor
impl Default for Config {
    fn default() -> Self {
        Self::load()
    }
}

impl Config {
    /// Load config from env vars.
    pub fn load() -> Self {
        // http server settings
        let port = env::var("HTTP_SERVER_PORT").unwrap_or("8080".into());
        let listen_addr = format!("0.0.0.0:{}", port);

        // database settings
        let mut db_max_connections = num_cpus::get() as u32;
        if let Ok(s) = env::var("DB_MAX_CONNECTIONS") {
            db_max_connections = s.parse().expect("DB_MAX_CONNECTIONS could not be parsed")
        }
        let db_host = env::var("DB_HOST").expect("DB_HOST not set");
        let db_port = env::var("DB_PORT")
            .unwrap_or("5432".to_owned())
            .parse()
            .expect("DB_PORT could not be parsed");
        let db_user = env::var("DB_USER").expect("DB_USER not set");
        let db_password = env::var("DB_PASS").expect("DB_PASS not set");
        let db_database = env::var("DB_NAME").expect("DB_NAME not set");
        let db_schema = env::var("DB_SCHEMA").unwrap_or("public".to_owned());

        // Create config
        Self {
            listen_addr,
            db_max_connections,
            db_host,
            db_port,
            db_user,
            db_password,
            db_database,
            db_schema,
        }
    }
}
