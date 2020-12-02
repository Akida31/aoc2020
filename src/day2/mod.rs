pub fn solve() {
    let input = include_str!("input.txt");
    println!("Day 2, Solution 1: {}", solve1(input));
    println!("Day 2, Solution 2: {}", solve2(input));
}
fn solve1(input: &str) -> usize {
        regex::Regex::new(r"(\d+)-(\d+) ([a-zA-Z]): ([a-zA-Z]+)").unwrap().captures_iter(input)
        .filter(|x| {
            let lower: usize = x[1].parse().expect("cant parse");
            let higher: usize = x[2].parse().expect("cant parse");
            let c = &x[3];
            let password = &x[4];
            let ms = &password.chars().filter(|p| p.to_string()==c).count();
            &lower <= ms && ms <= &higher
        }).count()
}

fn solve2(input: &str) -> usize {
    regex::Regex::new(r"(\d+)-(\d+) ([a-zA-Z]): ([a-zA-Z]+)").unwrap().captures_iter(input)
        .filter(|x| {
            let lower: usize = x[1].parse().expect("cant parse");
            let higher: usize = x[2].parse().expect("cant parse");
            let c = &x[3];
            let password = &x[4];
            (password.chars().nth(lower-1).unwrap().to_string()==c) ^ (password.chars().nth(higher-1).unwrap().to_string()==c)
        }).count()
}
