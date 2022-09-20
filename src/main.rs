use clap::Parser;
use find::Finder;
use std::path::Path;

mod cli;
mod find;

#[cfg(test)]
mod test;

fn main() -> eyre::Result<()> {
    let cli = cli::Cli::parse();

    let mut finder = find::Finder::new()?;
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
    let buf = std::fs::read(path)?;
    let buf = remove_slice(finder, buf)?;

    std::fs::write(path, buf)?;

    Ok(())
}

fn remove_slice<'a>(finder: &mut Finder, mut buf: Vec<u8>) -> eyre::Result<Vec<u8>> {
    while remove_slice_step(finder, &mut buf)? {}

    Ok(buf)
}

fn remove_slice_step<'a>(finder: &mut Finder, buf: &mut Vec<u8>) -> eyre::Result<bool> {
    let mut findings = finder.find(&buf);
    findings.reverse();

    let mut modified = false;
    for finding in findings {
        let cut = finding.cut_range(&buf);

        modified = true;
        if let Some(preserve) = finding.preserve_range(&buf) {
            let pre = cut.start..preserve.start;
            let post = preserve.end..cut.end;

            drop(buf.drain(post));
            drop(buf.drain(pre));
        } else {
            drop(buf.drain(cut));
        }
    }

    Ok(modified)
}
