pub fn run() {
    let input = include_str!("../../inputs/day2.txt");
    solve(input, false);
    solve(input, true);
}

fn is_valid_line(numbers: &[i32], allow_one_violation: bool) -> bool {
    let mut increasing_violations = 0;
    let mut decreasing_violations = 0;

    for pair in numbers.windows(2) {
        if (pair[1] - pair[0]).abs() > 3 {
            return false;
        }

        if !(pair[1] > pair[0]) {
            increasing_violations += 1;
        }
        if !(pair[1] < pair[0]) {
            decreasing_violations += 1;
        }
    }

    if allow_one_violation {
        increasing_violations <= 1 || decreasing_violations <= 1
    } else {
        increasing_violations == 0 || decreasing_violations == 0
    }
}

fn is_safe_with_dampener(numbers: &[i32]) -> bool {
    if is_valid_line(numbers, false) {
        return true;
    }
    for i in 0..numbers.len() {
        let mut modified_numbers = numbers.to_vec();
        modified_numbers.remove(i);
        if is_valid_line(&modified_numbers, false) {
            return true;
        }
    }
    false
}

fn solve(input: &str, allow_one_violation: bool) {
    let safe_lines: Vec<&str> = input
        .lines()
        .filter(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();

            if allow_one_violation {
                is_safe_with_dampener(&numbers)
            } else {
                is_valid_line(&numbers, false)
            }
        })
        .collect();

    println!(
        "Solution (allow one violation: {}): {}",
        allow_one_violation,
        safe_lines.len()
    );
}
