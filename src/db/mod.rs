use dotenv::{dotenv};
use std::env;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

// static DOTENV: Option<Result<PathBuf>> = None;

pub fn create_db_connection() -> MysqlConnection {
  // unsafe {
  //   DOTENV = Some(dotenv().ok());
  // }

  dotenv().ok();

  let env_db_uri = "DB_URI";

  let db_uri = env::var(env_db_uri).expect(&format!("Error finding env var {}", env_db_uri));

  MysqlConnection::establish(&db_uri).expect(&format!("Error connecting to db {}", db_uri))
}
