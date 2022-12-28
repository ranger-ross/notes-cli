#[derive(clap::Parser)]
pub struct NoteArgs {
    #[command(subcommand)]
    pub action: NoteAction,
}

#[derive(clap::Subcommand)]
pub enum NoteAction {
    /// Show a list of avaiable notes
    List,
    /// Create a new note
    Create { name: Option<String> },
    /// Edit a note by note id
    Edit,
    /// Delete a note by note id
    Delete { id: usize },
    /// Display a note by note id
    Show { id: usize },
}
