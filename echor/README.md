# Chapter 2. Test for Echo

- The lack of any return type for `main` implies that the function returns what Rust calls the `unit type`
  - The [unit type](https://doc.rust-lang.org/std/primitive.unit.html) is like an empty value and is signified with a set of empty parentheses: ()
- The () type has exactly one value (), 
  - and is used when there is no other meaningful value that could be returned. 
  - () is most commonly seen implicitly: functions without a `-> ...` implicitly have return type ()
  - these are equivalent:
```rust
fn long() -> () {}

fn short() {}
```
- [Function `std::env::args`](https://doc.rust-lang.org/stable/std/env/fn.args.html)
```rust
// pub fn args() -> Args
use std::env;

fn main() {
// Prints each argument on a separate line
    for argument in env::args() {
        println!("{}", argument);
    }
}
```
- If an object implements the `Display` trait, then it can be formatted for output
  - in the println! macro:
    - `{}` placeholder requires the `std::fmt::Display` trait
    - `{:?}` uses `std::fmt::Debug` trait
    - `{:#?}` for pretty printing
- [Crate clap](https://docs.rs/clap/latest/clap/index.html)
  - Create your command-line parser, with all of the bells and whistles, declaratively or procedurally
```rust
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
```
- `clap = "3"` in Cargo.toml => fine using the latest version in the major version “3.x”
- `$ cargo build` to build the new binary, not run it. It will download the newly added dependency and its transitive dependencies as well!
```shell
$ cargo build
   Compiling libc v0.2.119
   Compiling memchr v2.4.1
   Compiling autocfg v1.1.0
   Compiling hashbrown v0.11.2
   Compiling textwrap v0.14.2
   Compiling strsim v0.10.0
   Compiling termcolor v1.1.2
   Compiling bitflags v1.3.2
   Compiling indexmap v1.8.0
   Compiling os_str_bytes v6.0.0
   Compiling atty v0.2.14
   Compiling clap v3.1.2
   Compiling echor v0.1.0 (/Users/rnella01/workspaces/rust/command-line-rust-book/echor)
    Finished dev [unoptimized + debuginfo] target(s) in 7.11s
```
- Cargo places the downloaded source code into `.cargo` in the user's home directory 
  - the build artifacts go into the `target/debug/deps` directory of the project
- Struct `clap::App` is Deprecated since 3.1.0: Replaced with `clap::Command`
  - `pub type Command<'help> = App<'help>;`
```rust
fn main() {
  let _m = Command::new("My Program")
          .author("Me, me@mail.com")
          .version("1.0.2")
          .about("Explains in brief what the program does")
          .arg(
            Arg::new("in_file").index(1)
          )
          .after_help("Longer explanation to appear after the options when \
                 displaying the help information from --help or -h")
          .get_matches();

// Your program logic starts here...
}
```