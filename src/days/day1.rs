use std::collections::HashMap;

pub fn run() {
    let input = include_str!("../../inputs/day1.txt");
    solve(input);
}

fn solve(input: &str) {
    let lines: Vec<&str> = input.lines().collect();

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|parts| (parts[0], parts[1]))
        .unzip();

    left.sort();
    right.sort();

    let mut partOneResult = 0;
    for i in 0..lines.len() {
        partOneResult += (left[i] - right[i]).abs()
    }

    println!("Part one answer: {}", partOneResult);

    // PART TWO

    let mut map: HashMap<i32, i32> = HashMap::new();
    for n in right {
        *map.entry(n).or_insert(0) += 1;
    }

    let mut partTwoResult = 0;
    for n in left {
        partTwoResult += n * *map.get(&n).unwrap_or(&0);
    }
    print!("Part two answer: {}", partTwoResult)
}
