#![deny(unsafe_code)]

#[macro_use]
extern crate bitflags;
extern crate image;

use image::{ImageBuffer, RgbImage};
use rand::{Rng, SeedableRng};
use rand_pcg::Lcg64Xsh32;
use serde::{Deserialize, Serialize};

/*
Cell represents a single square in a maze's Grid.
It stores links in the four directions.
For example, if NORTH is true, then this Cell has
an open passage to the Cell above it.
Otherwise, there is a wall between the two Cells.

It would be a logic error if this Cell had
NORTH, but its northern neighbor did not have SOUTH.
*/
bitflags! {
    #[derive(Default, Serialize, Deserialize,)]
    pub struct Cell: u8 {
        const NORTH = 0b0001;
        const SOUTH = 0b0010;
        const EAST =  0b0100;
        const WEST =  0b1000;
    }
}

/*
Grid represents a maze.
*/
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Grid {
    // Must be at least 1x1
    pub fn binary_tree(width: usize, height: usize, seed: Option<u64>) -> Grid {
        let mut cells = vec![Cell::default(); height * width];

        // For all cells in the northernmost row, there are no
        // northern neighbors. So link with eastern neighbor,
        // except the corner, which has neither a northern nor
        // eastern neighbor.
        let mut rng = match seed {
            Some(seed) => Lcg64Xsh32::seed_from_u64(seed),
            None => Lcg64Xsh32::from_entropy(),
        };

        for i in 0..cells.len() {
            let east_edge = (i + 1) % width == 0;
            let north_edge = i < width;
            let choose_north = rng.gen();

            if !north_edge && (east_edge || choose_north) {
                cells[i] |= Cell::NORTH;
                cells[i - width] |= Cell::SOUTH;
            } else if !east_edge {
                cells[i] |= Cell::EAST;
                cells[i + 1] |= Cell::WEST;
            }
        }

        Grid {
            cells,
            width,
            height,
        }
    }

    pub fn sidewinder(width: usize, height: usize, seed: Option<u64>) -> Grid {
        let mut cells = vec![Cell::default(); height * width];

        let mut run_start = width;
        let mut rng = match seed {
            Some(seed) => Lcg64Xsh32::seed_from_u64(seed),
            None => Lcg64Xsh32::from_entropy(),
        };
        for i in 0..cells.len() {
            let east_edge = (i + 1) % width == 0;
            let north_edge = i < width;
            let choose_north = rng.gen();

            if !north_edge && (east_edge || choose_north) {
                let chosen = rng.gen_range(run_start, i + 1);
                cells[chosen] |= Cell::NORTH;
                cells[chosen - width] |= Cell::SOUTH;
                run_start = i + 1;
            } else if !east_edge {
                cells[i] |= Cell::EAST;
                cells[i + 1] |= Cell::WEST;
            } else {
                run_start = i + 1;
            }
        }

        Grid {
            cells,
            width,
            height,
        }
    }

    pub fn to_image(
        &self,
        cell_size: usize,
        wall_size: usize,
        background_pixel: image::Rgb<u8>,
        wall_pixel: image::Rgb<u8>,
    ) -> RgbImage {
        let image_width = cell_size * self.width + wall_size;
        let image_height = cell_size * self.height + wall_size;

        let mut image =
            ImageBuffer::from_pixel(image_width as u32, image_height as u32, background_pixel);

        for (cell_index, cell) in self.cells.iter().enumerate() {
            let x = (cell_index % self.width) * cell_size;
            let y = (cell_index / self.width) * cell_size;

            if !cell.contains(Cell::NORTH) {
                for wall_offset in 0..wall_size {
                    for cell_offset in 0..=cell_size {
                        let x_temp = x + cell_offset;
                        let y_temp = y + wall_offset;
                        image.put_pixel(x_temp as u32, y_temp as u32, wall_pixel)
                    }
                }
            }

            if !cell.contains(Cell::SOUTH) {
                for wall_offset in 0..wall_size {
                    for cell_offset in 0..(cell_size + wall_size) {
                        let x_temp = x + cell_offset;
                        let y_temp = y + cell_size + wall_offset;
                        image.put_pixel(x_temp as u32, y_temp as u32, wall_pixel)
                    }
                }
            }

            if !cell.contains(Cell::WEST) {
                for wall_offset in 0..wall_size {
                    for cell_offset in 0..=cell_size {
                        let y_temp = y + cell_offset;
                        let x_temp = x + wall_offset;
                        image.put_pixel(x_temp as u32, y_temp as u32, wall_pixel);
                    }
                }
            }

            if !cell.contains(Cell::EAST) {
                for wall_offset in 0..wall_size {
                    for cell_offset in 0..=cell_size {
                        let x_temp = x + cell_size + wall_offset;
                        let y_temp = y + cell_offset;
                        image.put_pixel(x_temp as u32, y_temp as u32, wall_pixel);
                    }
                }
            }
        }

        image
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = format!("+{}\n", "---+".to_string().repeat(self.width));

        let mut top = "|".to_string();
        let mut bottom = "+".to_string();

        for (i, cell) in self.cells.iter().enumerate() {
            top.push_str("   ");
            let east_boundary = if cell.contains(Cell::EAST) { " " } else { "|" };
            top.push_str(east_boundary);

            let south_boundary = if cell.contains(Cell::SOUTH) {
                "   "
            } else {
                "---"
            };

            bottom.push_str(south_boundary);
            bottom.push_str("+");

            if (i + 1) % self.width == 0 {
                output.push_str(&top);
                output.push_str("\n");
                output.push_str(&bottom);
                output.push_str("\n");

                top = "|".to_string();
                bottom = "+".to_string();
            }
        }

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // A perfect maze has 2n - 2 edges
    fn maze_is_perfect(grid: &Grid) -> bool {
        let mut edges = 0;
        for cell in grid.cells.iter() {
            if cell.contains(Cell::NORTH) {
                edges += 1;
            }

            if cell.contains(Cell::SOUTH) {
                edges += 1;
            }

            if cell.contains(Cell::EAST) {
                edges += 1;
            }

            if cell.contains(Cell::WEST) {
                edges += 1;
            }
        }

        (2 * grid.height * grid.width - 2) == edges
    }

    #[test]
    fn test_binary_tree() {
        let width = 5_usize;
        let height = 5_usize;
        let grid = Grid::binary_tree(height, width, None);

        assert!(maze_is_perfect(&grid));
    }

    #[test]
    fn test_sidewinder() {
        let width = 5_usize;
        let height = 5_usize;
        let grid = Grid::sidewinder(height, width, None);

        assert!(maze_is_perfect(&grid));
    }
}
