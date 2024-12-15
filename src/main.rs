use helpers::read_file;

mod helpers;

mod d1;
mod d2;

fn main() {
    println!("Advent of Code 2024!");
    // Day 01
    assert_eq!(2344935, d1::part1(read_file(1)));
    assert_eq!(27647262, d1::part2(read_file(1)));
    // Day 02
    assert_eq!(257, d2::part1(read_file(2)));
    assert_eq!(328, d2::part2(read_file(2)));
}
