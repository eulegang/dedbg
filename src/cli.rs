use clap::Parser;
use std::{env::current_dir, path::PathBuf};

/// Find dbg! macros in a rust project
#[derive(Parser)]
#[clap(author, version)]
pub struct Cli {
    /// Remove dbg! macros
    #[clap(short, long)]
    pub remove: bool,

    /// Files to operate on
    pub files: Vec<PathBuf>,
}

impl Cli {
    pub fn files(&self) -> Box<dyn Iterator<Item = PathBuf> + '_> {
        if self.files.is_empty() {
            Box::new(
                walkdir::WalkDir::new(current_dir().unwrap())
                    .into_iter()
                    .flat_map(|e| e.ok())
                    .filter(|e| e.file_type().is_file())
                    .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("rs"))
                    .map(|e| e.into_path()),
            )
        } else {
            Box::new(self.files.clone().into_iter().map(|e| e))
        }
    }
}
