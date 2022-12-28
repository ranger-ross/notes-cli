use std::fs::OpenOptions;
use std::io::Write;

use crate::command::NoteAction;
use crate::command::NoteArgs;
use crate::database;
use crate::note::Note;
use crate::repository;
use chrono::Utc;
use rand::distributions::{Alphanumeric, DistString};

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
        NoteAction::Edit { id } => edit_note(id),
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
        termimad::print_text(&note.body);
    }
}

fn edit_note(id: usize) {
    let mut notes = repository::get_all_notes();
    let note = &notes[id];
    let text = &note.body;

    let data = open_editor(text.to_string());
    notes[id].body = data;

    repository::save_all_notes(notes);
}

/// Creates a temp file and and opens that file with the default system editor.
fn open_editor(text: String) -> String {
    let temp_dir = "/tmp/notes-cli";
    std::fs::create_dir_all(temp_dir).expect("Failed to create temp direcory");

    let file_name = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    let temp_file = String::from(temp_dir) + "/" + &file_name + ".md";

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&temp_file)
        .expect("Failed to create temp file");

    file.write_all(text.as_bytes()).unwrap();

    let editor = std::env::var("EDITOR").unwrap_or("/bin/vi".to_string());
    std::process::Command::new(editor)
        .arg(&temp_file)
        .spawn()
        .expect("Error: Failed to run editor")
        .wait()
        .expect("Error: Editor returned a non-zero status");

    let data = std::fs::read_to_string(&temp_file).expect("Failed to read from temp file");

    std::fs::remove_file(&temp_file).expect("Failed to remove temp file");

    data
}
