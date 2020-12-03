pub fn solve() {
    let input = crate::rf::read_to("src/day3/input.txt");
    println!("Day 3, Solution 1: {}", solve1(input.clone()));
    println!("Day 3, Solution 2: {}", solve2(input));
}
fn solve1(input: Vec<String>) -> usize {
    input.iter().enumerate().fold(0, |p, (i, x)| {
        if x.chars().nth((i * 3) % x.len()).unwrap() == '#' {
            p + 1
        } else {
            p
        }
    })
}

fn solve2(input: Vec<String>) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(r, d)| solve_complete(input.clone(), *r, *d))
        .product()
}

fn solve_complete(input: Vec<String>, right: usize, down: usize) -> usize {
    input.iter().enumerate().fold(0, |p, (i, x)| {
        if i % down != 0 {
            return p;
        }
        if x.chars().nth((i * right / down) % x.len()).unwrap() == '#' {
            p + 1
        } else {
            p
        }
    })
}
