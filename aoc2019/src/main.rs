use aoc2019::{one, two};

fn main() {
    println!("Day 1: part1 = {}, part2 = {}", one::part1(one::input("input/one.txt")), one::part2(one::input("input/one.txt")));
    println!("Day 2: part1 = {}, part2 = {}", two::part1(two::input("input/two.txt"), 12, 2), two::part2(two::input("input/two.txt")));
}
