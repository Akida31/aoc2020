fn main() {
    let input: Vec<_> = include_str!("test.txt")
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect();
    println!("Day 11, Solution 1: {}", solve1(input.clone()));
    println!("Day 11, Solution 2: {}", solve2(input));
}

const EMPTY: char = 'L';
const OCCUPIED: char = '#';

fn count_occupied(input: &[Vec<char>], x: usize, y: usize) -> usize {
    (0..3)
        .map(|ny| {
            (0..3)
                .filter(|nx| {
                    if ny == 1 && &ny == nx
                        || x + nx == 0
                        || y + ny == 0
                        || x + nx > input[0].len()
                        || y + ny > input.len()
                    {
                        false
                    } else {
                        input[y + ny - 1][x + nx - 1] == OCCUPIED
                    }
                })
                .count()
        })
        .sum()
}

fn solve1(mut input: Vec<Vec<char>>) -> usize {
    let mut last = input.clone();
    let input_len = input.len();
    loop {
        for y in 0..input_len {
            for x in 0..input[0].len() {
                match input[y][x] {
                    EMPTY if count_occupied(&last, x, y) == 0 => {
                        input[y][x] = OCCUPIED;
                    }
                    OCCUPIED if count_occupied(&last, x, y) >= 4 => {
                        input[y][x] = EMPTY;
                    }
                    _ => {}
                }
            }
        }
        if last == input {
            break;
        }
        last = input.clone();
    }
    input
        .iter()
        .map(|r| r.iter().filter(|x| x == &&OCCUPIED).count())
        .sum()
}

fn count_occupied2(input: &[Vec<char>], x: usize, y: usize) -> usize {
    let x = x as isize;
    let y = y as isize;
    let res = (-1..=1)
        .map(|ny| {
            (-1..=1)
                .filter(|nx| {
                    if ny == 0 && *nx == 0 {
                        return false;
                    }
                    let mut i = 1;
                    while y + ny * i > 0
                        && y + ny * i < input.len() as isize
                        && x + nx * i > 0
                        && x + nx * i < input[0].len() as isize
                    {
                        match input[(y + ny * i) as usize][(x + nx * i) as usize] {
                            OCCUPIED => return true,
                            EMPTY => return false,
                            _ => i += 1,
                        }
                    }
                    false
                })
                .count()
        })
        .sum();
    println!("found {} with x={} and y={}", res, x, y);
    res
}

fn solve2(mut input: Vec<Vec<char>>) -> usize {
    let mut last = input.clone();
    let input_len = input.len();
    loop {
        for y in 0..input_len {
            for x in 0..input[0].len() {
                match input[y][x] {
                    EMPTY if count_occupied2(&last, x, y) == 0 => {
                        input[y][x] = OCCUPIED;
                    }
                    OCCUPIED if count_occupied2(&last, x, y) >= 5 => {
                        input[y][x] = EMPTY;
                    }
                    _ => {}
                }
            }
        }
        if last == input {
            break;
        }
        last = input.clone();
        println!("{:?}", input);
    }
    input
        .iter()
        .map(|r| r.iter().filter(|x| x == &&OCCUPIED).count())
        .sum()
}
