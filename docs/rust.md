# Rust

A language empowering everyone to build reliable and efficient software.

!!! note
    이 문서는 [rust-first-steps](https://docs.microsoft.com/ko-kr/learn/paths/rust-first-steps), [rust-lang.org](https://www.rust-lang.org), [rust-doc](https://doc.rust-lang.org/book/title-page.html) 을 보고 내용을 정리한 문서입니다.

## Why Rust?

- Performance
- Reliability
- Productivity

### Performance

Rust is blazingly fast and memory-efficient: with no runtime or garbage collector, it can power performance-critical services, run on embedded devices, and easily integrate with other languages.

### Reliability

Rust’s rich type system and ownership model guarantee memory-safety and thread-safety — enabling you to eliminate many classes of bugs at compile-time.

### Productivity

Rust has great documentation, a friendly compiler with useful error messages, and top-notch tooling — an integrated package manager and build tool, smart multi-editor support with auto-completion and type inspections, an auto-formatter, and more.

## Roadamp to improve

- Building tools; 직관적이고 간단한 building, packaging tool 지원
- Web­Assembly (Writing Web Apps); Binary instruction set in web browsers
- Working with servers (server-side code) 
- Embedded system에 적합하도록 향상

## Rust 특성

Open-source high-level and low-level system programming language

- `Type safe`: The compiler assures that no operation will be applied to a variable of a wrong type.
- `Memory safe`: Rust pointers (known as references) always refer to valid memory.
- `Data race free`: Rust's borrow checker guarantees thread-safety by ensuring that multiple parts of a program can't mutate the same value at the same time.
- `Zero-cost abstractions`: Rust allows the use of high-level concepts, like iteration, interfaces, and functional programming, with minimal to no performance costs. The abstractions perform as well, as if you wrote the underlying code by hand.
- `Minimal runtime`: Rust has a very minimal and optional runtime. The language also has **no garbage collector** to manage memory efficiently. In this way Rust is most similar to languages like C and C++.
- `Targets bare metal`: Rust can target embedded and "bare metal" programming, making it suitable to write an operating system kernel or device drivers.

## Rust module system

- `Crates`: It's the smallest piece of code the Rust compiler can run.
- `Modules`: Groups of crates; Related code items or items that are used together
- `Path`: Paths to access and use the code or items in Rust
- Third-party crate registry: [crates.io](https://crates.io)

### Rust standard libary and useful crates

- `std` - The Rust standard library. In the Rust exercises, you'll notice the following modules:
  - `std::collections` - Definitions for collection types, such as HashMap.
  - `std::env` - Functions for working with your environment.
  - `std::fmt` - Functionality to control output format.
  - `std::fs` - Functions for working with the file system.
  - `std::io` - Definitions and functionality for working with input/output.
  - `std::path` - Definitions and functions that support working with file system path data.
- `structopt` - A third-party crate for easily parsing command-line arguments.
- `chrono` - A third-party crate to handle date and time data.
- `regex` - A third-party crate to work with regular expressions.
- `serde` - A third-party crate of serialization and deserialization operations for Rust data structures.

### How to use crates?

```rust
use std::fmt
```

## Terms

- Wasm (WebAssembly)

`WebAssembly` (abbreviated `Wasm`) is a binary instruction format for a stack-based virtual machine. `Wasm` is designed as a portable compilation target for programming languages, enabling deployment on the web for client and server applications.

`WebAssembly` is a new type of code that can be run in modern web browsers — it is a low-level assembly-like language with a compact binary format that runs with near-native performance and provides languages such as C/C++, C# and Rust with a compilation target so that they can run on the web. It is also designed to run alongside JavaScript, allowing both to work together.

> - **Emscripten SDK** - compile wasm code and generate javascript stub and wasm binary.  
> - **Doc**: [🔗 WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly)

## Latest version

`Version 1.60.0`

## Rust installation

```bash
# Install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update rust
rustup update

# Uninstall rust
rustup self uninstall
```

- `vscode`: [🔗 ext install matklad.rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
- `vim`: [🔗 rust.vim](https://github.com/rust-lang/rust.vim)

> All tools including `rustc`, `cargo`, and `rustup` are installed to the `~/.cargo/bin` directory.

### Cargo

`cargo` is the rust build tool and package manager.

- `cargo --version`: check the rust version
- `cargo new --vcs=git`: create new project with `git init`
- `cargo new`
- `cargo build`: build your project with
- `cargo run`: run your project with
- `cargo test`: test your project with
- `cargo check`: checks your code to make sure it compiles but doesn’t produce an executable
- `cargo doc`: build documentation for your project with
- `cargo publish`: publish a library to `crates.io` with
- Add dependent crates to a project by adding the crate name to the Cargo.toml file.
- `cargo fmt`: reformats your code according to the community code style.
- `cargo fix`: Automatically fix lint warnings reported by rustc

> **manual**: [🔗 cargo doc](https://doc.rust-lang.org/cargo/index.html)

#### Cargo.toml (TOML file for cargo config)

`Cargo.toml`: TOML로 정의된 cargo configuration file

TOML's syntax primarily consists of key = "value" pairs, [section names], and # comments. TOML's syntax somewhat resembles that of .INI files, but it includes a formal specification, whereas the INI file format suffers from many competing variants.

Its specification includes a list of supported data types: String, Integer, Float, Boolean, Datetime, Array, and Table.

```toml
# This is a TOML document.

title = "TOML Example"

[owner]
name = "Tom Preston-Werner"
dob = 1979-05-27T07:32:00-08:00 # First class dates

[database]
server = "192.168.1.1"
ports = [ 8000, 8001, 8002 ]
connection_max = 5000
enabled = true

[servers]

  # Indentation (tabs and/or spaces) is allowed but not required
  [servers.alpha]
  ip = "10.0.0.1"
  dc = "eqdc10"

  [servers.beta]
  ip = "10.0.0.2"
  dc = "eqdc10"

[clients]
data = [ ["gamma", "delta"], [1, 2] ]

# Line breaks are OK when inside arrays
hosts = [
  "alpha",
  "omega"
]
```

### Useful development tool

```bash
rustup component add rustfmt
rustup component add clippy
```

- `cargo fmt`: `rustfmt`; Automatic Formatting development tool
- `cargo fix`: Automatically fix lint warnings reported by rustc
- `cargo clippy`: `clippy`; A collection of lints to analyze your code to catch common mistakes and improve your code.

### Rust build tools

- `rustc`: Rust compiler
- `rustup`: Rust toolchain installer; Rust tool manager
- `rustc --version`: check rust version
- `rustc RUST_FILE.rs`: build the rust binary

> 다음 tool 확인 필요  
> `rustc`, `rustdoc`, `rustfmt`, `rust-gdb`, `rust-lldb`, `rustup`

### Rustup & cargo command completion

Rust build tool에 대한 tab completion

```bash
mkdir -p ~/.local/share/bash-completion/completions
rustup completions bash rustup >> ~/.local/share/bash-completion/completions/rustup
rustup completions bash cargo >> ~/.local/share/bash-completion/completions/cargo
source ~/.local/share/bash-completion/completions/rustup
source ~/.local/share/bash-completion/completions/cargo
```

### Rust Cookbook

Rust crate의 종류를 분류한 cookbook에서 필요한 library를 가장 먼저 찾자!

> 🔗 [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook)

## Helloworld with cargo

```bash
# create new project
cargo new hello-rust
cd hello-rust
tree
#.
#├── Cargo.toml # for the metadata and dependencies of the project
#└── src # rust code
#    └── main.rs

# fn main() is the starting point of all rust appliciations.
cat src/main.rs 
# fn main() {
#     println!("Hello, world!");
# }

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
```

## How to add external crates

외부 라이브러 사용법

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
..
$ RUST_BACKTRACE=1 ./main 
thread 'main' panicked at 'not yet implemented: To Do!', main.rs:2:5
stack backtrace:
   0: rust_begin_unwind
...
   2: main::main
   3: core::ops::function::FnOnce::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

## vscode with rust

> [🔗 how-to-launch-a-rust-application-from-visual-studio-code](https://stackoverflow.com/questions/46885292/how-to-launch-a-rust-application-from-visual-studio-code)

## Rust macro

Rust macro는 개수가 가변적인 입력 인수를 취하는 함수

- `println!`
- `todo!`
- `panic!`

### `println!`

가변 인수를 받아 `stdout`에 출력

```rust
// - If it called a function instead, it would be entered as println (without the `!`).
// - "Hello, world!": string representation of the string
// - {} 인수의 값 대체
println!("Hello, {}!", "world");
```

### `todo!`

완성되지 않은 함수의 prototype 점검 및 선언, 호출시 exception 발생

```rust
// todo! - Display "Hello, world!" with an exception.
fn main() {
  todo!("Display the message by using the println!() macro");
}
```

### `panic!`

The call to `panic!` causes the error message contained in the last two lines.

```rust
fn main() {
    panic!("crash and burn");
}
```

## Rust Syntax

### Constant and Variable declaration

A value is not assigned, it is binded to a variable. python과 같은 bind 개념을 차용함

```rust
// 함수 선언
fn func()

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
let x = 5;
let x = x + 1;
let x = x * 2;
println!("The value of x is: {}", x); // The value of x is: 12

// shardowing으로 변수의 type이 변경되는 것처럼 동작시킬 수 있다.
let spaces = "   ";
let spaces = spaces.len(); // space는 interger가 됨

// 만약, mut를 사용하게되면, type 변경은 불허
let mut spaces = "   ";
spaces = spaces.len(); // cause an error

// constant (상수)
// - constants use uppercase with underscores.
// - constants must be computed at compile time.
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

### Built-in scalar data types

Rust에서 지원하는 scalar data type은 다음과 같다.

```rust
i8, i16, i32, i64 // signed integer number (default: i32)
u8, u16, u32, u64 // unsigned integer number
isize, usize // architecture-dependent number
f32, f64 // floating point number (default f64)
bool // true, false boolean value
char // A 21-bit integer represent a character 
     // that's padded to be 32 bits wide for unicode
&str // a string slice consists of characters.
     // This is immutable string data! (Read-only)
String // It is mutable string data allocated to the heap area.
```

- Rust is a statically typed language. 따라서 compiler가 모든 변수의 정확한 data type을 알아야 함
- `let VAR: TYPE`으로 variable의 type을 명시
- `println!` 사용시 data type suffix를 추가해 compiler가 type을 인지하도록 입력해야 함.

```rust
// Integer literal
let _i = 1000; // i32 assigned by default
let _i: i32 = 1000i32; // suffix for type direction
let _i: i32 = 98_222; // = 98222 Decimal for visual separation
let _i: i32 = 0xff; // Hex
let _i: i32 = 0o77; // Octal
let _i: i32 = 0b1111_0000; // Binary
let _i: u8 = b'A'; // Byte (u8 only)

// Float literal
let _number_64 = 4.0;      // compiler infers the value to use the default type f64
let _number_32: f32 = 5.0; // type f32 specified via annotation

// All of the primitive number types in Rust support mathematical operations
// like addition, subtraction, multiplication, and division.
// When we call the println macro, we add the data type suffix 
// to each literal number to inform Rust about the data type. 

// Addition, Subtraction, and Multiplication
println!("1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}", 1u32 + 2, 8i32 - 5, 15 * 3);

// Integer and Floating point division
println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);

// integer types
let sum = 5 + 10; // addition
let difference = 95.5 - 4.3; // subtraction
let product = 4 * 30; // multiplication
let quotient = 56.7 / 32.2; // division
let remainder = 43 % 5; // remainder

// floating point types
let x = 2.0; // f64
let y: f32 = 3.0; // f32

// boolean type
let t = true;
let f: bool = false; // with explicit type annotation

// character
let c = 'z';
let z = 'ℤ';
let heart_eyed_cat = '😻';
```

### Char type

- Rust’s `char` type is four bytes in size and represents a Unicode Scalar Value.
- Unicode Scalar Values range from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF` inclusive.
- `char` type is a 21-bit integer represent a character that's padded to be 32 bits wide for unicode.
- Char literals use single quotes.`'C'`
- [Storing UTF-8 Encoded Text with Strings](https://doc.rust-lang.org/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings)

### Compound Types

Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

### Tuple Type

Tuples have a fixed length: once declared, they cannot grow or shrink in size.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

// [Tuple destructuring]
// To get the individual values out of a tuple ...
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
println!("The value of y is: {}", y); // The value of y is: 6.4

// [Access a tuple element directly]
let x: (i32, f64, u8) = (500, 6.4, 1);
let five_hundred = x.0;
let six_point_four = x.1;
let one = x.2;
```

> ❓ The tuple without any values, (), is a special type that has only one value, also written (). The type is called the unit type and the value is called the unit value. Expressions implicitly return the unit value if they don’t return any other value.

### Array Type


### &str과 String Type

- &str as a pointer to immutable string data. String literals are all of type &str.

## To be considered

- Integer Overflow: https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow