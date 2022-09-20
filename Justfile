
name := `cargo crate-version`

default:
  @just -l

test:
  cargo test

check:
  @type cargo > /dev/null
  @type curl > /dev/null
  @type tree-sitter > /dev/null
  @type cargo-crate-version > /dev/null
  @echo all good

tree-sitter:
  @echo TODO: create tree-sitter update process

rpm:
  mkdir -p target
  tar -czvf target/src.tgz --transform='s:^:{{name}}/:' Cargo.lock Cargo.toml build.rs LICENSE src
  cp target/src.tgz ~/rpmbuild/SOURCES/{{name}}.tgz
  rpmbuild -bb pkg/dedbg.spec

