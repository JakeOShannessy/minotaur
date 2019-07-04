fn main() {
    let grid = minotaur::Grid::sidewinder(4, 4);
    println!("{}", grid);

    let image = grid.to_image();
    image.save("test.png").unwrap();
}
