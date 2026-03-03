mod cli;
mod walk;

use std::{env};

fn main() -> std::io::Result<()> {
    let config = cli::parse_args(env::args_os());

    let walker = walk::Walker::new(config);
    let paths = walker.run()?; // io::Result<Vec<PathBuf>>

    for p in paths {
        println!("{}", p.display());
    }

    Ok(())
}