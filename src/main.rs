use clap::Parser;
use find::Finder;
use std::path::Path;

mod cli;
mod find;

fn main() -> eyre::Result<()> {
    let cli = cli::Cli::parse();

    let mut finder = find::Finder::new().unwrap();
    let action = if cli.remove { remove } else { report };

    for path in cli.files() {
        action(&mut finder, &path)?;
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
