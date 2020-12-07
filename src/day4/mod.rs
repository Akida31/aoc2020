use std::collections::HashMap;

pub fn solve() {
    let input = include_str!("input.txt");
    let inputs = input
        .split("\n\n")
        .map(|x| x.replace("\n", " "))
        .collect::<Vec<_>>();
    println!("Day 4, Solution 1: {}", solve1(inputs.clone()));
    println!("Day 4, Solution 2: {}", solve2(inputs));
}

const FIELDS: [&str; 7] = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];

fn solve1(input: Vec<String>) -> usize {
    input
        .iter()
        .filter(|x| {
            for field in &FIELDS {
                if !x.contains(field) {
                    return false;
                }
            }
            true
        })
        .count()
}

fn solve2(input: Vec<String>) -> usize {
    #[allow(clippy::type_complexity)]
    let validators: [(&str, Box<dyn Fn(&str) -> bool>); 7] = [
        ("byr", Box::new(byr)),
        ("iyr", Box::new(iyr)),
        ("eyr", Box::new(eyr)),
        ("hgt", Box::new(hgt)),
        ("hcl", Box::new(hcl)),
        ("ecl", Box::new(ecl)),
        ("pid", Box::new(pid)),
    ];
    input
        .iter()
        .filter(|x| {
            let mut map = HashMap::new();
            for s in x.split(' ') {
                let w = s.split(':').collect::<Vec<_>>();
                map.insert(w[0], w[1]);
            }
            for (name, func) in &validators {
                if !func(map.get(name).unwrap_or(&"")) {
                    return false;
                }
            }
            true
        })
        .count()
}

fn hcl(input: &str) -> bool {
    if input.len() != 7 {
        return false;
    }
    let mut chars = input.chars();
    if chars.next().unwrap_or('a') != '#' {
        return false;
    }
    for c in chars {
        if !c.is_ascii_hexdigit() {
            return false;
        }
    }
    true
}

fn pid(input: &str) -> bool {
    input.len() == 9 && input.chars().all(|x| x.is_numeric())
}

fn check_year(input: &str, lower: usize, upper: usize) -> bool {
    let year = input.parse().unwrap_or(0);
    lower <= year && year <= upper
}

fn eyr(input: &str) -> bool {
    check_year(input, 2020, 2030)
}

fn ecl(input: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&input)
}

fn byr(input: &str) -> bool {
    check_year(input, 1920, 2002)
}

fn iyr(input: &str) -> bool {
    check_year(input, 2010, 2020)
}

fn hgt(input: &str) -> bool {
    if input.ends_with("cm") {
        let h = input.to_string()[..input.len() - 2].parse().unwrap_or(0);
        (150..=193).contains(&h)
    } else if input.ends_with("in") {
        let h = input.to_string()[..input.len() - 2].parse().unwrap_or(0);
        (59..=76).contains(&h)
    } else {
        false
    }
}
