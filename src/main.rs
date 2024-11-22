use clap::Parser;
mod cli;

/// Search for the word meaning
#[derive(Parser)]
struct Cli {
  /// Word to search for the meaning
  word: String,

  /// Suggest similar words
  #[arg(short, long)]
  search: bool,
}

fn main() {
  let args = Cli::parse();
  if args.search {
    cli::find_word(args.word.replace(" ", "-").to_lowercase());
    return
  }
  cli::search_word(args.word.replace(" ", "-").to_lowercase());
}
