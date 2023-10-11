#![allow(unused)]
use clap::Parser;
use console::style;
use iocore::absolute_path;
use regex;
use regex::Regex;
use rxnow::clap::{AesOps, Cli, HighlightOps};
use rxnow::errors::Error;
use std::io::{self, BufRead, BufReader, IsTerminal};
use walkdir::WalkDir;

pub fn match_reader(
    re: &Regex,
    reader: &mut dyn BufRead,
    reader_name: &str,
    hops: &HighlightOps,
    aesops: &AesOps,
) -> Result<(), Error> {
    for (idx, line) in reader.lines().enumerate() {
        let mut hline = line?.clone();
        if hops.delete_matched {
            match re.find(&hline.clone()) {
                Some(found) => {
                    hline = hline.replace(found.as_str(), "");
                }
                None => continue,
            }
        } else {
            if re.find(hline.as_str()) == None {
                continue;
            }

            for matches in re.captures_iter(&hline.clone()) {
                let mut found = matches.get(0).ok_or(hline.clone())?.as_str().to_string();
                if !hops.colorless {
                    hline = hline.replace(
                        found.clone().as_str(),
                        &if let Some(more) = &aesops.fable {
                            let mut fnd = String::new();
                            matches.expand(
                                &format!("{}", style(more).color256(hops.group_color)),
                                &mut fnd,
                            );
                            fnd
                        } else {
                            format!("{}", style(found).color256(hops.match_color))
                        },
                    );
                } else {
                    hline = hline.replace(
                        found.clone().as_str(),
                        &if let Some(more) = &aesops.fable {
                            let mut fnd = String::new();
                            matches.expand(&more, &mut fnd);
                            fnd
                        } else {
                            found.as_str().to_string()
                        },
                    );
                }
                for (idx, cap) in matches.iter().enumerate() {
                    let found = cap.unwrap().as_str();
                    if !hops.colorless {
                        hline = hline.replace(
                            found,
                            &format!("{}", style(found).color256(hops.group_color)),
                        );
                    }
                }
            }
        }
        if hops.hide_filename {
            println!("{}", hline);
        } else {
            println!("{}:{}:{}", reader_name, idx, hline);
        }
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let re = Regex::new(&args.rgx)?;
    if args.filenames.len() > 0 {
        for filename in args.filenames.iter() {
            let path = absolute_path(filename.as_str())?;
            if !path.try_exists()? {
                continue;
            }
            if path.is_dir() {
                for entry in WalkDir::new(path) {
                    let entry = entry?.clone();
                    let path = entry.path();
                    // let path = absolute_path(&format!("{}", entry.path().display()))?;
                    let filename = format!("{}", path.display());
                    if path.is_dir() {
                        continue;
                    }
                    let file = std::fs::File::open(path)?;
                    let mut handle = BufReader::new(file);
                    match_reader(
                        &re,
                        &mut handle,
                        filename.as_str(),
                        &args.hops,
                        &args.aesops,
                    )?;
                }
            } else {
                let file = std::fs::File::open(path)?;
                let mut handle = BufReader::new(file);
                match_reader(
                    &re,
                    &mut handle,
                    filename.as_str(),
                    &args.hops,
                    &args.aesops,
                )?;
            }
        }
    } else {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        if handle.is_terminal() {
            eprintln!("stdin appears to be a tty");
            std::process::exit(0x54);
        } else {
            match_reader(&re, &mut handle, "stdin", &args.hops, &args.aesops)?;
        }
    }
    Ok(())
}
