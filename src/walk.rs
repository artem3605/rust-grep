use std::ffi::{OsStr};
use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Debug, Clone)]
pub(crate) struct WalkConfig {
    pub(crate) root: PathBuf,
    pub(crate) mode: Mode,
    pub(crate) recursive: bool,
}

#[derive(Debug, Clone)]
pub(crate) enum Mode {
    List,
    Find(FindQuery),
}

#[derive(Debug, Clone)]
pub(crate) struct FindQuery {
    pub(crate) name: String,
}

pub(crate) struct Walker {
    config: WalkConfig,
}

impl Walker {
    pub(crate) fn new(config: WalkConfig) -> Self {
        Self { config }
    }
    fn collect_files(path: &Path, recursive: bool) -> io::Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        if path.is_file() {
            files.push(path.to_path_buf());
            return Ok(files);
        }

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let p = entry.path();

            if file_type.is_file() {
                files.push(p);
            } else if file_type.is_dir() && recursive {
                files.extend(Self::collect_files(&p, true)?);
            }
        }

        Ok(files)
    }
    pub(crate) fn run(self) -> io::Result<Vec<PathBuf>> {
        let all_files = Self::collect_files(self.config.root.as_path(), self.config.recursive)?;
        match self.config.mode {
            Mode::List => Ok(all_files),

            Mode::Find(q) => {
                let query = OsStr::new(&q.name);

                let mut found = Vec::new();
                for p in all_files {
                    if p.file_name() == Some(query) {
                        found.push(p);
                    }
                }
                Ok(found)
            }
        }
    }
}
