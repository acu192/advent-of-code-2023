use std::fs::read_to_string;

pub fn solve_part1(filename: &str) -> i64 {
    let strgrid: Vec<String> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let grid: Vec<Vec<u8>> = strgrid.iter()
        .map(|s| s.as_bytes().to_vec())
        .collect();
    println!("{}", grid[4][2]);
    return 0;
}

fn main() {
    println!("{}", solve_part1("in1.txt"))
}
