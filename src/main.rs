use helpers::read_file;

mod helpers;

mod d1;

fn main() {
    println!("Advent of Code 2024!");
    // Day 01
    assert_eq!(2344935, d1::part1(read_file(1)));
    assert_eq!(27647262, d1::part2(read_file(1)));
}
