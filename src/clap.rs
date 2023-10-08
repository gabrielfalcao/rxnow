// use crate::coreio::ensure_dir_exists;
// use crate::errors::Error;
use clap::{Parser, Args};


#[derive(Args, Debug)]
#[group(multiple = false)]
pub struct HighlightOps {
    #[arg(short, long)]
    pub newline: bool,

    #[arg(short = 'H', long)]
    pub hide_filename: bool,

    #[arg(short = 'c', long)]
    pub colorless: bool,

    #[arg(short = 'l', long = "files-with-matches")]
    pub only_filename: bool,

    #[arg(short = 'C', long, value_name = "NUM")]
    pub context: Option<usize>,

    #[arg(short, long, default_value_t = 220)]
    pub group_color: u8,

    #[arg(short, long, default_value_t = 154)]
    pub match_color: u8,
}

#[derive(Args, Debug)]
#[group(multiple = false)]
pub struct AesOps {
    #[arg(short = 'r', long = "replace", value_name = "REPLACEMENT", help = "replace (ft. group matching)")]
    pub fable: Option<String>,
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(value_name = "EXPRESSION", help = "the regex pattern")]
    pub rgx: String,

    #[arg(value_name = "FILENAMES", help = "list of files wherein search shall happen. Defaults to stdin if none is provided")]
    pub filenames: Vec<String>,

    #[command(flatten)]
    pub aesops: AesOps,

    #[command(flatten)]
    pub hops: HighlightOps,
}
