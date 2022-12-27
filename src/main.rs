mod command;
mod handler;
mod note;
mod database;

use crate::command::NoteArgs;
use clap::Parser;

fn main() {
    let args = NoteArgs::parse();

    let _database_file = match database::init_database()  {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the database file: {:?}", error),
    };

    
    handler::handle_command(args);
}
