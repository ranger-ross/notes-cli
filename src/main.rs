mod command;
mod handler;
mod note;

use crate::command::NoteArgs;
use clap::Parser;

fn main() {
    let args = NoteArgs::parse();

    handler::handle_command(args);
}
