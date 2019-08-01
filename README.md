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
./target/release/minotaur --help
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
            Maze generating algorithm [default: AldousBroder]  [possible values: BinaryTree,
            Sidewinder, AldousBroder, Wilsons, HuntAndKill, RecursiveBacktracker]
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
./target/release/minotaur --seed 12345678 -a binarytree
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
./target/release/minotaur --seed 12345678 -o examples/maze.png --cell-size 50 --wall-size 5 -a binarytree
```
![Generated maze](examples/maze.png?raw=true "Generated Maze")

## Benchmarks

```
cargo +nightly bench
   Compiling minotaur v0.2.0 (/home/jonstites/Code/rust/minotaur)
    Finished release [optimized] target(s) in 3.65s
     Running target/release/deps/minotaur-7f054a3c582211a2

running 8 tests
test tests::test_aldous_broder ... ignored
test tests::test_aldous_broder_all_mazes ... ignored
test tests::test_binary_tree ... ignored
test tests::test_hunt_and_kill ... ignored
test tests::test_recursive_backtracker ... ignored
test tests::test_sidewinder ... ignored
test tests::test_wilsons ... ignored
test tests::test_wilsons_all_mazes ... ignored

test result: ok. 0 passed; 0 failed; 8 ignored; 0 measured; 0 filtered out

     Running target/release/deps/minotaur-409fa2e8229853a7

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/release/deps/benches-7671d38865a46d79

running 12 tests
test bench::aldous_broder::generate_100_x_100         ... bench:   8,729,860 ns/iter (+/- 2,127,433)
test bench::aldous_broder::generate_10_x_10           ... bench:      32,356 ns/iter (+/- 1,810)
test bench::binary_tree::generate_100_x_100           ... bench:     127,001 ns/iter (+/- 5,049)
test bench::binary_tree::generate_10_x_10             ... bench:       1,990 ns/iter (+/- 97)
test bench::hunt_and_kill::generate_100_x_100         ... bench:   3,741,404 ns/iter (+/- 224,387)
test bench::hunt_and_kill::generate_10_x_10           ... bench:      31,779 ns/iter (+/- 2,358)
test bench::recursive_backtracker::generate_100_x_100 ... bench:   2,928,601 ns/iter (+/- 305,673)
test bench::recursive_backtracker::generate_10_x_10   ... bench:      27,949 ns/iter (+/- 3,844)
test bench::sidewinder::generate_100_x_100            ... bench:     186,256 ns/iter (+/- 17,789)
test bench::sidewinder::generate_10_x_10              ... bench:       2,595 ns/iter (+/- 121)
test bench::wilsons::generate_100_x_100               ... bench:   8,551,824 ns/iter (+/- 3,229,443)
test bench::wilsons::generate_10_x_10                 ... bench:      57,559 ns/iter (+/- 3,439)

test result: ok. 0 passed; 0 failed; 0 ignored; 12 measured; 0 filtered out
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