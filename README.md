# Minotaur

`minotaur` is a command-line maze generator.

[![Build Status](https://travis-ci.com/jonstites/minotaur.svg?branch=master)](https://travis-ci.com/jonstites/minotaur)
[![Cargo Version](https://img.shields.io/crates/v/minotaur.svg)](https://crates.io/crates/minotaur)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/minotaur.svg)](#license)

## Installation

Currently, [the Rust toolchain](https://www.rust-lang.org/tools/install) is required for installation:

From [crates.io](https://crates.io/crates/minotaur):
```bash
cargo install minotaur
```

From source:
```bash
$ git clone git@github.com:jonstites/minotaur
$ cd minotaur
$ cargo build --release
$ ./target/release/minotaur --version
minotaur 0.2.0
```

## Usage

```
./target/release/minotaur -h
minotaur 0.2.0
Jonathan Stites <mail@jonstites.com>
A command-line program for generating mazes.

USAGE:
    minotaur [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --algorithm <algorithm>
            Maze generating algorithm [default: BinaryTree]  [possible values: BinaryTree,
            Sidewinder]
    -x, --width <width>                          Maze width in number of cells [default: 5]
    -y, --height <height>                        Maze height in number of cells [default: 5]
        --background-color <background-color>    Background color when saving to an image file [default: #FFFFFF]
        --cell-size <cell-size>                  Cell size when saving to an image file [default: 10]
    -i, --input <input>                          Input file of ".mz" stored from a previous run
    -o, --output <output>
            Output file. Can be ".png" for an image, ".mz" to store the maze inself for later loading, otherwise, saves
            as ASCII art [default: /dev/stdout]
    -s, --seed <seed>                            Seed for random number generator
        --wall-color <wall-color>                Wall color when saving to an image file [default: #000000]
        --wall-size <wall-size>                  Wall size when saving to an image file [default: 1]
```

## Examples

```bash
./target/release/minotaur --seed 12345678
+---+---+---+---+---+
|                   |
+   +   +---+---+   +
|   |   |           |
+---+   +   +---+   +
|       |   |       |
+   +---+   +---+   +
|   |       |       |
+   +   +---+---+   +
|   |   |           |
+---+---+---+---+---+
```

```bash
./target/release/minotaur --seed 12345678 -o examples/maze.png --cell-size 50 --wall-size 5
```
![Generated maze](examples/maze.png?raw=true "Generated Maze")

## Benchmarks

```
$ cargo bench
   Compiling minotaur v0.2.0 (/home/jonstites/Code/rust/minotaur)
    Finished release [optimized] target(s) in 2.18s
     Running target/release/deps/minotaur-7f054a3c582211a2

running 2 tests
test tests::test_binary_tree ... ignored
test tests::test_sidewinder ... ignored

test result: ok. 0 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out

     Running target/release/deps/minotaur-409fa2e8229853a7

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/release/deps/benches-7671d38865a46d79

running 4 tests
test bench::binary_tree::generate_100_x_100 ... bench:     110,879 ns/iter (+/- 3,735)
test bench::binary_tree::generate_10_x_10   ... bench:       1,740 ns/iter (+/- 35)
test bench::sidewinder::generate_100_x_100  ... bench:     166,913 ns/iter (+/- 1,214)
test bench::sidewinder::generate_10_x_10    ... bench:       2,310 ns/iter (+/- 28)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.