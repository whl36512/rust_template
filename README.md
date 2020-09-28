# rust_template

Usage:
- git clone https://github.com/whl36512/rust_template
- rm -rf rust_template/.git
- cd rust_template
- git init
- mv rust_template to any name you want
- Edit Cargo.toml to suit your needs

Install Rust:
- curl https://sh.rustup.rs -sSf | sh
- make sure PATH=$PATH:$HOME/.cargo/bin is in ~/.profile ( or eqivalent profile)
- install linker. On CentOs: sudo yum install cmake gcc

Update Rust:
- rustup update

The Rust module rules are:
- source file is just its own module (except the special files main.rs, lib.rs and mod.rs).
- directory is just a module path component.
- The file mod.rs is just the directory's module.  E.g., struct A in utils/mod.rs is referenced as utils::A


Testing: 
- Unit test: cargo test --lib
- Unit test and test everything in  tests/ , bin/ and ./main.rs with #[test] tag: cargo test
- Run a particular test in bin/  with #[test] tag : cargo test --bin abc
- Run a particular test in tests/  with #[test] tag : cargo test --test abc
- Run a particular test in examples/  with #[test] tag : cargo test --example abc

Formatting
- cargo install cargo-watch
- cargo watch -x fmt &
- For integation with an editor, see rust-fmt documentaion

Print stdout when running test
- cargo test -- --nocapture
