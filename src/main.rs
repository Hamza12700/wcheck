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
  let mut args = Cli::parse();
  let remove_chars = [" ", "'"];
  for remove_char in remove_chars {
    args.word = args.word.to_lowercase().replace(remove_char, "-");
  }

  if args.search {
    cli::find_word(args.word);
    return
  }
  cli::search_word(args.word);
}
