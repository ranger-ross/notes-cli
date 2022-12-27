use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;
use std::path::Path;

use crate::note::DatabaseSchema;

const DEFAULT_DATABASE_STATE: &str = r#"{
    "config": {},
    "notes": []
}"#;

fn get_database_file() -> String {
    String::from("./note-cli-database.json")
}

/**
 * Checks if the database json file exist and creates it if it does not.
 */
pub fn init_database() -> Result<()> {
    let is_new = !Path::new(&get_database_file()).exists();

    let mut database = open_database_file(false)?;

    if is_new {
        database.write_all(DEFAULT_DATABASE_STATE.as_bytes())?
    }

    Ok(())
}

pub fn read_database() -> Result<DatabaseSchema> {
    let database = open_database_file(false).unwrap();
    let reader = BufReader::new(database);

    let mut data: String = "".to_owned();

    for line in reader.lines() {
        data.push_str(&line.unwrap())
    }

    let database_data: DatabaseSchema = serde_json::from_str(&data)?;

    Ok(database_data)
}

pub fn save_database(data: DatabaseSchema) -> Result<()> {
    let json_data_as_string: String = serde_json::to_string_pretty(&data).unwrap();
    let mut database = open_database_file(true).unwrap();
    database.write_all(json_data_as_string.as_bytes())?;
    Ok(())
}

fn open_database_file(is_trunc: bool) -> Result<File> {
    let database_file = get_database_file();

    let database = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(is_trunc)
        .open(database_file)
        .unwrap();

    Ok(database)
}
