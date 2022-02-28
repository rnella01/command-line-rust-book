# Chapter 1. Truth or Consequences

## Commands
```bash
# Create project with cargo
$ cargo new hello
# verify the generated program works
$ cargo run
```
- If you would like for Cargo to not print status messages about compiling and running the code, 
  - you can use the `-q`, or -`-quiet`, option
- To run executable in `target/debug/hello`
```shell
$ ./target/debug/hello
```
- Run the program and manually check the exit value
```shell
$ cargo run --quiet --bin true
$ echo $?

$ cargo run --quiet --bin false
$ echo $?
```