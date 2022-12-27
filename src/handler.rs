use crate::command::NoteArgs;
use crate::command::NoteAction;


pub fn handle_command(args: NoteArgs) {
    
    match args.action {
        NoteAction::List => println!("List"),
        NoteAction::Show => println!("Show"),
        NoteAction::Create => println!("Create"),
        NoteAction::Delete => println!("Delete"),
        NoteAction::Edit => println!("Edit"),
    }

}