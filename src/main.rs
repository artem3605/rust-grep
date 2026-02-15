use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::Path;

fn ls(dir: &Path) -> io::Result<Vec<OsString>> {
    let mut names = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?; // Result<DirEntry> -> DirEntry
        names.push(entry.file_name()); //OsString -> String
    }
    Ok(names)
}

fn main() -> io::Result<()> {
    let mut args = env::args();

    let _exe = args.next();

    let path = match args.next() {
        Some(p) => p,
        None => {
            eprintln!("Usage: cargo run -- <path>");
            std::process::exit(1);
        }
    };

    let file_names = ls(Path::new(&path))?;

    for name in file_names {
        println!("{}", name.to_string_lossy());
    }

    Ok(())
}
