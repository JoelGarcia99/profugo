use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct ProfugoContainersConfig {
    pub container_name: String,
    pub engine: String,
    pub db_name: String,
    pub user: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProfugoConfig {
    pub credentials: Vec<ProfugoContainersConfig>,
    pub output_dir: String,
    pub log_dir: String,
}

pub enum DBEngine {
    Postgres,
    MySQL,
}

impl DBEngine {
    pub fn from_str(engine: &str) -> Option<DBEngine> {
        match engine {
            "postgres" => Some(DBEngine::Postgres),
            "mysql" => Some(DBEngine::MySQL),
            _ => None,
        }
    }
}

