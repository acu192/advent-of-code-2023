use std::fs::read_to_string;

struct Iter {
    red: i32,
    green: i32,
    blue: i32,
}

struct Game {
    id: i64,
    iters: Vec<Iter>,
}

fn parse_iter(s: &&str) -> Iter {
    let mut i = Iter {
        red: 0,
        green: 0,
        blue: 0,
    };
    let counts = s.split(", ").collect::<Vec<_>>();
    for c in counts {
        let x = c.split(" ").collect::<Vec<_>>();
        let n: i32 = x[0].parse().unwrap();
        match x[1] {
            "red" => i.red = n,
            "green" => i.green = n,
            "blue" => i.blue = n,
            _ => panic!(),
        }
    }
    return i;
}

fn parse_game(s: &str) -> Game {
    let mut g = Game {
        id: 0,
        iters: vec!(),
    };
    let x = s.split(": ").collect::<Vec<_>>();
    let a = x[0];
    let b = x[1];
    let x = a.split(" ").collect::<Vec<_>>();
    g.id = x[1].parse().unwrap();
    g.iters = b.split("; ").collect::<Vec<_>>()
        .iter()
        .map(parse_iter)
        .collect::<Vec<_>>();
    return g;
}

fn possible(g: &Game) -> bool {
    let max_red = g.iters.iter().map(|i| i.red).max().unwrap();
    let max_green = g.iters.iter().map(|i| i.green).max().unwrap();
    let max_blue = g.iters.iter().map(|i| i.blue).max().unwrap();
    return max_red <= 12 && max_green <= 13 && max_blue <= 14;
}

pub fn solve_part1(filename: &str) -> i64 {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(parse_game)
        .filter(possible)
        .map(|g| g.id)
        .sum()
}

fn power(g: Game) -> i64 {
    let max_red = g.iters.iter().map(|i| i.red).max().unwrap();
    let max_green = g.iters.iter().map(|i| i.green).max().unwrap();
    let max_blue = g.iters.iter().map(|i| i.blue).max().unwrap();
    return (max_red * max_green * max_blue).into();
}

pub fn solve_part2(filename: &str) -> i64 {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(parse_game)
        .map(power)
        .sum()
}

fn main() {
    println!("{}", solve_part1("in1.txt"));
    println!("{}", solve_part2("in1.txt"));
}
