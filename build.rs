use std::path::PathBuf;

fn main() {
    let dir: PathBuf = ["src", "tree-sitter-rust"].iter().collect();

    cc::Build::new()
        .include(&dir)
        .warnings(false)
        .file(dir.join("parser.c"))
        .file(dir.join("scanner.c"))
        .compile("tree-sitter-rust");
}
