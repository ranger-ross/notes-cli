mod command;
mod handler;
mod note;
mod database;
mod repository;

use crate::command::NoteArgs;
use clap::Parser;

fn main() {
    let args = NoteArgs::parse();

    handler::handle_command(args);
}
