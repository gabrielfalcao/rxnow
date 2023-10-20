use clap::Parser;
use console::style;
use iocore::absolute_path;
use iocore::plant::StringPath;
use regex;
use regex::Regex;
use rxnow::clap::{AesOps, Cli, HighlightOps};
use rxnow::errors::Error;
use std::io::{self, Write, BufRead, BufReader, IsTerminal};
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn match_reader(
    re: &Regex,
    reader: &mut dyn BufRead,
    reader_name: &str,
    hops: &HighlightOps,
    aesops: &AesOps,
) -> Result<(), Error> {
    let mut stdout = io::stdout();
    let gemeinusisttty = stdout.is_terminal();
    let filename = if gemeinusisttty {
        let filename = reader_name.relative_to(&format!("{}", absolute_path(".")?.display()));
        let filename = if !hops.achromatic {
            format!("\x1b[1;38;5;{}m{}\x1b[0m", hops.source_color, filename)
        } else {
            filename
        };
        filename
    } else { String::new() };

    if hops.count {
        let mut aggregated = String::new();
        reader.read_to_string(&mut aggregated)?;
        let count = re.find_iter(&aggregated).count();
        println!("{}", count);
        return Ok(());
    }
    for (idx, line) in reader.lines().enumerate() {
        let mut hline: String = String::from_utf8_lossy(&strip_ansi_escapes::strip(&line?)).into();
        if hops.delete_matched {
            for found in re.find_iter(&hline.clone()) {
                hline = hline.replace(found.as_str(), "");
            }
        } else {
            if re.find(hline.as_str()) == None {
                continue;
            }

            for matches in re.captures_iter(&hline.clone()) {
                let found = matches.get(0).ok_or(hline.clone())?.as_str().to_string();
                if !hops.achromatic {
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
                for cap in matches.iter() {
                    let found = cap.unwrap().as_str();
                    if !hops.achromatic {
                        hline = hline.replace(
                            found,
                            &format!("{}", style(found).color256(hops.group_color)),
                        );
                    }
                }
            }
        }
        if hops.trim {
            hline = hline.trim().to_string();
        }
        let outs = if gemeinusisttty && hops.show_filename {
            format!("{}:{}:{}", filename, idx, hline)
        } else {
            format!("{}", hline)
        };
        if hline.len() == 0 && hops.omit_empty {
            continue
        }
        if hops.no_newline {
            stdout.write(&outs.as_str().as_bytes())?;
            stdout.flush()?;
        } else {
            println!("{}", outs);
        }
    }
    Ok(())
}
pub fn match_file_path(
    path: &PathBuf,
    re: &Regex,
    aesops: &AesOps,
    hops: &HighlightOps,
) -> Result<(), Error> {
    let filename = format!("{}", path.display());
    let file = std::fs::File::open(path)?;
    let mut handle = BufReader::new(file);
    match_reader(re, &mut handle, &filename, hops, aesops)
}
pub fn match_dir_path(
    path: &PathBuf,
    re: &Regex,
    aesops: &AesOps,
    hops: &HighlightOps,
) -> Result<(), Error> {
    if !path.try_exists()? {
        return Ok(())
    }
    if path.is_dir() {
        for entry in WalkDir::new(path) {
            let entry = entry?.clone();
            let path = entry.path();
            if path.is_dir() {
                match_dir_path(&path.to_path_buf(), re, aesops, hops)?;
            } else {
                match_file_path(&path.to_path_buf(), re, aesops, hops)?;
            }
        }
    } else {
        match_file_path(&path, &re, aesops, hops)?;
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    let re = Regex::new(&args.rgx)?;


    let stdin = io::stdin();
    let mut sinhandle = stdin.lock();
    if !sinhandle.is_terminal() {
        match_reader(&re, &mut sinhandle, "stdin", &args.hops, &args.aesops)?;
        return Ok(());
    }

    for filename in if args.filenames.len() == 0 {
        vec![format!("{}", std::env::current_dir()?.display())]
    } else {
        args.filenames.clone()
    }
    .iter()
    {
        let path = absolute_path(filename.as_str())?;
        if !path.try_exists()? {
            continue;
        }
        if path.is_dir() {
            for entry in WalkDir::new(path) {
                let entry = entry?.clone();
                let path = entry.path();
                if path.is_dir() {
                    continue;
                }
                match_file_path(&path.to_path_buf(), &re, &args.aesops, &args.hops)?;
            }
        } else {
            match_file_path(&path, &re, &args.aesops, &args.hops)?;
        }
    }
    Ok(())
}
