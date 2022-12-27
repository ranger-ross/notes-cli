use crate::command::NoteAction;
use crate::command::NoteArgs;
use crate::note::Note;
use chrono::Utc;

pub fn handle_command(args: NoteArgs) {
    match args.action {
        NoteAction::List => println!("List"),
        NoteAction::Show => println!("Show"),
        NoteAction::Create { name } => create_note(name),
        NoteAction::Delete => println!("Delete"),
        NoteAction::Edit => println!("Edit"),
    }
}

fn create_note(name: Option<String>) {
    let now = Utc::now();

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

    println!("Create, {:?}", note);
}
