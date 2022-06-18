use clap::Parser;
use find::Finder;
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

mod find;

/// Find dbg! macros in a rust project
#[derive(Parser)]
pub struct Cli {
    /// Remove dbg! macros
    #[clap(short, long)]
    remove: bool,

    /// Files to operate on
    files: Vec<PathBuf>,
}

fn main() -> eyre::Result<()> {
    let cli = Cli::parse();

    let mut finder = find::Finder::new().unwrap();
    let action = if cli.remove { remove } else { report };

    if cli.files.is_empty() {
        for entry in walkdir::WalkDir::new(current_dir().unwrap())
            .into_iter()
            .flat_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("rs"))
        {
            action(&mut finder, entry.path())?;
        }
    } else {
        for file in &cli.files {
            action(&mut finder, file)?;
        }
    }

    Ok(())
}

fn report(finder: &mut Finder, path: &Path) -> eyre::Result<()> {
    let buf = std::fs::read(path)?;
    for finding in finder.find(&buf) {
        println!(
            "[{}:{}] {}",
            path.display(),
            finding.line(&buf),
            finding.lookup(&buf)
        );
    }

    Ok(())
}

fn remove(finder: &mut Finder, path: &Path) -> eyre::Result<()> {
    let mut buf = std::fs::read(path)?;

    let mut findings = finder.find(&buf);
    findings.reverse();

    for finding in findings {
        let cut = finding.cut_range(&buf);

        if let Some(preserve) = finding.preserve_range(&buf) {
            let pre = cut.start..preserve.start;
            let post = preserve.end..cut.end;

            drop(buf.drain(post));
            drop(buf.drain(pre));
        } else {
            drop(buf.drain(cut));
        }
    }

    std::fs::write(path, buf)?;

    Ok(())
}
