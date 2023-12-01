use std::fs::read_to_string;

pub fn solve_part1(filename: &str) -> i64 {
    let lines = read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().filter(|c| c.is_digit(10)).collect::<String>())
        .map(|line| [line.chars().nth(0).unwrap(), line.chars().nth(line.len()-1).unwrap()])
        .map(|line| line.iter().collect::<String>())
        .map(|line| line.parse::<i64>().unwrap())
        .sum::<i64>();
    return lines;
}

pub fn solve_part2(filename: &str) -> i64 {
    let lines = read_to_string(filename)
        .unwrap()
        .lines()
        .map(extract_digits)
        .map(|line| [line.chars().nth(0).unwrap(), line.chars().nth(line.len()-1).unwrap()])
        .map(|line| line.iter().collect::<String>())
        .map(|line| line.parse::<i64>().unwrap())
        .sum::<i64>();
    return lines;
}

fn extract_digits(s: &str) -> String {
    let mut ret = String::from("");
    for (i, c) in s.chars().enumerate() {
        if c.is_digit(10) {
            ret.push(c);
        }
        else if s[i..].starts_with("one") { ret.push('1'); }
        else if s[i..].starts_with("two") { ret.push('2'); }
        else if s[i..].starts_with("three") { ret.push('3'); }
        else if s[i..].starts_with("four") { ret.push('4'); }
        else if s[i..].starts_with("five") { ret.push('5'); }
        else if s[i..].starts_with("six") { ret.push('6'); }
        else if s[i..].starts_with("seve") { ret.push('7'); }
        else if s[i..].starts_with("eight") { ret.push('8'); }
        else if s[i..].starts_with("nine") { ret.push('9'); }
    }
    ret
}

fn main() {
    // println!("{}", solve_part1("in1.txt"));
    println!("{}", solve_part2("in1.txt"));
}
