use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let inputs = regex::Regex::new(r"([a-zA-Z ]+) bags contain ([a-zA-Z, 0-9]+).")
        .unwrap()
        .captures_iter(input)
        .map(|x| (x[1].to_string(), x[2].to_string()))
        .collect::<Vec<_>>();
    println!("Day 7, Solution 1: {}", solve1(inputs.clone()));
    println!("Day 7, Solution 2: {}", solve2(inputs));
}

const MINE: &str = "shiny gold";

fn solve1(input: Vec<(String, String)>) -> usize {
    let input = input
        .into_iter()
        .map(|x| {
            let name = x.0;
            let bags = regex::Regex::new(r"\d? ?([a-zA-Z]+) ([a-zA-Z]+) bags?")
                .unwrap()
                .captures_iter(&x.1)
                .map(|bag| format!("{} {}", &bag[1], &bag[2]))
                .collect();
            (name, bags)
        })
        .collect::<HashMap<String, Vec<String>>>();
    input
        .keys()
        .filter(|&bag| {
            let mut bags = input.get(bag).unwrap().clone();
            while !bags.contains(&MINE.to_owned()) && !bags.is_empty() {
                bags = bags
                    .iter()
                    .filter(|x| "no other" != *x)
                    .map(|x| input.get(x).unwrap().clone())
                    .flatten()
                    .collect()
            }
            bags.contains(&MINE.to_owned())
        })
        .count()
}

fn solve2(input: Vec<(String, String)>) -> usize {
    let input = input
        .into_iter()
        .map(|x| {
            let name = x.0;
            let bags = regex::Regex::new(r"(\d?) ?([a-zA-Z]+) ([a-zA-Z]+) bags?")
                .unwrap()
                .captures_iter(&x.1)
                .filter(|x| &x[0] != "no other bags")
                .map(|bag| (bag[1].parse().unwrap(), format!("{} {}", &bag[2], &bag[3])))
                .collect();
            (name, bags)
        })
        .collect::<HashMap<String, Vec<(usize, String)>>>();
    get_bags(input, MINE.to_owned()) - 1
}

fn get_bags(input: HashMap<String, Vec<(usize, String)>>, bag: String) -> usize {
    let bags = input.get(&bag).unwrap();
    if !bags.is_empty() {
        bags.iter()
            .map(|bag| bag.0 * get_bags(input.clone(), bag.1.clone()))
            .sum::<usize>()
            + 1
    } else {
        1
    }
}
