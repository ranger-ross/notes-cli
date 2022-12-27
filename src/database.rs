use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

const DEFAULT_DATABASE_STATE: &str = r#"{
    "config": {},
    "notes": []
}"#;


/**
 * Checks if the database json file exist and creates it if it does not.
 */
pub fn init_database() -> Result<File> {
    let database_file = "./note-cli-database.json";

    let is_new = !Path::new(database_file).exists();

    if is_new {
        println!("Initalizaing notes database...");
        let mut file = File::create(database_file)?;
        file.write_all(DEFAULT_DATABASE_STATE.as_bytes())?;
        Ok(file)
    } else {
        let file = File::open(database_file)?;
        Ok(file)
    }
}

