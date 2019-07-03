fn main() {
    let grid = minotaur::Grid::binary_tree(20, 20);
    println!("{}", grid);

    let image = grid.to_image();
    image.save("test.png").unwrap();
}
