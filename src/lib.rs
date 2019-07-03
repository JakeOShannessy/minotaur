#[macro_use]
extern crate bitflags;

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
    struct Cell: u8 {
        const NORTH = 0b00000001;
        const SOUTH = 0b00000010;
        const EAST =  0b00000100;
        const WEST =  0b00001000;
    }
}

/*
Grid represents a maze.
*/
pub struct Grid {
    cells: Vec<Cell>,
    perfect: bool,
}

impl Grid {

    pub fn binary_tree(height: usize, width: usize) -> Grid {
        let cells = vec![Cell::default(); height * width];
        let perfect = true;

        Grid {
            cells,
            perfect,
        }
    }

}

pub fn hello() {
    println!("hello");
}