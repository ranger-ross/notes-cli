use crate::{database, note::Note};
use std::io::{Error, ErrorKind};

pub fn insert_note(note: Note) {
    let mut data = database::read_database().unwrap();

    data.notes.push(note);

    match database::save_database(data) {
        Ok(fc) => fc,
        Err(e) => panic!("Problem updating the database file: {:?}", e),
    }
}

pub fn get_all_notes() -> Vec<Note> {
    let data = database::read_database().unwrap();
    data.notes
}

pub fn delete_note(id: usize) -> Result<(), Error> {
    let mut data = database::read_database().unwrap();

    if id > data.notes.len() - 1 {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid note index"));
    }

    data.notes.remove(id);

    match database::save_database(data) {
        Ok(fc) => fc,
        Err(e) => panic!("Problem updating the database file: {:?}", e),
    }

    Ok(())
}

pub fn save_all_notes(notes: Vec<Note>) {
    let mut data = database::read_database().unwrap();
    data.notes = notes;

    match database::save_database(data) {
        Ok(fc) => fc,
        Err(e) => panic!("Problem updating the database file: {:?}", e),
    }
}
