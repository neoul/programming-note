# Rust

- installation: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- up to date: `rustup update`

## Cargo 

`cargo` is the rust build tool and package manager.

- `cargo build`: build your project with
- `cargo run`: run your project with
- `cargo test`: test your project with
- `cargo doc`: build documentation for your project with
- `cargo publish`: publish a library to crates.io with
- `cargo --version`: check the rust version

## Helloworld

```bash
# create new project
cargo new hello-rust
cd hello-rust
tree
#.
#├── Cargo.toml # for the metadata and dependencies of the project
#└── src # rust code
#    └── main.rs

1 directory, 2 files

# Building and running
cargo run
#    Compiling hello-rust v0.1.0 (/home/neoul/projects/programming-note/rust/hello-rust)
#     Finished dev [unoptimized + debuginfo] target(s) in 0.54s
#      Running `target/debug/hello-rust`
# Hello, world!

tree
#.
#├── Cargo.lock # [FIXME] what is it?
#├── Cargo.toml
#├── src
#│   └── main.rs
#└── target
#    ├── CACHEDIR.TAG # 
#    └── debug
#        ├── build
#        ├── deps
#        │   ├── hello_rust-542ed0184760343f
#        │   └── hello_rust-542ed0184760343f.d
#        ├── examples
#        ├── hello-rust
#        ├── hello-rust.d
#        └── incremental
#

```

## Add packages = crates (물품 운반용 상자)

외부 라이브러 사용

```bash
# Edit Cargo.toml
[dependencies]
ferris-says = "0.2"
```

```rust
// in rust code
use ferris_says::say;
```

`cargo build`: Cargo will install our dependency for us.
