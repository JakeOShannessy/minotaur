#[deny(unsafe_code)]
extern crate structopt;

use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use structopt::clap::arg_enum;
use structopt::StructOpt;

arg_enum! {
    #[derive(Debug)]
    enum Algorithm {
        BinaryTree,
        Sidewinder,
    }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case", raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Opt {
    /// Maze generating algorithm
    #[structopt(short = "a", long = "algorithm", default_value = "BinaryTree", case_insensitive = true)]
    algorithm: Algorithm,
    /// Maze width in number of cells
    #[structopt(short = "x", long = "width", default_value = "5")]
    width: usize,
    /// Maze height in number of cells
    #[structopt(short = "y", long = "height", default_value = "5")]
    height: usize,
    /// Output file. Uses the file extension to determine whether to save an Ascii art image or a real image.
    #[structopt(short = "o", long = "output", default_value = "/dev/stdout", case_insensitive = true)]
    output: String,
    /// Seed for random number generator
    #[structopt(short = "s", long = "seed")]
    seed: Option<u64>,
    /// Cell size when saving to an image
    #[structopt(long = "cell-size", default_value = "10")]
    cell_size: usize,

}

fn main() -> std::io::Result<()> {
    use minotaur::Grid;
    use Algorithm::*;

    let opt = Opt::from_args();

    let grid = match opt.algorithm {
        BinaryTree => Grid::binary_tree(opt.width, opt.height, opt.seed),
        Sidewinder => Grid::sidewinder(opt.width, opt.height, opt.seed),
    };

    let filepath = Path::new(&opt.output);

    match filepath.extension().and_then(OsStr::to_str) {
        Some("txt") | None => {
            let file = File::create(filepath)?;
            let mut file_writer = BufWriter::new(file);
            file_writer.write_all(format!("{}", grid).as_bytes())?;
        }
        _ => {
            let image = grid.to_image(opt.cell_size);
            image.save(opt.output)?;
        }
    };

    Ok(())
}
