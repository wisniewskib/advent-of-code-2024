use regex::Regex;

pub fn run() {
    let input = include_str!("../../inputs/day3.txt");
    solve(input);
    solve_part_two(input);
}

fn solve_part_two(input: &str) {
    let mut result = 0;
    let re = Regex::new(r"mul\((\d+)\,(\d+)\)|don\'t\(\)|do\(\)").unwrap();
    let mut enabled = true;
    for item in re.captures_iter(input) {
        if (&item[0] == "do()") {
            enabled = true;
        } else if (&item[0] == "don't()") {
            enabled = false
        } else {
            if (enabled) {
                result += (item[1].parse::<i32>().unwrap() * item[2].parse::<i32>().unwrap());
            }
        }
    }
    println!("Result part two: {}", result)
}

fn solve(input: &str) {
    let mut result = 0;
    let re = Regex::new(r"mul\((\d+)\,(\d+)\)").unwrap();
    for item in re.captures_iter(input) {
        result += (item[1].parse::<i32>().unwrap() * item[2].parse::<i32>().unwrap());
    }
    println!("Result: {}", result)
}
