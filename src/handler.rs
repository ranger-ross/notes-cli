use crate::command::NoteAction;
use crate::command::NoteArgs;
use crate::database;
use crate::note::Note;
use crate::repository;
use chrono::Utc;

pub fn handle_command(args: NoteArgs) {

    match database::init_database() {
        Ok(()) => (),
        Err(error) => panic!("Problem opening the database file: {:?}", error),
    };

    match args.action {
        NoteAction::List => println!("List"),
        NoteAction::Show => println!("Show"),
        NoteAction::Create { name } => create_note(name),
        NoteAction::Delete => println!("Delete"),
        NoteAction::Edit => println!("Edit"),
    }
}

fn create_note(name: Option<String>) {
    let now = Utc::now().timestamp();

    let title: String;

    match name {
        Some(inner) => title = inner,
        None => title = String::from("New note"),
    }

    let note = Note {
        title: title,
        body: String::from(""),
        create_timestamp: now,
        last_updated_timestamp: now,
    };

    repository::insert_note(note);
}
