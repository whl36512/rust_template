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

