use crate::{database, note::Note};

pub fn insert_note(note: Note) {
    let mut data = database::read_database().unwrap();

    data.notes.push(note);

    match database::save_database(data) {
        Ok(fc) => fc,
        Err(e) => panic!("Problem updating the database file: {:?}", e),
    }

    println!("Note added!");
}

pub fn get_all_notes() -> Vec<Note> {
    let data = database::read_database().unwrap();
    data.notes
}