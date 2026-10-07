use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct NoteArgs {
    #[command(subcommand)]
    pub commands: Commands,
    /// Open in the a text editor.
    /// if not set, it will default to using
    /// notepad on windows, and vim on linux.
    #[arg(short, long)]
    pub edit: bool,
}
#[derive(Debug, Clone, Parser)]
pub enum Commands {
    /// Creates a new note.
    New(NewArgs),
    /// Search for keywords recursively
    Keywords(KeywordArgs),
}
#[derive(Debug, Clone, Parser)]
pub struct NewArgs {
    /// Name of the file.
    pub filename: String,
    #[arg(long, short, value_delimiter = ',')]
    /// Appends the Keywords to the file;
    /// Separated by commas.
    pub keywords: Vec<String>,
    #[arg(long, short)]
    /// When provided this will title the note
    /// the following name. Otherwise, it uses
    /// the file name as the file name.
    pub title: Option<String>,
}
#[derive(Debug, Clone, Parser)]
pub struct KeywordArgs {
    #[arg(long, short)]
    /// Specify a directory to recursively search.
    pub directory: Option<String>,
    /// Specify an output format for the keywords
    /// Can be either "markdown" or "csv"
    #[arg(long, short, default_value = "md")]
    pub output: String,
    #[arg(long, short, value_delimiter = ' ',num_args=1..)]
    /// Provide a list of keywords to ignore.
    pub ignore: Option<Vec<String>>,
    #[arg(long, short = 'e', value_delimiter = ' ', num_args=1..)]
    pub ignore_paths: Option<Vec<String>>,
}
