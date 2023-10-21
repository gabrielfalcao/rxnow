// use crate::coreio::ensure_dir_exists;
// use crate::errors::Error;
use clap::{Args, Parser};

#[derive(Args, Debug)]
#[group(multiple = true)]
pub struct HighlightOps {
    #[arg(short, long)]
    pub no_newline: bool,

    #[arg(short = 'f', long = "show-filename")]
    pub show_filename: bool,

    #[arg(short = 'a', long = "achromatic", help = "disable colored output")]
    pub achromatic: bool,

    #[arg(short = 'c', long = "count", help = "counts occurrences of regex", conflicts_with = "show_filename", conflicts_with="no_newline", conflicts_with="trim")]
    pub count: bool,

    #[arg(
        short = 'o',
        long = "omit-empty",
        help = "omits empty lines"
    )]
    pub omit_empty: bool,

    #[arg(
        short = 't',
        long = "trim",
        alias = "strip",
        help = "strip space characters at both ends of each line"
    )]
    pub trim: bool,

    #[arg(
        short = 'd',
        long = "delete",
        help = "deletes portions of input-data matching the given expression"
    )]
    pub delete_matched: bool,


    #[arg(short = 'l', long = "files-with-matches")]
    pub only_filename: bool,

    #[arg(short = 'C', long, value_name = "NUM")]
    pub context: Option<usize>,

    #[arg(short, long, default_value_t = 220)]
    pub group_color: u8,

    #[arg(short, long, default_value_t = 154)]
    pub match_color: u8,

    #[arg(short, long, default_value_t = 202)]
    pub source_color: u8,
}

#[derive(Args, Debug)]
#[group(multiple = true)]
pub struct AesOps {
    #[arg(
        short = 'r',
        long = "replace",
        value_name = "REPLACEMENT",
        help = "replace (ft. group matching)"
    )]
    pub fable: Option<String>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(value_name = "EXPRESSION", help = "the regex pattern")]
    pub rgx: String,

    #[arg(short, long, help = "whether to ignore case")]
    pub ignore_case: bool,

    #[arg(short = 'I', long, help = "whether to ignore spaces")]
    pub ignore_spaces: bool,

    #[arg(
        value_name = "FILENAMES",
        help = "list of files wherein search shall happen. Defaults to stdin if none is provided"
    )]
    pub filenames: Vec<String>,

    #[command(flatten)]
    pub aesops: AesOps,

    #[command(flatten)]
    pub hops: HighlightOps,
}
