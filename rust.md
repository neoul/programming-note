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
- `cargo new --vcs=git`: create new project with `git init`
- `cargo new`
- `cargo check`: checks your code to make sure it compiles but doesn’t produce an executable

## rustc

- `rustc --version`
- `rustc RUST_FILE.rs`: build the rust binary


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

## Enabling rust backtrace

Rust display the backtrace if running with `RUST_BACKTRACE=1`.

```bash
$ ./main 
thread 'main' panicked at 'not yet implemented: To Do!', main.rs:2:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
$ RUST_BACKTRACE=1 ./main 
thread 'main' panicked at 'not yet implemented: To Do!', main.rs:2:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/std/src/panicking.rs:498:5
   1: core::panicking::panic_fmt
             at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:116:14
   2: main::main
   3: core::ops::function::FnOnce::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

## vscode with rust

https://stackoverflow.com/questions/46885292/how-to-launch-a-rust-application-from-visual-studio-code

## Rust Syntax

- `println!("Hello, {}!", "world");`
  - `println!` calls a Rust macro. If it called a function instead, it would be entered as println (without the `!`). 
  - "Hello, world!": string representation of the string
  - we end the line with a semicolon (;)
  - {} 인수의 값 대체


```rust
// 함수 선언
fn FUNCTION_NAME()

// 변수 선언
let a_number;

// 변수에 값 바인딩; variable binding to a value
// variable binding == value assginment (값할당)
let a_number = 10;

// 값이 바인딩된 변수는 값 re-바인딩 불가능!!
a_number = 11;

// mut (mutable, 변할 수 있는)로 선언할 경우
let mut b_number = 10;
// variable binding 변경 가능
b_number = 11;

// variable shadowing (변수 섀도잉)
// let으로 동일명의 변수를 계속 선언할 수 있음
// 이것으로 변수의 값을 변경하는 것과 같은 효과를 가질 수 있음
let c_number = 1;
let c_number = 2;
let c_number = 11;
println!("{} {}!","hello", c_number);


```

### Rust macro

rust macro는 개수가 가변적인 입력 인수를 취하는 함수

- `println!`: 가변 인수를 받아 표준 출력에 출력
- `todo!`: 완성되지 않은 함수의 prototype 점검 및 선언, 호출시 exception 발생




