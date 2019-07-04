#[macro_use]
extern crate bitflags;
extern crate image;

use image::{GrayImage, ImageBuffer};
use rand::{thread_rng, Rng};


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
    #[derive(Default)]
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
#[derive(Debug)]
pub struct Grid {
    cells: Vec<Cell>,
    perfect: bool,
    width: usize,
    height: usize,
}

impl Grid {
    // Must be at least 1x1
    pub fn binary_tree(height: usize, width: usize) -> Grid {
        let mut cells = vec![Cell::default(); height * width];

        // For all cells in the northernmost row, there are no
        // northern neighbors. So link with eastern neighbor,
        // except the corner, which has neither a northern nor
        // eastern neighbor.

        for i in 0..cells.len() {
            let east_edge = (i + 1) % width == 0;
            let north_edge = i < width;
            let choose_north = rand::random();

            if !north_edge && (east_edge || choose_north) {
                cells[i] |= Cell::NORTH;
                cells[i - width] |= Cell::SOUTH;
            } else if !east_edge {
                cells[i] |= Cell::EAST;
                cells[i + 1] |= Cell::WEST;
            }
        }

        let perfect = true;

        Grid {
            cells,
            perfect,
            width,
            height,
        }
    }

    pub fn sidewinder(height: usize, width: usize) -> Grid {
        let mut cells = vec![Cell::default(); height * width];

        let mut run_start = width;
        let mut rng = thread_rng();
        for i in 0..cells.len() {
            let east_edge = (i + 1) % width == 0;
            let north_edge = i < width;
            let choose_north = rand::random();

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

        let perfect = true;

        Grid {
            cells,
            perfect,
            width,
            height,
        }
    }

    pub fn to_image(&self) -> GrayImage {
        let cell_size = 10;
        let image_width = cell_size * self.width + 1;
        let image_height = cell_size * self.height + 1;

        let background_pixel = image::Luma([255u8]);
        let wall_pixel = image::Luma([0u8]);

        let mut image =
            ImageBuffer::from_pixel(image_width as u32, image_height as u32, background_pixel);

        for (cell_index, cell) in self.cells.iter().enumerate() {
            let x = (cell_index % self.width) * cell_size;
            let y = (cell_index / self.width) * cell_size;

            if !cell.contains(Cell::NORTH) {
                for i in 0..=cell_size {
                    let x = x + i;
                    image.put_pixel(x as u32, y as u32, wall_pixel)
                }
            }

            if !cell.contains(Cell::SOUTH) {
                for i in 0..=cell_size {
                    let x = x + i;
                    let y = y + cell_size;
                    image.put_pixel(x as u32, y as u32, wall_pixel)
                }
            }

            if !cell.contains(Cell::WEST) {
                for i in 0..=cell_size {
                    let y = y + i;
                    image.put_pixel(x as u32, y as u32, wall_pixel);
                }
            }

            if !cell.contains(Cell::EAST) {
                for i in 0..=cell_size {
                    let x = x + cell_size;
                    let y = y + i;
                    image.put_pixel(x as u32, y as u32, wall_pixel);
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
