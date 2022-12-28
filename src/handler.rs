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
        NoteAction::List => list_notes(),
        NoteAction::Show { id } => show_note(id),
        NoteAction::Create { name } => create_note(name),
        NoteAction::Delete { id } => delete_note(id),
        NoteAction::Edit { id }  => edit_note(id),
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
    println!("Note added!");
    list_notes();
}

fn list_notes() {
    let notes = repository::get_all_notes();

    if notes.is_empty() {
        println!("No notes");
        return;
    }

    let result = notes
        .into_iter()
        .enumerate()
        .map(|(i, note)| {
            let mut line = String::from("[");
            line.push_str(i.to_string().as_str());
            line.push_str("] - ");
            line.push_str(note.title.as_str());
            line.push_str("\n");
            line
        })
        .collect::<String>();

    println!("{}", result);
}

fn delete_note(id: usize) {
    match repository::delete_note(id) {
        Ok(()) => {
            println!("Removed note");
            list_notes();
        }
        Err(err) => println!("Error deleting note, {:?}", err),
    };
}

fn show_note(id: usize) {
    let notes = repository::get_all_notes();

    let note = &notes[id];

    if note.body.is_empty() {
        println!("Note is empty");
    } else {
        println!("{}", note.body);
    }
}

fn edit_note(id: usize) {
    println!("Edit")
}
