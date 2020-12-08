use std::str::Lines;

fn main() {
    let input = include_str!("input.txt").lines();
    println!("Day 5, Solution 1: {}", solve1(input.clone()));
    println!("Day 5, Solution 2: {}", solve2(input));
}

fn solve1(input: Lines) -> usize {
    input.map(get_id).max().unwrap()
}

fn solve2(input: Lines) -> usize {
    let mut ids = input.map(get_id).collect::<Vec<usize>>();
    ids.sort_unstable();
    for id in ids.windows(2) {
        if let [x, y] = id {
            if y - x == 2 && x > &7 && x < &(127 * 8) && y % 8 != 0 {
                return *x + 1;
            }
        }
    }
    unreachable!()
}

fn get_id(enc: &str) -> usize {
    let mut row_min = 0;
    let mut row_max = 127;
    let mut column_min = 0;
    let mut column_max = 8;
    for c in enc.chars() {
        match c {
            'F' => row_max = (row_min + row_max + 1) / 2 - 1,
            'B' => row_min = (row_min + row_max + 1) / 2,
            'L' => column_max = (column_min + column_max + 1) / 2 - 1,
            'R' => column_min = (column_min + column_max + 1) / 2,
            _ => {}
        }
    }
    row_max * 8 + column_max
}
