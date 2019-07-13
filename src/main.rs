#![deny(unsafe_code)]

extern crate structopt;

use bincode;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use structopt::clap::{arg_enum, AppSettings};
use structopt::StructOpt;

arg_enum! {
    #[derive(Debug)]
    enum Algorithm {
        BinaryTree,
        Sidewinder,
        AldousBroder,
        Wilsons,
        HuntAndKill,
        RecursiveBacktracker,
    }
}

fn parse_hex_to_rgb(src: &str) -> Result<image::Rgb<u8>, ParseHexError> {
    let src = if src.starts_with('#') { &src[1..] } else { src };

    if src.len() != 6 {
        return Err(ParseHexError::Length(src.to_string()));
    }

    let mut rgb = [0_u8; 3];
    rgb[0] = u8::from_str_radix(&src[..2], 16)?;
    rgb[1] = u8::from_str_radix(&src[2..4], 16)?;
    rgb[2] = u8::from_str_radix(&src[4..6], 16)?;
    Ok(image::Rgb(rgb))
}

#[derive(Debug)]
enum ParseHexError {
    IntError(std::num::ParseIntError),
    Length(String),
}

impl std::fmt::Display for ParseHexError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseHexError::Length(e) => write!(
                f,
                "Expected a 6 charactor color value in hex, but got: {:?}",
                e
            ),
            // This is a wrapper, so defer to the underlying types' implementation of `fmt`.
            ParseHexError::IntError(ref e) => e.fmt(f),
        }
    }
}

impl From<std::num::ParseIntError> for ParseHexError {
    fn from(err: std::num::ParseIntError) -> ParseHexError {
        ParseHexError::IntError(err)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    rename_all = "kebab-case",
    raw(global_settings = "&[AppSettings::ColoredHelp]")
)]
struct Opt {
    /// Maze generating algorithm
    #[structopt(
        short = "a",
        long = "algorithm",
        default_value = "AldousBroder",
        case_insensitive = true,
        raw(possible_values = "&Algorithm::variants()"),
        display_order = 0_usize
    )]
    algorithm: Algorithm,
    /// Maze width in number of cells
    #[structopt(
        short = "x",
        long = "width",
        default_value = "5",
        display_order = 1_usize
    )]
    width: usize,
    /// Maze height in number of cells
    #[structopt(
        short = "y",
        long = "height",
        default_value = "5",
        display_order = 2_usize
    )]
    height: usize,
    /// Output file. Can be ".png" for an image, ".mz" to store the maze inself for later loading, otherwise, saves as ASCII art
    #[structopt(short = "o", long = "output", default_value = "/dev/stdout")]
    output: String,
    /// Input file of ".mz" stored from a previous run
    #[structopt(short = "i", long = "input")]
    input: Option<String>,
    /// Seed for random number generator
    #[structopt(short = "s", long = "seed")]
    seed: Option<u64>,
    /// Cell size when saving to an image file
    #[structopt(long = "cell-size", default_value = "10")]
    cell_size: usize,
    /// Wall size when saving to an image file
    #[structopt(long = "wall-size", default_value = "1")]
    wall_size: usize,
    /// Background color when saving to an image file
    #[structopt(
        long = "background-color",
        default_value = "#FFFFFF",
        parse(try_from_str = "parse_hex_to_rgb")
    )]
    background_color: image::Rgb<u8>,
    /// Wall color when saving to an image file
    #[structopt(
        long = "wall-color",
        default_value = "#000000",
        parse(try_from_str = "parse_hex_to_rgb")
    )]
    wall_color: image::Rgb<u8>,
}

fn main() -> std::io::Result<()> {
    use minotaur::Grid;
    use Algorithm::*;

    let opt = Opt::from_args();

    let grid = if let Some(input) = opt.input {
        let f = File::open(input)?;
        bincode::deserialize_from(f).expect("Could not parse .mz file")
    } else {
        match opt.algorithm {
            BinaryTree => {
                let mut grid = Grid::new(opt.width, opt.height);
                grid.binary_tree(opt.seed);
                grid
            }
            Sidewinder => {
                let mut grid = Grid::new(opt.width, opt.height);
                grid.sidewinder(opt.seed);
                grid
            }
            AldousBroder => {
                let mut grid = Grid::new(opt.width, opt.height);
                grid.aldous_broder(opt.seed);
                grid
            }
            Wilsons => {
                let mut grid = Grid::new(opt.width, opt.height);
                grid.wilsons(opt.seed);
                grid
            }
            HuntAndKill => {
                let mut grid = Grid::new(opt.width, opt.height);
                grid.hunt_and_kill(opt.seed);
                grid
            }
            RecursiveBacktracker => {
                let mut grid = Grid::new(opt.width, opt.height);
                grid.recursive_backtracker(opt.seed);
                grid
            }
        }
    };

    let filepath = Path::new(&opt.output);

    match filepath.extension().and_then(OsStr::to_str) {
        Some("png") => {
            let image = grid.to_image(
                opt.cell_size,
                opt.wall_size,
                opt.background_color,
                opt.wall_color,
            );
            image.save(opt.output)?;
        }
        Some("mz") => {
            let encoded = bincode::serialize(&grid).unwrap();
            let file = File::create(filepath)?;
            let mut file_writer = BufWriter::new(file);
            file_writer.write_all(&encoded)?;
        }
        _ => {
            let file = File::create(filepath)?;
            let mut file_writer = BufWriter::new(file);
            file_writer.write_all(format!("{}", grid).as_bytes())?;
        }
    };

    Ok(())
}
