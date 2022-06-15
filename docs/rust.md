# Rust

A language empowering everyone to build reliable and efficient software.

!!! note
    이 문서는 [rust-first-steps](https://docs.microsoft.com/ko-kr/learn/paths/rust-first-steps), [rust-lang.org](https://www.rust-lang.org), [rust-doc](https://doc.rust-lang.org/book/title-page.html) 을 보고 내용을 정리한 문서입니다.


- [Rust](#rust)
  - [Why Rust?](#why-rust)
    - [Performance](#performance)
    - [Reliability](#reliability)
    - [Productivity](#productivity)
  - [Roadamp to improve](#roadamp-to-improve)
  - [Rust 특성](#rust-특성)
  - [내가 생각하는 rust 특성](#내가-생각하는-rust-특성)
  - [Rust module system](#rust-module-system)
    - [Rust standard libary and useful crates](#rust-standard-libary-and-useful-crates)
    - [How to use crates?](#how-to-use-crates)
  - [Terms](#terms)
  - [Latest version](#latest-version)
  - [Rust installation](#rust-installation)
    - [Cargo](#cargo)
      - [Cargo versioning](#cargo-versioning)
      - [Cargo.toml (TOML file for cargo config)](#cargotoml-toml-file-for-cargo-config)
      - [Cargo.lock](#cargolock)
    - [Useful development tool](#useful-development-tool)
    - [Rust build tools](#rust-build-tools)
    - [Rustup & cargo command completion](#rustup--cargo-command-completion)
    - [Rust Cookbook](#rust-cookbook)
  - [Helloworld with cargo](#helloworld-with-cargo)
  - [How to add external crates](#how-to-add-external-crates)
  - [Enabling rust backtrace](#enabling-rust-backtrace)
  - [Rust macro](#rust-macro)
    - [`println!`, `print!`](#println-print)
    - [`todo!`](#todo)
    - [`panic!`](#panic)
    - [`dbg!`](#dbg)
  - [Prelude (imported std libraries)](#prelude-imported-std-libraries)
  - [Rust ownership (값에 대한 소유권)](#rust-ownership-값에-대한-소유권)
  - [References and Borrowing](#references-and-borrowing)
  - [Comments](#comments)
    - [Document comments](#document-comments)
  - [Constants and Variables](#constants-and-variables)
  - [Built-in scalar data types](#built-in-scalar-data-types)
  - [Char type](#char-type)
  - [Compound Types](#compound-types)
    - [Tuple Type](#tuple-type)
    - [Array Type](#array-type)
    - [&str과 String Type](#str과-string-type)
  - [Functions](#functions)
    - [Diverging functions](#diverging-functions)
    - [Associated function indication `::`](#associated-function-indication-)
  - [closure](#closure)
    - [Call chaining using closure](#call-chaining-using-closure)
  - [Statements and expressions](#statements-and-expressions)
  - [Control flow](#control-flow)
    - [`if..else`:](#ifelse)
    - [`loop`, `while` and `for`](#loop-while-and-for)
    - [`match`](#match)
  - [Result type](#result-type)
  - [Reference](#reference)
  - [methods](#methods)
    - [Field Init Shorthand](#field-init-shorthand)
    - [Struct Update Syntax](#struct-update-syntax)
  - [Tuple Structs](#tuple-structs)
  - [Generics](#generics)
    - [Generic Type](#generic-type)
    - [Generic functions](#generic-functions)
    - [Generic Implementations (Generic methods)](#generic-implementations-generic-methods)
    - [Bounds](#bounds)
    - [Where clause](#where-clause)
    - [Associated types](#associated-types)
    - [Phantom type parameters](#phantom-type-parameters)
  - [Traits](#traits)
  - [To be considered](#to-be-considered)
  - [collections](#collections)
    - [Box, stack and heap](#box-stack-and-heap)
    - [String](#string)
    - [hash map](#hash-map)
  - [Useful code or crates](#useful-code-or-crates)
    - [Read stdin](#read-stdin)
  - [Modules](#modules)
  - [Macro](#macro)
    - [Declarative Macros](#declarative-macros)
    - [Procedural Macros](#procedural-macros)
    - [Links for macro](#links-for-macro)
  - [Testing](#testing)
  - [Rust Attributes](#rust-attributes)
    - [Scope](#scope)
    - [Attribute arguments](#attribute-arguments)
    - [Frequently Used Attributes](#frequently-used-attributes)
    - [Custom cfg](#custom-cfg)
    - [`derive` attribute](#derive-attribute)
  - [Keywords](#keywords)
    - [`crate`](#crate)
  - [Good answer to understand](#good-answer-to-understand)
  - [Associated items](#associated-items)
  - [Rust RFC](#rust-rfc)
  - [FFI (Foreign Function Interface)](#ffi-foreign-function-interface)
    - [Calling foreign functions](#calling-foreign-functions)
    - [Calling Rust code from C](#calling-rust-code-from-c)
    - [가변 인자 함수 (variadic functions)](#가변-인자-함수-variadic-functions)
  - [Logging](#logging)
  - [lib.rs and main.rs](#librs-and-mainrs)

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

## 내가 생각하는 rust 특성

- Variable과 function type을 극도로 제어하여 gabage collection이 필요없도록 만듦

## Rust module system

- `Crates`: It's the smallest piece of code the Rust compiler can run.
- `Modules`: A number of modules become a crate; Related code items or items that are used together
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
  - `cargo doc --open`: 현재 crate의 html 문서 생성
- `cargo publish`: publish a library to `crates.io` with
- Add dependent crates to a project by adding the crate name to the Cargo.toml file.
- `cargo fmt`: reformats your code according to the community code style.
- `cargo fix`: Automatically fix lint warnings reported by rustc
- `cargo install`: `$HOME/.cargo/bin`에 crate binary를 설치
  - `cargo install cargo-generate`: make a new Rust project by leveraging a pre-existing git repository as a template. e.g. 

> **manual**: [🔗 cargo doc](https://doc.rust-lang.org/cargo/index.html)

#### Cargo versioning

Cargo는 버전을 명시하는 표준에 해당하는 [Semantic Versioning(semver)](https://semver.org/)을 이용합니다.

```toml
# Example
rand = "0.8.3"
```

The number 0.8.3 is actually shorthand for ^0.8.3, which means any version that is at least 0.8.3 but below 0.9.0. 

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

#### Cargo.lock

- `go.sum`과 같이 다운로드한 crate (package)에 대한 version과 정보를 명세하여, 이후 동일한 crate version으로 build의 일관성을 유지함.
- `cargo update`: crate의 minor version만 업데이트함; major version을 변경하려면, Cargo.toml의 major version을 업데이트해야 한다.

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

## Rust macro

Rust macro는 개수가 가변적인 입력 인수를 취하는 함수

- `println!`, `print!`
- `todo!`
- `panic!`
- `dbg!`

### `println!`, `print!`

가변 인수를 받아 `stdout`에 출력

```rust
// - If it called a function instead, it would be entered as println (without the `!`).
// - "Hello, world!": string representation of the string
// - {}에 인수의 값 대체 삽입; placeholder라 부름
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

### `dbg!`

Another way to print out a value using the Debug format is to use the `dbg!` macro, which takes ownership of an expression, prints the file and line number of where that dbg! macro call occurs in your code along with the resulting value of that expression, and returns ownership of the value.

> Note: print message to `stderr`.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    let rect = dbg!(rect);
    println!(
        "The area of the rectangle {:?} is {} square pixels.",
        rect,
        area(&rect)
    );
}
fn area(rect :&Rectangle) -> u32 {
    return rect.width * rect.height;
}
// [src/main.rs:16] 30 * scale = 60
// [src/main.rs:20] rect = Rectangle {
//     width: 60,
//     height: 50,
// }
// The area of the rectangle Rectangle { width: 60, height: 50 } is 3000 square pixels.
```

## Prelude (imported std libraries)

The prelude is the list of things that Rust automatically imports into every Rust program. 

[🔗 Module std::prelude](https://doc.rust-lang.org/std/prelude/index.html)

For example ... 

- `std::result::Result::{self, Ok, Err}`, a type for functions that may succeed or fail. Like Option, its variants are exported as well.
- `std::string::{String, ToString}`, heap-allocated strings.
- `std::vec::Vec`, a growable, heap-allocated vector.

## Rust ownership (값에 대한 소유권)

- Rust의 모든 값은 owner라는 변수를 가지며, 하나의 값은 하나의 owner 변수에만 종속될 수 있음
- 만약, copy trait이 정의되어 있으면, deep copy를 수행하나 아닌 경우 ownership move가 일어남(?)
- ownership move: 다수의 변수가 하나의 값을 가리킬 수 없으며, ownership move (소유권 이전)가 수행됨
- owner가 종속 범위 (curly bracket)을 벗어나면, drop (free)됨 (C++ RAII 패턴)

```rust
let s1 = String::from("hello");
let s2 = s1;

// String은 copy trait이 정의되지 않아 ownership move가 발생하며, 이후 s1을 사용할 경우 오류 발생
// error[E0382]: use of moved value: `rect` 발생
```

- 함수에 값을 인자로 넘길 경우 ownership도 변경됨
- deep copy: copy trait이 구현된 structure나 built-in scalar는 deep copy를 수행함
- shallow copy: rust에서는 일어나지 않음
- heap에 할당되는 가변 (mutable)의 structure는 보통 reference로 ownership 관리를 회피


Here are some of the types that implement Copy:

- All the integer types, such as `u32`.
- The Boolean type, `bool`, with values true and false.
- All the floating point types, such as `f64`.
- The character type, `char`.
- `Tuples`, if they only contain types that also implement Copy. For example, `(i32, i32)` implements Copy, but `(i32, String)` does not.

> - https://velog.io/@timothy160620/Learning-Rust
> - https://showx123.tistory.com/81

## References and Borrowing

![Figure 4-5: A diagram of &String s pointing at String s1](https://doc.rust-lang.org/book/img/trpl04-05.svg)

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

> Note: The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, `*`. We’ll see some uses of the dereference operator in Chapter 8 and discuss details of dereferencing in Chapter 15.


## Comments

In Rust, the idiomatic comment style starts a comment with two slashes, and the comment continues until the end of the line.

```rust
// Hello, world.
```

### Document comments

Rust also has a particular kind of comment for documentation, known conveniently as a documentation comment, that will generate HTML documentation. Documentation comments use three slashes, `///`, instead of two and support Markdown notation for formatting the text.

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

Another style of doc comment, `//!`, is used to describe the crate introduction.

```rust
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.
```

> [[FIXME] document comments 다시 읽기](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html)

## Constants and Variables

- A value is not assigned, it is binded to a variable. python과 같은 bind 개념을 차용함
- Constants must be computed at compile time.
- The equal sign (`=`) tells Rust we want to bind something to the variable now. 

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

## Built-in scalar data types

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
- `String` is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text. This is not a built-in type.

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

## Char type

- Rust’s `char` type is four bytes in size and represents a Unicode Scalar Value.
- Unicode Scalar Values range from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF` inclusive.
- `char` type is a 21-bit integer represent a character that's padded to be 32 bits wide for unicode.
- Char literals use single quotes.`'C'`
- [Storing UTF-8 Encoded Text with Strings](https://doc.rust-lang.org/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings)

## Compound Types

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

- Unlike a tuple, every element of an array must have the same type.
- Unlike arrays in some other languages, arrays in Rust have a fixed length.
- Arrays are useful when you want your data allocated on the stack rather than the heap.
- Flexible size array가 필요하다면 `Vec<T>`를 사용

```rust
let a = [1, 2, 3, 4, 5];
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
let a: [i32; 5] = [1, 2, 3, 4, 5]; // [Type; Length]
let a = [3; 5]; // [3, 3, 3, 3, 3]으로 초기화

// [Access an array element]
let first = a[0];
let second = a[1];

// mutable해야 Arrary element 변경 가능함
let mut a = [1, 2, 3, 4, 5];
a[1] = 100;
```

### &str과 String Type

- &str as a pointer to immutable string data. String literals are all of type &str.

## Functions

- `fn` keyward를 사용
- All letters of function names and variables are lowercase and underscores (`_`) separate words.
- A set of parentheses and curly brackets are followed to the function name.
- In function signatures, you must declare the type of each parameter that you want to input.
- The function returns a concrete type: The Rust compiler needs to know how much space every function's return type requires.

```rust
fn main() {
  another_function(5);
  print_labeled_measurement(5, 'h');
}

fn another_function(x: i32) {
  println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("The measurement is: {}{}", value, unit_label);
}

fn four() -> i32 {
  4; // error because it becomes a statement
}

fn five() -> i32 {
  5 // no semicolon; return the value as an expression
}

fn six() -> i32 { // 명시적 반환
  return 6; // ok
}

fn seven() -> i32 {
  return 7 // ok
}

fn main() {
  let x = five();
  println!("The value of x is: {}", x);
}
```

### Diverging functions

Diverging functions never return. They are marked using `!`, which is an empty type.

> server등의 non-terminated app에서 사용하면 될 듯 ...

```rust
fn foo() -> ! {
  panic!("This call never returns.");
}
```

### Associated function indication `::`

```rust
let mut guess = String::new();
```

The `::` syntax in the `::new` line indicates that new is an associated function of the String type. An associated function is a function that’s implemented on a type, in this case String.


## closure

Closures are functions that can capture the enclosing environment. For example, a closure that captures the x variable:

`|val| val + x`

- **capturing** and **bollowing**: closure 함수가 variable에 할당될 때 동일 namespace에 있는 변수를 capture하여 사용함
  - by reference: `&T`
  - by mutable reference: `&mut T`
  - by value: `T`
- `move` 사용시 variable의 ownership을 가져감 
  - e.g. `let contains = move |needle| haystack.contains(needle);`
- input & output parameter로 사용가능
  - type bound에 아래와 같은 argument function의 trait을 지정해야 함
    - `Fn`: the closure uses the captured value by reference (`&T`)
    - `FnMut`: the closure uses the captured value by mutable reference (`&mut T`)
    - `FnOnce`: the closure uses the captured value by value (`T`)
- iterator 동작을 구현할 때 사용 e.g. `Iterator::any`


```rust
// Increment via closures and functions.
fn function(i: i32) -> i32 {
    i + 1
}

// Closures are anonymous, here we are binding them to references
// Annotation is identical to function annotation but is optional
// as are the `{}` wrapping the body. These nameless functions
// are assigned to appropriately named variables.
let closure_annotated = |i: i32| -> i32 { i + 1 };
let closure_inferred = |i| i + 1;

let mut i = 1;
i = function(i); // 2
i = closure_annotated(i); // 3
i = closure_inferred(i); // 4
println!("closure example i: {}", i);
// let i = 1;
// Call the function and closures.
println!("function: {}", function(i)); // 5
println!("closure_annotated: {}", closure_annotated(i)); // 5
println!("closure_inferred: {}", closure_inferred(i)); // 5

// A closure taking no arguments which returns an `i32`.
// The return type is inferred.
let one = || 1;
println!("closure returning one: {}", one());

use std::mem;
let color = String::from("green");

// bollow immutable reference of the color variable.
let print = || println!("`color`: {}", color);
print();

let _reborrow = &color;
print();

// A move or reborrow is allowed after the final use of `print`
let _color_moved = color;
// print(); // 호출 불가; color already moved to _color_moved


let mut count = 0;
// bollow mutable count reference.
let mut inc = || {
    count += 1;
    println!("`count`: {}", count);
};

// Call the closure using a mutable borrow.
inc();

// The closure no longer needs to borrow `&mut count`. Therefore, it is
// possible to reborrow without an error
let _count_reborrowed = &mut count; 

// `consume` consumes the variable so this can only be called once.
let movable = Box::new(3);
let consume = || {
    println!("`movable`: {:?}", movable);
    mem::drop(movable);
};
consume();
// consume(); // 호출 불가; The bollowed variable is not available.
// ^ TODO: Try uncommenting this line.

// move 사용시 ownership을 가져감
let haystack = vec![1, 2, 3];
let contains = move |needle| haystack.contains(needle);
println!("{}", contains(&1));

// Functional approach
let sum_of_squared_odd_numbers: u32 =
    (0..).map(|n| n * n)                             // All natural numbers squared
          .take_while(|&n_squared| n_squared < upper) // Below upper limit
          .filter(|&n_squared| is_odd(n_squared))     // That are odd
          .fold(0, |acc, n_squared| acc + n_squared); // Sum them
println!("functional Approach: {}", sum_of_squared_odd_numbers);
```

### Call chaining using closure

```rust
// call chaining using closure
fn is_odd(n: u32) -> bool {
    n % 2 == 1
}
let upper = 1000;
let sum_of_squared_odd_numbers: u32 = (0..)
    .map(|n| n * n) // All natural numbers squared
    .take_while(|&n_squared| n_squared < upper) // Below upper limit
    .filter(|&n_squared| is_odd(n_squared)) // That are odd
    .fold(0, |acc, n_squared| acc + n_squared); // Sum them
println!("functional Approach: {}", sum_of_squared_odd_numbers);
```



## Statements and expressions

Rust는 Statement와 expression의 구분이 다음과 같이 명확함.

- Statements are instructions that perform some action and do not return a value.
- Expressions evaluate to a resulting value and return the result value.
- Expressions do not include ending semicolons.
- If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
- javascript와 유사하게 세미콜론(`;`) 없이 function 마지막 문장이 끝나면, 그 마지막 결과를 반환함 (as an expression)

> Expressions in rust: function call, value, {} (블록)

```rust
fn main() {
  // error 발생 let y는 return value 없음
  // expression이 아니므로 x에 binding 불가
  let x = (let y = 6);

  // 4로 판정된 {} 안의 값이 y에 할당
  // x + 1에는 세미콜론(;) 이 없으며 expression으로 판정, 값을 반환
  let y = {
    let x = 3;
    x + 1
  };
  println!("The value of y is: {}", y);
}
```

## Control flow

### `if..else`:

- condition은 반드시 boolean을 반환해야 함
- parenthesis `()` 는 사용안함
- `arms`: 갈래?; condition에 따라 실행되는 코드블록을 말함
- `let` 할당 연산 (`=`)에 `if..else` 사용 가능

```rust
let number = 6;
if number % 4 == 0 {
  println!("number is divisible by 4");
} else if number % 3 == 0 {
  println!("number is divisible by 3");
} else if number % 2 == 0 {
  println!("number is divisible by 2");
} else {
  println!("number is not divisible by 4, 3, or 2");
}

if number { // error - expected bool, found integral variable
  println!("number was three");
}

// 
let number = if condition > 4 {
  if condition > 8 {
    11
  } else {
    5
  }
} else {
  3
};

println!("The value of number is: {}", number); // 5
```

### `loop`, `while` and `for`

```rust
loop {
  println!("again!");
  // ...
  break;
}

let mut number = 3;
while number != 0 {
  println!("{}!", number);
  number = number - 1;
  println!("LIFTOFF!!!");
}


let a = [10, 20, 30, 40, 50];
for element in a.iter() {
  // for문 내에서 element 삭제해도 panic X
  println!("the value is: {}", element);
}

// range
for number in (1..4).rev() {
  println!("{}!", number);
}
println!("LIFTOFF!!!");
```

### `match`

```rust
fn main() {
  // Destructuring values in match
  // - rust는 복잡한 type을 match로 wrapping structure를 벗겨내어 처리함
  // - destructuring type이 다를 경우 match, if let 할당 불가
  // - destructuring 
  let member = 13;
  match member {
      1 => println!("1"),
      2 | 3 | 4 => println!("2,3,4"),
      5..=10 => println!("5..10"),
      _ => println!("_")
  }

  // tuple
  let triple = (0, 1, -3);
  match triple {
      (0, y, z) => println!("{},{}", y, z),
      (1, ..) => println!("first is 1"), // .. ignore the rest
      _ => println!("??")
  }

  // arrays/slices
  let array = [3, -2, 6];
  match array {
      [0, second, third] => {
          println!("second {}, thrid {}", second, third);
      }
      [1, _, third] => println!("thrid {}", third), // ignore a value with _
      // The code below would not compile
      // [-1, second] => println!("compile error"),
      // store middle values to another array/slice
      [3, middle @.., last] => println!("{:?} {:?}", middle, last),
      _ => ()
  }

  // enums
  #[allow(dead_code)]
  #[derive(Debug)]
  enum Color {
      Red,
      Blue,
      Green,
      RGB(u32, u32, u32),
      CMYK(u32, u32, u32, u32)
  }
  let color = Color::RGB(122, 17, 40);
  match color {
      Color::Red   => println!("The color is Red!"),
      Color::Blue  => println!("The color is Blue!"),
      Color::Green => println!("The color is Green!"),
      Color::RGB(r, g, b) =>
          println!("Red: {}, green: {}, and blue: {}!", r, g, b),
      Color::CMYK(c, m, y, k) =>
          println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
              c, m, y, k),
      // all matching case must be in the scope.
  }

  // pointers/ref
  // dereference in matching
  let reference: &u32 = &4;
  match reference {
      &val => println!("{:?}", val) // if matched, it drops `&`
  }
  // To avoid the `&`, you dereference before matching.
  match *reference {
      val => println!("{:?}", val)
  }

  let _not_a_reference = 3;
  let ref _is_a_reference = 3; // &i32; explicit reference
  let value = 5;
  let mut mut_value = 6;

  let rr = match value { // create the reference of r
      ref r => r
  };
  println!("{:?}, {:?}", value, rr);
  match mut_value {
      ref mut m => {
          // Got a reference. Gotta dereference it before we can add anything to it.
          *m += 10;
          println!("We added 10. `mut_value`: {:?}", m);
      },
  }

  // structs
  #[allow(dead_code)]
  struct Foo {
      x: (u32, u32),
      y: u32
  }
  let foo = Foo {x: (10, 20), y: 30};
  match foo { 
      Foo { x: (1, b), y: c } => println!("b={:?} c={:?}", b, c),
      Foo {y, ..} => println!("y={:?}, others=Don't Care", y) // match any values
  }

  // Guards
  let pair = (3, 1);
  match pair {
      (x, y) if x == y => println!("Twins!"),
      (x, _) if x%2 == 1 => println!("The first odd!"),
      _ => println!("no correlation ...")
  }

  // binding to a value with inclusive range
  let num = 3;
  // @1..=10 inclusive range 사용해야 함
  // exclusive range = @1..10
  match num {
      n @1..=10 => println!("num in 1..10 {}", n),
      n => println!("num {}", n)
  }

  // binding to an enum variant value
  #[allow(dead_code)]
  fn some_number() -> Option<u32> {
      Some(42)
  }
  match some_number() {
      Some(n @ 42) => println!("Binding to an enum variant with value {:?}", n),
      Some(n) => println!("Binding to an enum variant {:?}", n),
      _ => ()
  }

  // match assign
  let option = Some(10);
  let i = match option {
      Some(i) => i,
      _ => panic!("?"),
  };
  println!("{}", i);

  // if let
  let num = Some(7);
  let letter: Option<i32> = None;
  // let emotion: Option<i32> = None;
  if let Some(i) = num {
      println!("i = {}", i);
  } else {
      println!("destructuring failed ...");
  }

  // match any enum value using if let
  if let None = letter {
      println!("letter is None");
  }

  if let Color::Blue = color {
      println!("Blue");
  } else if let Color::RGB(x, _, z @1..=100) = color {
      println!("x={}, z={}", x, z);
  } else {
      println!("{:?}", color);
  }

  // while let
  let mut optional = Some(0);
  while let Some(i) = optional {
      if i > 9 {
          optional = None;
      } else {
          optional = Some(i+ 1);
      }
  }
  optional = Some(0);
  loop { // while let과 동일 동작
      match optional {
          Some(i) => {
              if i > 9 {
                  optional = None;
              } else {
                  optional = Some(i+1);
              }
          },
          _ => break
      }
  }
}
```

## Result type

Rust는 result type은 열거형(enumerations)의 에러처리 정보

variants: enumeration data

```rust
// io::Result
pub type Result<T> = Result<T, Error>;

// std::result::Result
Enum std::result::Result
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## Reference


## methods

Rust는 다음과 같이 struct와 method를 정의한다.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // method definition in implement block
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Associated Functions
    fn square(size: u32) -> Rectangle {
        Rectangle{
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    // dbg! return the ownership of the input object
    let rect = dbg!(rect);

    println!(
        "The area of the rectangle {:?} is {} square pixels.",
        rect,
        rect.area()
    );

    let square = Rectangle::square(100);
    let rect1 = Rectangle { height: 50, width: 30 };
    let rect2 = Rectangle { height: 40, width: 10 };
    let rect3 = Rectangle { height: 45, width: 60 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can square hold rect1? {}", square.can_hold(&rect1));
}
```

- The `&self` is actually short for `self: &Self`.
- mutable 선언시, `&mut self`, field 수정 가능
- Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.
- Associated Function은 self가 없는 관련 함수 (class 함수와 유사), 생성자 함수에 주로 사용
- Associated Function은 struct에 대한 namespace syntax(`::`)로 접근/사용 가능
- 다수의 impl block 사용 가능

### Field Init Shorthand

생성함수에서 field name과 function argument을 동일하게 입력하여 짧게 쓰는 방법

```rust
fn build_user(email: String, username: String) -> User {
  User {
    email, // field init shorthand
    username,
    active: true,
    sign_in_count: 1,
  }
}
```

### Struct Update Syntax

앞서 사용한 인스턴스의 값을 사용해 구조체 업데이트하는 방법

```rust
fn main() {
  // --snip--
  let user1 = User {
      // init ...
  };

  let user2 = User {
      email: String::from("another@example.com"),
      ..user1 // user1의 값으로 structure update
  };
}
```

> Note that the struct update syntax uses = like an assignment; this is because it moves the data, just as we saw in the [“Ways Variables and Data Interact: Move”](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ways-variables-and-data-interact-move) section. In this example, we can no longer use user1 after creating user2 because the String in the username field of user1 was moved into user2. The types of active and sign_in_count are types that implement the Copy trait, so the behavior we discussed in the [“Stack-Only Data: Copy”](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy) section would apply.

## Tuple Structs

- Tuple과 유사한 구조체로 filed name이 없이 field type만을 정의한 구조체
- Named tuple

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

## Generics

Generics is the topic of generalizing types and functionalities to broader cases. This is extremely useful for reducing code duplication in many ways, but can call for rather involved syntax. 

- **Generic type**: Generic type parameter `<T>`가 사용된 모든 type
- **Concrete type**: generic type parameter가 사용되지 않은 (type이 명시된) 모든 type
- **Generic bounds**: Generic type이 가져야 할 type의 특성을 규정하기 위해 사용 `<T: Bounds>`; 주로 trait이 bound로 쓰임 e.g. `fn printme<T: std::fmt::Debug> (x: T)`
  - It places further constraints on the kind of the Generic types.
  - `where` can also be used to apply bounds in some cases to be more expressive.
- **Multiple generic bounds**: Multiple bounds for a single type can be applied with a `+`. Like normal, different types are separated with `,`. e.g. `fn compare_prints<T: Debug + Display>(t: &T)`
- **Associated types**: trait generics에서 내부적으로 사용될 type을 정의하여 가독성을 높이는 방법


> Good Aricle: [Using Generic Types in Rust](https://oswalt.dev/2021/06/using-generic-types-in-rust/)
> The combination of generics and traits in Rust gives us the same kind of flexibility that we are seeking in a dynamically typed language, but without any of the runtime tradeoffs. 

### Generic Type

```rust
struct A; // A concrete type
struct Single(A); // A concrete type; a tuple structure
struct SingleGen<T>(T); // A generic type
let _char: SingleGen<char> = SingleGen('a');
let _t    = SingleGen(A); // Uses `A` defined at the top.
let _i32  = SingleGen(6); // Uses `i32`.
let _char = SingleGen('a'); // Uses `char`.
```

### Generic functions

```rust
fn foo<T>(arg: T) { ... }
fn bar(s: SGen<A>) { ... } // not a generic function

generic::<char>(SGen('a')); // call with explicitly specified type parameters
generic(SGen('c')); // call with implicitly specified type parameters
```

### Generic Implementations (Generic methods)

```rust
struct S; // Concrete type `S`
struct GenericVal<T>(T); // Generic type `GenericVal`

// implementation 선언시 다음과 같이 특정 type을 명세할 수 있음
impl GenericVal<f32> {} // Specify `f32`
impl GenericVal<S> {} // Specify `S` as defined above

// `<T>` Must precede the type to remain generic
impl<T> GenericVal<T> {}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// NewStruct doesn't use any generic types,
// so we don't need to specify any here.
struct NewStruct {}
impl NewStruct {
    // We can still, however, define our own generic parameters
    // on an individual method as desired
    fn x<T>(&self, foo: T) -> T {
        foo
    }
}
```

### Bounds

### Where clause

Where clause는 generic bound에서 선행 표현되는 복잡한 type 정의를 후위에 표현하여 코드의 가독성을 높이는 방법

```rust
// 선행 표현
impl<K:Hash+Eq,V> HashMap<K, V> {}

// 후위 표현 with where
impl<K,V> HashMap<K, V>
    where K : Hash + Eq {}

impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

// Expressing bounds with a `where` clause
impl <A, D> MyTrait<A, D> for YourType where
    A: TraitB + TraitC,
    D: TraitE + TraitF {}
```

> [Rust RFC for `where`](https://github.com/rust-lang/rfcs/blob/master/text/0135-where.md)

### Associated types

The use of "Associated types" improves the overall readability of code by moving inner types locally into a trait as output types. Syntax for the trait definition is as follows:

```rust
// TRAIT = TRAIT_HEADER '{' TRAIT_ITEM* '}'
// TRAIT_ITEM =
//   ... <existing productions>
//   | 'const' IDENT ':' TYPE [ '=' CONST_EXP ] ';'
//   | 'type' IDENT [ ':' BOUNDS ] [ WHERE_CLAUSE ] [ '=' TYPE ] ';'
//   | 'lifetime' LIFETIME_IDENT ';'

trait Graph {
    type N: Show + Hash;
    type E: Show + Hash;
    ...
}

impl Graph for MyGraph {
    // Both MyNode and MyEdge must implement Show and Hash
    type N = MyNode;
    type E = MyEdge;
    ...
}

fn print_nodes<G: Graph>(g: &G) {
    // here, can assume G::N implements Show
    ...
}
```

https://github.com/rust-lang/rfcs/blob/master/text/0195-associated-items.md

### Phantom type parameters

- [phantom-data](https://doc.rust-lang.org/nomicon/phantom-data.html)
- [Phantom type parameters](https://doc.rust-lang.org/rust-by-example/generics/phantom.html)

## Traits

A `trait` is a collection of methods defined for an unknown type: `Self`. They can access other methods declared in the same trait. Traits can be implemented for any data type.

- Derive: 
- [Operation overloading](https://doc.rust-lang.org/core/ops/)
  - `Fn`, `FnMut`, and `FnOnce` traits for types that can be invoked like functions
  - `+`, `+=`, `-`, `*`, `/` ... for operator traits
- `Drop`: The `Drop` trait only has one method: `drop`, which is called automatically when an object goes out of scope.
  - The main use of the Drop trait is to free the resources that the implementor instance owns.
  - `Box`, `Vec`, `String`, `File`, and `Process`: the types implemented the `Drop` trait
- `Iterators`: The Iterator trait is used to implement iterators over collections such as arrays.
  - `fn next(&mut self) -> Option<Self::Item>` 구현해야 함
- dyn Trait: https://doc.rust-lang.org/rust-by-example/trait/dyn.html
- impl Trait: https://doc.rust-lang.org/rust-by-example/trait/impl_trait.html
- Copy Trait: 할당시 resource move가 아닌 copy
- Clone Trait: `.clone()`으로 명시적으로 copy
- Supertraits: Rust doesn't have "inheritance", but you can define a trait as being a superset of another trait.
- Disambiguating overlapping traits
- https://cotigao.medium.com/dyn-impl-and-trait-objects-rust-fd7280521bea
- https://modoocode.com/334


## To be considered

- Integer Overflow: https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow
- 역참조 강제(deref coercion)


## collections

### Box, stack and heap

All values in Rust are stack allocated by default. Values can be boxed (allocated on the heap) by creating a `Box<T>`. A box is a smart pointer to a heap allocated value of type `T`. When a box goes out of scope, its destructor is called, the inner object is destroyed, and the memory on the heap is freed.

```rust
use std::mem;

#[derive(Debug)]
#[allow(dead_code)]
struct Point {
    x: f64,
    y: f64,
}
#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point { x: 0f64, y: 0f64 }
}
fn boxed_origin() -> Box<Point> {
    // allocate a point to heap
    return Box::new(Point { x: 0.0, y: 0.0 });
}

// stack allocated variables
let point: Point = origin();
let rectangle: Rectangle = Rectangle {
    top_left: origin(),
    bottom_right: Point { x: 100.0, y: 100.0 },
};
// heap allocated variables
let box_point: Box<Point> = Box::new(origin());
let box_rectangle: Box<Rectangle> = Box::new(Rectangle {
    top_left: origin(),
    bottom_right: Point { x: 200.0, y: 200.0 },
});

// inner box in stack
let doubleIndirectBox: Box<Box<Point>> = Box::new(boxed_origin());

println!("{point:?}");
println!(
    "Point occupies {} bytes on the stack",
    mem::size_of_val(&point)
);
println!(
    "Rectangle occupies {} bytes on the stack",
    mem::size_of_val(&rectangle)
);

// box size == pointer size
println!(
    "Boxed point occupies {} bytes on the stack",
    mem::size_of_val(&box_point)
);
println!(
    "Boxed rectangle occupies {} bytes on the stack",
    mem::size_of_val(&box_rectangle)
);
println!(
    "Boxed box occupies {} bytes on the stack",
    mem::size_of_val(&doubleIndirectBox)
);

// Copy the data contained in `boxed_point` into `unboxed_point`
let unboxed_point: Point = *box_point;
println!(
    "Unboxed point occupies {} bytes on the stack",
    mem::size_of_val(&unboxed_point)
);
```

### String

- `to_string()`: Display trait이 구현된 모든 type에 사용 가능
- `+` 연산동작 추천 X
- String의 index 접근 금지됨
- `"नमस्ते".chars()`과 같이 문자소(grapheme)로 접근해야 함

```rust
let data = "initial contents";
let s = data.to_string();
// the method also works on a literal directly:
let s = "initial contents".to_string();
let s = String::from("initial contents"); // 위와 동일

let mut s = String::from("foo");
s.push_str("bar");

let mut s = String::from("lo");
s.push('l');

let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1은 여기서 이동되어 더이상 쓸 수 없음을 유의하세요

let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{}-{}-{}", s1, s2, s3);

for c in "नमस्ते".chars() { // 문자소 반환
    println!("{}", c);
}
for b in "नमस्ते".bytes() { // byte 반환
    println!("{}", b);
}
```

### hash map

## Useful code or crates


### Read stdin

```rust
use std::io;
// ...
// Read a string from stdin
let mut index = String::new();
io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");
let index: usize = index
    .trim()
    .parse()
    .expect("Index entered was not a number");

```

## Modules

Rust provides a powerful module system that can be used to hierarchically split code in logical units (modules), and manage visibility (public/private) between them.

A module is a collection of items: functions, structs, traits, impl blocks, and even other modules.

> Rust의 module이란 코드를 계층화/조직화 (namespace)하고, pub(public) 키워드를 통해 코드의 접근성 제어하여, 코드의 재사용성을 높이기 위한 방법

```rust
mod my_mod { // module my_mod
  pub fn public_func() {
    // ... 외부에서 호출 가능
  }
  fn private_func() {
    // ... module 외부에서 호출 불가
    // ... 같은 module 내에선 접근 가능
  }

  pub mod nested_mod { // module nested_mod
    pub(in crate::my_mod) fn nested_ {
      // ... crate::my_mode에서만 public function
    }
    pub(self) fn fname {
      // ... pub(self) == private
    }
    pub(super) fn fname {
      // ... parent module에서만 보임
    }
  }
  
  pub(crate) fn public_function_in_crate() {
    // ... 속한 crate에서만 접근 가능
  }
}
```

## Macro

Rust macro system의 macro 종류는 다음과 같다.

> - Declarative Macros
> - Procedural Macros
>   - Function-like macros
>   - Derive mode macros
>   - Attribute macros

### Declarative Macros

흔히 사용하는 "선언적" 형태의 매크로로 Rust의 Declarative Macro는 단순 문자열 치환이 아니라 Rust Abstract Syntax Tree를 직접 제어하는 방식이다.

- `macro_rules` allows users to define syntax extension in a declarative way.
- `MacroRule` = `MacroMatcher => MacroTranscriber`로 구성, `MacroRule`은 `;`으로 구분
- Each macro by example has a name, and one or more rules. Each rule has two parts: a matcher, describing the syntax that it matches, and a transcriber, describing the syntax that will replace a successfully matched invocation.
- Both the matcher and the transcriber must be surrounded by delimiters. Macros can expand to expressions, statements, items (including traits, impls, and foreign items), types, or patterns.
- **Transcribing**: 처음 match된 MacroMatcher의 MacroTranscriber로 code 변환하고 못찾거나 오류 발생시 중단
- `()`, `[]`, `{}` 모든 괄호는 MacroMatch, MacroRule에서 모두 사용가능


### Procedural Macros

컴파일 시점에 macro 함수를 실행시켜 코드를 업데이트하는 **매크로**로 `#[proc_macro]`로 선언된 macro 함수는 컴파일 시점에 TokenStream을 받아 TokenStream을 출력하고, 그 출력된 TokenStream으로 코드를 치환해 컴파일하는 매크로이다.

- Procedural Macro를 사용해 원본 코드에는 없는 함수를 생성하거나 (function generation),
- macro로 정의된 namespace block을 TokenStream으로 받아 해당 block을 치환할 수 있다.
- 이러한 동작을 통해 compile time에 Python의 decorator와 같은 함수를 작성할 수 있다.
- 추가 정보는 [good article for rust macro](http://www.secmem.org/blog/2019/02/10/rust-procedural-macros-by-example/)에 너무 잘 설명되어 있어 생략한다.

### Links for macro

> - [good article for rust macro](http://www.secmem.org/blog/2019/02/10/rust-procedural-macros-by-example/)
> - [macros-by-example](https://doc.rust-lang.org/reference/macros-by-example.html)
> - [Rust by example](https://doc.rust-lang.org/rust-by-example/macros.html)

## Testing

- `#[test]` 함수가 test함수임을 선언
- `cargo test -- --nocapture --test-threads=1`
- `cargo test -- -h`으로 testing option 확인해보기
- `cargo test TESTNAME`: 단일 테스트 항목 실행
- `cargo test -- --ignored`: `#[ignore]`로 설정된 테스트 항목 테스트
- unit test는 mod test 만들고, `#[cfg(test)]` cfg attribute를 설정하여 구성; `cargo test`시에만 컴파일/실행

```rust
#[derive(Debug, PartialEq, Eq)]
pub struct Rectangle {
  length: u32,
  width: u32,
}

impl Rectangle {
  pub fn can_hold(&self, other: &Rectangle) -> bool {
      self.length > other.length && self.width > other.width
  }
}

#[allow(dead_code)]
fn add_two(a: i32) -> i32 {
  if a > 100 {
      panic!("value must be less than or equal to 100.");
  }
  a + 2
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn larger_can_hold_smaller() {
      let larger = Rectangle { length: 8, width: 7 };
      let smaller = Rectangle { length: 5, width: 1 };

      assert!(larger.can_hold(&smaller), "{:?}", larger);
  }

  #[test]
  fn smaller_cannot_hold_larger() {
      let larger = Rectangle { length: 8, width: 7 };
      let smaller = Rectangle { length: 5, width: 1 };

      assert!(!smaller.can_hold(&larger), "{:?}", smaller);
  }

  #[test]
  fn it_adds_two() {
      // assert_eq!와 assert_ne!는 
      // 각각 ==과 != 연산자 사용하므로
      // PartialEq와 Debug 트레잇을 구현해 함
      assert_eq!(4, add_two(2));
      assert_ne!(4, add_two(3));
  }

  #[test]
  #[should_panic]
  fn greater_than_100() {
      add_two(200);
  }

  #[test]
  #[should_panic(expected = "value must be less than or equal to 100")]
  fn greater_than_100_2() {
      add_two(200);
  }
}
```

## Rust Attributes

> - [rust-by-example](https://doc.rust-lang.org/rust-by-example/attribute.html)
> - [sjquant.tistory.com](https://sjquant.tistory.com/53)

An attribute is **metadata** applied to some `module`, `crate` or `item`. This metadata can be used to/for:

- conditional compilation of code
- set crate name, version and type (binary or library)
- disable lints (warnings)
- enable compiler features (macros, glob imports, etc.)
- link to a foreign library
- mark functions as unit tests
- mark functions that will be part of a benchmark

### Scope

- `#![crate_attribute]` for a whole crate
- `#[item_attribute]`: for a module or item

다른 문서에서는 

- `#![Attr]`: InnerAttribute로 선언된 범위 내에 적용
- `#[Attr]`: OuterAttribute로 선언이후 오는 module, item에 적용

### Attribute arguments

Attribute는 Argument를 가질 수 있음

- `#[attribute = "value"]`
- `#[attribute(key = "value")]`
- `#[attribute(value)]`
- `#[attribute(value, value2, value3, value4, value5)]`

### Frequently Used Attributes

- `#[allow(dead_code)]`: used to disable linting of the following code block; the compiler option?
- `#![crate_name = "rary"]`: The library is named "rary"
- `#![crate_type = "lib"]`: This crate is a library; When the crate_type attribute is used, we no longer need to pass the `--crate-type` flag to rustc.
- `#[cfg(target_os = "linux")]` This function only gets compiled if the target OS is linux
- `#[cfg(not(target_os = "linux"))]`: And this function only gets compiled if the target OS is *not* linux

> ❗`cfg!`, unlike `#[cfg]`, it is a **macro** that does not remove any code and only evaluates to true or false.
> ```
> if cfg!(target_os = "linux") {
>   println!("Yes. It's definitely linux!");
> } else {
>   println!("Yes. It's definitely *not* linux!");
> }
> ```

- `#[test]`: used for functional test
- `#[cfg_attr(a, b)]`: 만약 #[cfg(a)]를 만족하면 #[b] attribute를 적용



### Custom cfg

사용자 `cfg`도 만들 수 있음!

```rust
#[cfg(mine)]
fn cond_function() {
    println!("mine cond!!!");
}

#[cfg(not(mine))]
fn cond_function() {
    println!("not mine cond!!!");
}

fn main() {
   cond_function();
}
```

```bash
rustc --cfg mine main.rs && ./main
RUSTFLAGS='--cfg mine' cargo run
```

### `derive` attribute

- 특정한 Trait에 대한 기본적인 구현(impl)을 간편하게 제공
- 기본적인 구현은 이미 정해져 있으며, 이런 Trait을 derivable 하다고 함
- derivable Trait: `Eq`, `PartialEq`, `Copy`, `Clone`, `Debug` ...?

```rust
#[derive(PartialEq, Clone)]
struct Foo<T> {
    a: i32,
    b: T,
}
```

The following is a list of derivable traits:

- Comparison traits: `Eq`, `PartialEq`, `Ord`, `PartialOrd`.
- `Clone`, to create `T` from `&T` via a copy.
- `Copy`, to give a type 'copy semantics' instead of 'move semantics'.
- `Hash`, to compute a hash from `&T`.
- `Default`, to create an empty instance of a data type.
- `Debug`, to format a value using the `{:?}` formatter.

## Keywords

### `crate`

```rust

// 사용하는 외부 crate 선언
extern crate rand;
extern crate my_crate as thing; // the alias of my_crate in my project
extern crate std; // implicitly added to the root of every Rust project

// item의 visibility를 현재 crate에 포함된 module로만 한정
pub(crate) enum CoolMarkerType { }

// The root of the current crate
crate::foo::bar
```


## Good answer to understand

- [whats-the-difference-between-self-and-self](https://stackoverflow.com/questions/32304595/whats-the-difference-between-self-and-self)
- [Why is the `Sized` bound necessary in this trait?](https://stackoverflow.com/questions/30938499/why-is-the-sized-bound-necessary-in-this-trait)

## Associated items

This following RFC extends traits with associated items, which make generic programming more convenient, scalable, and powerful. In particular, traits will consist of a set of methods, together with:

> https://github.com/rust-lang/rfcs/blob/master/text/0195-associated-items.md

- Associated functions (already present as "static" functions)
- Associated consts
- Associated types
- Associated lifetimes

## Rust RFC

https://github.com/rust-lang/rfcs/tree/master/text


## FFI (Foreign Function Interface)

다른 언어 코드에서 rust 함수를 호출하거나, rust 코드에서 외부 함수를 호출하는 방법

### Calling foreign functions

아래와 같이 `libc`는 crate으로 구현(wrapping)되어 있음

```toml
[dependencies]
libc = "0.2.0"
```

- 외부 함수 코드는 safe하다 가정하고, `unsafe`로 감싸 코드를 검증하지 않음
- wrapping 함수를 만드는게 일반적임
- 외부 함수가 자원 해제를 하지 않을 경우 직접 Drop trait으로 자원 해제해야 함

```rust
use libc::{c_int, size_t};

#[link(name = "snappy")] // 외부 library
extern { // library내 함수 목록
  fn snappy_compress(input: *const u8,
                      input_length: size_t,
                      compressed: *mut u8,
                      compressed_length: *mut size_t) -> c_int;
  fn snappy_uncompress(compressed: *const u8,
                        compressed_length: size_t,
                        uncompressed: *mut u8,
                        uncompressed_length: *mut size_t) -> c_int;
  fn snappy_max_compressed_length(source_length: size_t) -> size_t;
  fn snappy_uncompressed_length(compressed: *const u8,
                                compressed_length: size_t,
                                result: *mut size_t) -> c_int;
  fn snappy_validate_compressed_buffer(compressed: *const u8,
                                        compressed_length: size_t) -> c_int;
}

pub fn validate_compressed_buffer(src: &[u8]) -> bool {
  unsafe {
      snappy_validate_compressed_buffer(src.as_ptr(), src.len() as size_t) == 0
  }
}

pub fn compress(src: &[u8]) -> Vec<u8> {
  unsafe {
      let srclen = src.len() as size_t;
      let psrc = src.as_ptr();

      let mut dstlen = snappy_max_compressed_length(srclen);
      let mut dst = Vec::with_capacity(dstlen as usize);
      let pdst = dst.as_mut_ptr();

      snappy_compress(psrc, srclen, pdst, &mut dstlen);
      dst.set_len(dstlen as usize);
      dst
  }
}

pub fn uncompress(src: &[u8]) -> Option<Vec<u8>> {
  unsafe {
      let srclen = src.len() as size_t;
      let psrc = src.as_ptr();

      let mut dstlen: size_t = 0;
      snappy_uncompressed_length(psrc, srclen, &mut dstlen);

      let mut dst = Vec::with_capacity(dstlen as usize);
      let pdst = dst.as_mut_ptr();

      if snappy_uncompress(psrc, srclen, pdst, &mut dstlen) == 0 {
          dst.set_len(dstlen as usize);
          Some(dst)
      } else {
          None // SNAPPY_INVALID_INPUT
      }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn valid() {
      let d = vec![0xde, 0xad, 0xd0, 0x0d];
      let c: &[u8] = &compress(&d);
      assert!(validate_compressed_buffer(c));
      assert!(uncompress(c) == Some(d));
  }

  #[test]
  fn invalid() {
      let d = vec![0, 0, 0, 0];
      assert!(!validate_compressed_buffer(&d));
      assert!(uncompress(&d).is_none());
  }

  #[test]
  fn empty() {
      let d = vec![];
      assert!(!validate_compressed_buffer(&d));
      assert!(uncompress(&d).is_none());
      let c = compress(&d);
      assert!(validate_compressed_buffer(&c));
      assert!(uncompress(&c) == Some(d));
  }
}

fn main() {
    let x = unsafe { snappy_max_compressed_length(100) };
    println!("max compressed length of a 100 byte buffer: {}", x);
}
```

### Calling Rust code from C

lib일 경우 C에서 쉽게 rust 함수에 접근 가능함

```rust
#[no_mangle] // turns off Rust's name mangling
pub extern "C" fn hello_from_rust() { // extern "C"로 C에서 호출가능한 형식으로 함수명 유지
    println!("Hello from Rust!");
}
```

Cargo.toml에 C dynamic library 명시 (staticlib도 가능)

```toml
[lib]
crate-type = ["cdylib"]
```

```c
int main(void) {
  hello_from_rust();
  return 0;
}
```

- Compile with `-L` and `-l` options: `gcc call_rust.c -o call_rust -lrust_from_c -L./target/debug`
- `export`되는 함수의 C header file의 자동 생성: https://github.com/eqrion/cbindgen

### 가변 인자 함수 (variadic functions)

`...` 사용해 표현, `unsafe`로 validation skip

```rust
extern {
    fn foo(x: i32, ...);
}

fn main() {
    unsafe {
        foo(10, 20, 30, 40, 50);
    }
}
```

## Logging

[https://crates.io/crates/log](https://crates.io/crates/log)은 logging abstraction interface 만을 제공할 뿐 log output을 제공하지 않는다.
따라서 [https://crates.io/crates/log](https://crates.io/crates/log)의 `In executables`에 logger를 import해야 실제 log를 화면에 출력할 수 있다.

```toml
[dependencies]
    log = "0.4"
    env_logger = "0.9.0" # 기본 logger
```

```rust
mod foo {
    mod bar {
        pub fn run() {
            log::warn!("[bar] warn");
            log::info!("[bar] info");
            log::debug!("[bar] debug");
        }
    }

    pub fn run() {
        log::warn!("[foo] warn");
        log::info!("[foo] info");
        log::debug!("[foo] debug");
        bar::run();
    }
}

fn main() {
    env_logger::init();
    log::warn!("[root] warn");
    log::info!("[root] info");
    log::debug!("[root] debug");
    foo::run();
}
```

`RUST_LOG`로 env_logger output을 끄거나 켤 수 있으며, logging은 module의 계층 구조에 따라
logging할 target module을 logging level과 함께 RUST_LOG에 지정하면, log가 출력된다.

```bash
RUST_LOG="warn,test::foo=info,test::foo::bar=debug" ./test # Rust binary
```

이외 사용자의 환경변수 설정에 따라 log를 켜거나, log 포맷, log 저장위치등을 변경 가능하다. 찾아 보도록!

```rust
// 우선순위 위 ==> 아래
//! [`error!`]
//! [`warn!`]
//! [`info!`]
//! [`debug!`]
//! [`trace!`]
```

## lib.rs and main.rs

rust library 작성시 main.rs를 통해 동작확인 가능

https://stackoverflow.com/questions/26946646/package-with-both-a-library-and-a-binary

