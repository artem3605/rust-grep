use std::{env::ArgsOs, path::PathBuf, process};

use crate::walk::{FindQuery, Mode, WalkConfig};

pub fn parse_args(mut args: ArgsOs) -> WalkConfig {
    args.next(); // exe

    let root = match args.next() {
        Some(p) => PathBuf::from(p),
        None => {
            eprintln!("Usage: cargo run -- <root> [--find <name>]");
            process::exit(1);
        }
    };

    let recursive = true; // for now, it's always true
    let mut mode = Mode::List;

    while let Some(a) = args.next() {
        if a == "--find" {
            let name = args.next().unwrap_or_else(|| {
                eprintln!("Usage: ... --find <name>");
                process::exit(1);
            });
            mode = Mode::Find(FindQuery {
                name: name.to_string_lossy().into_owned(),
            });
        }
    }

    WalkConfig {
        root,
        mode,
        recursive,
    }
}
