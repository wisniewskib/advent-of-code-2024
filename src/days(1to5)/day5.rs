use std::collections::{HashMap, HashSet, VecDeque};

pub fn run() {
    let input = include_str!("../../inputs/day5.txt");
    solve(input, false);
    solve(input, true);
}

fn solve(input: &str, is_part_two: bool) {
    let mut parts = input.split("\n\n");
    let rules = parts.next().unwrap_or("");
    let updates = parts.next().unwrap_or("");

    let mut rulesMap: HashMap<i32, Vec<i32>> = HashMap::new();
    for line in rules.lines() {
        let input_parts: Vec<i32> = line.split("|").map(|x| x.parse::<i32>().unwrap()).collect();
        let left = input_parts[0];
        let right = input_parts[1];
        rulesMap.entry(left).or_insert_with(Vec::new).push(right);
    }

    let mut result_part1 = 0;
    let mut incorrect_updates = vec![];

    for update in updates.lines() {
        let mut is_valid = true;
        let mut numbers_before: HashSet<i32> = HashSet::new();
        let numbers_in_update: Vec<i32> = update
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let length = numbers_in_update.len();

        for &update in &numbers_in_update {
            if let Some(numbers_to_check) = rulesMap.get(&update) {
                if numbers_to_check
                    .iter()
                    .any(|number_to_go_before| numbers_before.contains(number_to_go_before))
                {
                    is_valid = false;
                }
            }
            numbers_before.insert(update);
        }

        if is_valid {
            result_part1 += numbers_in_update[length / 2];
        } else {
            incorrect_updates.push(numbers_in_update);
        }
    }

    println!("Part1: {}", result_part1);

    if is_part_two {
        let mut result_part2 = 0;

        for mut update in incorrect_updates {
            update = reorder_update(&update, &rulesMap);

            result_part2 += update[update.len() / 2];
        }

        println!("Part2: {}", result_part2);
    }
}

fn reorder_update(update: &[i32], rulesMap: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut in_degree = HashMap::new();
    let mut graph = HashMap::new();

    for &page in update {
        in_degree.insert(page, 0);
        graph.insert(page, vec![]);
    }

    for &page in update {
        if let Some(dependents) = rulesMap.get(&page) {
            for &dependent in dependents {
                if update.contains(&dependent) {
                    graph.get_mut(&page).unwrap().push(dependent);
                    *in_degree.entry(dependent).or_insert(0) += 1;
                }
            }
        }
    }

    let mut queue: VecDeque<i32> = VecDeque::new();
    let mut sorted = vec![];

    for (&page, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(page);
        }
    }

    while let Some(page) = queue.pop_front() {
        sorted.push(page);
        if let Some(dependents) = graph.get(&page) {
            for &dependent in dependents {
                let entry = in_degree.entry(dependent).or_insert(0);
                *entry -= 1;
                if *entry == 0 {
                    queue.push_back(dependent);
                }
            }
        }
    }

    sorted
}
