#![deny(unsafe_code)]
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_pcg::Lcg64Xsh32;
use serde::{Deserialize, Serialize};

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};

/*
Cell represents a single square in a maze's Grid.
It stores links in the four directions.
For example, if NORTH is true, then this Cell has
an open passage to the Cell above it.
Otherwise, there is a wall between the two Cells.

It would be a logic error if this Cell had
NORTH, but its northern neighbor did not have SOUTH.
*/
bitflags::bitflags! {
    #[derive(Default, Serialize, Deserialize,PartialEq,Clone,Debug,Copy)]
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
    pub cells: Vec<Cell>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        let cells = vec![Cell::default(); height * width];
        Grid {
            cells,
            width,
            height,
        }
    }

    fn get_rng(seed: Option<u64>) -> Lcg64Xsh32 {
        match seed {
            Some(seed) => Lcg64Xsh32::seed_from_u64(seed),
            None => Lcg64Xsh32::from_entropy(),
        }
    }

    fn link_cells(&mut self, i: usize, direction: Cell) {
        match direction {
            Cell::NORTH => {
                self.cells[i] |= Cell::NORTH;
                self.cells[i - self.width] |= Cell::SOUTH;
            }

            Cell::SOUTH => {
                self.cells[i] |= Cell::SOUTH;
                self.cells[i + self.width] |= Cell::NORTH;
            }
            Cell::EAST => {
                self.cells[i] |= Cell::EAST;
                self.cells[i + 1] |= Cell::WEST;
            }
            Cell::WEST => {
                self.cells[i] |= Cell::WEST;
                self.cells[i - 1] |= Cell::EAST;
            }
            _ => panic!(),
        };
    }

    fn valid_direction(&self, i: usize, direction: Cell) -> bool {
        match direction {
            Cell::NORTH => i >= self.width,
            Cell::SOUTH => i + self.width < self.cells.len(),
            Cell::EAST => (i + 1) % self.width != 0,
            Cell::WEST => i % self.width != 0,
            _ => false,
        }
    }

    fn neighbor(&self, i: usize, direction: Cell) -> usize {
        match direction {
            Cell::NORTH => i - self.width,
            Cell::SOUTH => i + self.width,
            Cell::EAST => i + 1,
            Cell::WEST => i - 1,
            _ => panic!(),
        }
    }

    /// binary_tree populates the maze according to the following algorithm:
    /// Arbitrarily visit every cell, choosing NORTH or EAST as follows:
    ///
    /// If both NORTH and EAST are valid, choose one direction randomly
    /// If only NORTH is valid, choose it 100% of the time
    /// If only EAST is valid, choose it 100% of the time
    ///
    /// After choosing a direction, link this cell with its neighbor in that direction
    ///
    /// The only cell that will not have a valid direction to choose from is the northeastern corner.
    pub fn binary_tree(&mut self, seed: Option<u64>) {
        self.cells = vec![Cell::default(); self.height * self.width];
        let mut rng = Grid::get_rng(seed);

        for i in 0..self.cells.len() {
            let north_valid = self.valid_direction(i, Cell::NORTH);
            let east_valid = self.valid_direction(i, Cell::EAST);

            if north_valid && (!east_valid || rng.gen()) {
                self.link_cells(i, Cell::NORTH);
            } else if east_valid {
                self.link_cells(i, Cell::EAST);
            }
        }
    }

    /// sidewinder populates the maze according to the following algorithm:
    /// Start with a cell on the western column. This cell starts a local "run."
    ///
    /// Choose NORTH or EAST as follows:
    /// If both NORTH and EAST are valid, choose one direction randomly
    /// If only NORTH is valid, choose it 100% of the time
    /// If only EAST is valid, choose it 100% of the time
    ///
    /// After choosing a direction, if EAST was chosen, link this cell with its neighbor in that direction.
    /// The EASTERN neighbor is then added to the local run and a direction is chosen for it.
    /// But if NORTH was chosen, then select at random one of the cells from the local run and link
    /// it with its NORTHERN neighbor. The local run is reset. Continue from the EASTERN neighbor.
    pub fn sidewinder(&mut self, seed: Option<u64>) {
        self.cells = vec![Cell::default(); self.height * self.width];
        let mut rng = Grid::get_rng(seed);

        // We start on the Western cell on the second row - this is the first cell that can
        // be a valid "NORTH"
        let mut run_start = self.width;

        for i in 0..self.cells.len() {
            let north_valid = self.valid_direction(i, Cell::NORTH);
            let east_valid = self.valid_direction(i, Cell::EAST);

            if north_valid && (!east_valid || rng.gen()) {
                let chosen_cell = rng.gen_range(run_start..(i + 1));
                self.link_cells(chosen_cell, Cell::NORTH);
                // Run resets
                run_start = i + 1;
            } else if east_valid {
                self.link_cells(i, Cell::EAST);
            } else {
                run_start = i + 1;
            }
        }
    }

    /// aldous_broder populates a maze in an unbiased way.
    /// Basically, first, a cell is chosen at random and considered "visited."
    /// Travel in a random direction. If the next cell is "unvisited", then
    /// link the two cells. Continue until all cells have been visited.
    pub fn aldous_broder(&mut self, seed: Option<u64>) {
        self.cells = vec![Cell::default(); self.height * self.width];
        let mut rng = Grid::get_rng(seed);
        const DIRECTIONS: [Cell; 4] = [Cell::NORTH, Cell::SOUTH, Cell::EAST, Cell::WEST];

        // Keep track of all visited cells.
        let mut visited = vec![false; self.cells.len()];

        // Starting cell must be chosen at random.
        let mut current_cell = rng.gen_range(0..self.cells.len());
        visited[current_cell] = true;
        let mut num_visited = 1;

        while num_visited < self.cells.len() {
            // Loop until we've found a valid direction - only an issue at the maze borders
            let mut direction = Cell::default();
            while !self.valid_direction(current_cell, direction) {
                direction = *DIRECTIONS.choose(&mut rng).unwrap();
            }

            let next_cell = self.neighbor(current_cell, direction);

            // If we haven't visited yet, link the cells up.
            // Either way, keep random walking from here
            if !visited[next_cell] {
                self.link_cells(current_cell, direction);
                visited[next_cell] = true;
                num_visited += 1;
            }
            current_cell = next_cell;
        }
    }

    /// wilsons populates a maze in an unbiased way.
    /// First, some random cell is set to be "visited."
    /// Then, some other random cell is "started." From there,
    /// travel randomly until you hit a "visited" cell. Once you
    /// hit a "visited" cell, connect all the links from the "started"
    /// cell. Then start over, choosing a new "unvisited" cell.
    ///
    /// The trick is that there is a "loop removal" step. So while looking
    /// for a "visited" cell, if you loop back to a cell you've travelling through
    /// this run, then remove the loop you just made.
    pub fn wilsons(&mut self, seed: Option<u64>) {
        self.cells = vec![Cell::default(); self.height * self.width];
        let mut rng = Grid::get_rng(seed);
        const DIRECTIONS: [Cell; 4] = [Cell::NORTH, Cell::SOUTH, Cell::EAST, Cell::WEST];

        // Keep track of all unvisited cells.
        let mut unvisited = HashSet::new();
        for i in 0..self.cells.len() {
            unvisited.insert(i);
        }

        // Randomly set a single cell to be visited
        let initial: usize = rng.gen_range(0..self.cells.len());
        unvisited.remove(&initial);

        let mut unvisited_to_choose_from = unvisited.clone().into_iter().collect::<Vec<usize>>();

        while !unvisited.is_empty() {
            // Performance optimization heuristic
            if unvisited.len() * unvisited.len() < unvisited_to_choose_from.len() {
                unvisited_to_choose_from = unvisited.clone().into_iter().collect::<Vec<usize>>();
            }

            let mut path_init = *unvisited_to_choose_from[..].choose(&mut rng).unwrap();
            while !unvisited.contains(&path_init) {
                path_init = *unvisited_to_choose_from[..].choose(&mut rng).unwrap();
            }

            let mut current_cell = path_init;
            let mut path = HashMap::new();

            // Loop until we have finally reached a cell that's already visited.
            while unvisited.contains(&current_cell) {
                // Loop until we've found a valid direction - only an issue at the maze borders
                let mut direction = Cell::default();
                while !self.valid_direction(current_cell, direction) {
                    direction = *DIRECTIONS.choose(&mut rng).unwrap();
                }
                path.insert(current_cell, direction);
                current_cell = self.neighbor(current_cell, direction);
            }

            current_cell = path_init;
            while unvisited.contains(&current_cell) {
                let direction = *path.get(&current_cell).unwrap();
                unvisited.remove(&current_cell);
                self.link_cells(current_cell, direction);
                current_cell = self.neighbor(current_cell, direction);
            }
        }
    }

    pub fn hunt_and_kill(&mut self, seed: Option<u64>) {
        self.cells = vec![Cell::default(); self.height * self.width];
        let mut rng = Grid::get_rng(seed);
        const DIRECTIONS: [Cell; 4] = [Cell::NORTH, Cell::SOUTH, Cell::EAST, Cell::WEST];

        // Keep track of all visited cells.
        let mut visited_cells = HashSet::new();

        // Randomly set a single cell to be visited
        let mut current_cell: usize = rng.gen_range(0..self.cells.len());
        visited_cells.insert(current_cell);
        // Optimization: maintain frontier of possible cells that are
        // potentially adjacent to a visited cell
        let mut frontier = BinaryHeap::new();
        frontier.push(Reverse(current_cell));

        while !frontier.is_empty() {
            // Loop until we boxed ourselves in with visited cells
            loop {
                let mut directions = Vec::new();
                for direction in DIRECTIONS.iter() {
                    if self.valid_direction(current_cell, *direction) {
                        let neighbor = self.neighbor(current_cell, *direction);
                        if !visited_cells.contains(&neighbor) {
                            directions.push(*direction);
                            frontier.push(Reverse(neighbor));
                        }
                    }
                }
                if let Some(direction) = directions[..].choose(&mut rng) {
                    self.link_cells(current_cell, *direction);
                    current_cell = self.neighbor(current_cell, *direction);
                    visited_cells.insert(current_cell);
                } else {
                    break;
                }
            }
            // Boxed in! Time to iterate through maze and select first unvisited cell that borders
            // a visited cell
            while visited_cells.contains(&current_cell) && !frontier.is_empty() {
                current_cell = frontier.pop().unwrap().0;
            }

            if frontier.is_empty() {
                break;
            }
            // Now link it with the adjacent cell
            visited_cells.insert(current_cell);
            for direction in DIRECTIONS.iter() {
                if self.valid_direction(current_cell, *direction) {
                    let neighbor = self.neighbor(current_cell, *direction);
                    // Found the adjacent visited cell!
                    if visited_cells.contains(&neighbor) {
                        self.link_cells(current_cell, *direction);
                        break;
                    }
                }
            }
        }
    }

    pub fn recursive_backtracker(&mut self, seed: Option<u64>) {
        self.cells = vec![Cell::default(); self.height * self.width];
        let mut rng = Grid::get_rng(seed);
        const DIRECTIONS: [Cell; 4] = [Cell::NORTH, Cell::SOUTH, Cell::EAST, Cell::WEST];

        // Keep track of all visited cells.
        let mut visited_cells = HashSet::new();

        // Randomly set a single cell to be visited
        let mut current_cell: usize = rng.gen_range(0..self.cells.len());
        visited_cells.insert(current_cell);
        // Stack of visited cells
        let mut cell_stack = Vec::new();
        cell_stack.push(current_cell);

        while !cell_stack.is_empty() {
            // Loop until we boxed ourselves in with visited cells
            loop {
                let mut directions = Vec::new();
                for direction in DIRECTIONS.iter() {
                    if self.valid_direction(current_cell, *direction) {
                        let neighbor = self.neighbor(current_cell, *direction);
                        if !visited_cells.contains(&neighbor) {
                            directions.push(*direction);
                        }
                    }
                }
                if let Some(direction) = directions[..].choose(&mut rng) {
                    self.link_cells(current_cell, *direction);
                    current_cell = self.neighbor(current_cell, *direction);
                    visited_cells.insert(current_cell);
                    cell_stack.push(current_cell);
                } else {
                    break;
                }
            }
            // Boxed in! Time to pop cells off the stack and find one that has
            // an unvisited adjacent neighbor
            'outer: while let Some(next_cell) = cell_stack.pop() {
                current_cell = next_cell;
                // break if adjacent unvisited neighbor
                for direction in DIRECTIONS.iter() {
                    if self.valid_direction(current_cell, *direction) {
                        let neighbor = self.neighbor(current_cell, *direction);
                        // Found the adjacent unvisited cell!
                        if !visited_cells.contains(&neighbor) {
                            self.link_cells(current_cell, *direction);
                            current_cell = neighbor;
                            visited_cells.insert(current_cell);
                            cell_stack.push(current_cell);
                            break 'outer;
                        }
                    }
                }
            }
        }
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
            bottom.push('+');

            if (i + 1) % self.width == 0 {
                output.push_str(&top);
                output.push('\n');
                output.push_str(&bottom);
                output.push('\n');

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
    use std::collections::HashSet;

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
        if (2 * grid.height * grid.width - 2) != edges {
            println!("{}", grid);
            println!(
                "expect {:?} got {:?}",
                2 * grid.height * grid.width - 2,
                edges
            );
            println!("{:?} ", grid.cells)
        }
        (2 * grid.height * grid.width - 2) == edges
    }

    #[test]
    fn test_binary_tree() {
        let width = 50_usize;
        let height = 50_usize;
        for _i in 0..10000 {
            let mut grid = Grid::new(height, width);
            grid.binary_tree(None);

            assert!(maze_is_perfect(&grid));
        }
    }

    #[test]
    fn test_sidewinder() {
        let width = 50_usize;
        let height = 50_usize;
        for _i in 0..10000 {
            let mut grid = Grid::new(height, width);
            grid.sidewinder(None);

            assert!(maze_is_perfect(&grid));
        }
    }

    #[test]
    fn test_aldous_broder() {
        let width = 50_usize;
        let height = 50_usize;
        let mut grid = Grid::new(height, width);

        for _i in 0..1000 {
            grid.aldous_broder(None);

            assert!(maze_is_perfect(&grid));
        }
    }

    #[test]
    fn test_aldous_broder_all_mazes() {
        let width = 3_usize;
        let height = 3_usize;
        let mut grid = Grid::new(height, width);

        let mut mazes = HashSet::new();
        for _i in 0..100000 {
            grid.aldous_broder(None);
            mazes.insert(format!("{}", grid));
        }
        assert_eq!(192_usize, mazes.len());
    }

    #[test]
    fn test_wilsons() {
        let width = 50_usize;
        let height = 50_usize;
        let mut grid = Grid::new(height, width);

        for _i in 0..1000 {
            grid.wilsons(None);
            assert!(maze_is_perfect(&grid));
        }
    }

    #[test]
    fn test_wilsons_all_mazes() {
        let width = 3_usize;
        let height = 3_usize;
        let mut grid = Grid::new(height, width);

        let mut mazes = HashSet::new();
        for _i in 0..100000 {
            grid.wilsons(None);
            mazes.insert(format!("{}", grid));
        }
        assert_eq!(192_usize, mazes.len());
    }

    #[test]
    fn test_hunt_and_kill() {
        let width = 3_usize;
        let height = 3_usize;
        for _i in 0..10000 {
            let mut grid = Grid::new(height, width);
            grid.hunt_and_kill(None);

            assert!(maze_is_perfect(&grid));
        }
    }

    #[test]
    fn test_recursive_backtracker() {
        let width = 50_usize;
        let height = 50_usize;
        let mut grid = Grid::new(height, width);

        for _i in 0..100 {
            grid.recursive_backtracker(None);

            assert!(maze_is_perfect(&grid));
        }
    }
}
