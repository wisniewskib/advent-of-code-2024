pub fn run() {
    let input = include_str!("../../inputs/day4.txt");
    solve(input);
}

fn count_word_occurrences(grid: &[Vec<char>], word: &str) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let word_chars: Vec<char> = word.chars().collect();
    let word_len = word_chars.len();
    let mut count = 0;

    fn check_direction(
        grid: &[Vec<char>],
        word_chars: &[char],
        start_row: usize,
        start_col: usize,
        delta_row: isize,
        delta_col: isize,
    ) -> bool {
        let mut r = start_row as isize;
        let mut c = start_col as isize;
        for &ch in word_chars {
            if r < 0
                || r >= grid.len() as isize
                || c < 0
                || c >= grid[0].len() as isize
                || grid[r as usize][c as usize] != ch
            {
                return false;
            }
            r += delta_row;
            c += delta_col;
        }
        true
    }

    for row in 0..rows {
        for col in 0..cols {
            if col + word_len <= cols && check_direction(grid, &word_chars, row, col, 0, 1) {
                count += 1;
            }
            if col + 1 >= word_len && check_direction(grid, &word_chars, row, col, 0, -1) {
                count += 1;
            }
            if row + word_len <= rows && check_direction(grid, &word_chars, row, col, 1, 0) {
                count += 1;
            }
            if row + 1 >= word_len && check_direction(grid, &word_chars, row, col, -1, 0) {
                count += 1;
            }
            if row + word_len <= rows
                && col + word_len <= cols
                && check_direction(grid, &word_chars, row, col, 1, 1)
            {
                count += 1;
            }
            if row + word_len <= rows
                && col + 1 >= word_len
                && check_direction(grid, &word_chars, row, col, 1, -1)
            {
                count += 1;
            }
            if row + 1 >= word_len
                && col + word_len <= cols
                && check_direction(grid, &word_chars, row, col, -1, 1)
            {
                count += 1;
            }
            if row + 1 >= word_len
                && col + 1 >= word_len
                && check_direction(grid, &word_chars, row, col, -1, -1)
            {
                count += 1;
            }
        }
    }

    count
}

fn count_x_mas_patterns(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    fn is_valid_diagonal(diagonal: &[char]) -> bool {
        diagonal == ['M', 'A', 'S'] || diagonal == ['S', 'A', 'M']
    }

    fn is_x_mas(grid: &[Vec<char>], r: usize, c: usize) -> bool {
        let diag1 = vec![grid[r][c], grid[r + 1][c + 1], grid[r + 2][c + 2]];

        let diag2 = vec![grid[r][c + 2], grid[r + 1][c + 1], grid[r + 2][c]];

        is_valid_diagonal(&diag1) && is_valid_diagonal(&diag2)
    }

    for r in 0..rows - 2 {
        for c in 0..cols - 2 {
            if is_x_mas(grid, r, c) {
                count += 1;
            }
        }
    }

    count
}

fn solve(input: &str) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let word = "XMAS";
    let count = count_word_occurrences(&grid, word);
    let countX = count_x_mas_patterns(&grid);
    println!("Part1: {}", count);
    println!("Part2: {}", countX)
}
