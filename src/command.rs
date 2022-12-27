#[derive(clap::Parser)]
pub struct NoteArgs {
   #[command(subcommand)]
   pub action: NoteAction
}

#[derive(clap::Subcommand)]
pub enum NoteAction {
   List,
   Create,
   Edit,
   Delete,
   Show,
}