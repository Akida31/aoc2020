use std::collections::HashSet;

pub fn solve() {
    let input = include_str!("input.txt").split("\n\n");
    let inputs = input
        .clone()
        .map(|x| x.replace("\n", ""))
        .collect::<Vec<_>>();
    println!("Day 6, Solution 1: {}", solve1(inputs));
    println!("Day 6, Solution 2: {}", solve2(input.collect()));
}

fn solve1(input: Vec<String>) -> usize {
    input
        .iter()
        .map(|x| x.chars().collect::<HashSet<char>>().len())
        .sum()
}

fn solve2(input: Vec<&str>) -> usize {
    input
        .iter()
        .map(|x| {
            let mut lines = x.split('\n');
            let mut s: HashSet<char> = lines.next().unwrap().chars().collect();
            for line in lines {
                for c in s.clone().iter() {
                    if !line.contains(&c.to_string()) {
                        s.remove(&c);
                    }
                }
            }
            s.len()
        })
        .sum()
}
