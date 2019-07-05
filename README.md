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
minotaur 0.1.0
```

## Usage

```
minotaur 0.1.0
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
    -o, --output <output>
            Output file. Can be ".png" for an image - otherwise, saves as ASCII art [default: /dev/stdout]

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