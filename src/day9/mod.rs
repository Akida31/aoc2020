fn main() {
    let input: Vec<_> = include_str!("input.txt")
        .split('\n')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let res1 = solve1(input.clone());
    println!("Day 9, Solution 1: {}", res1);
    println!("Day 9, Solution 2: {}", solve2(input, res1));
}

fn solve1(input: Vec<usize>) -> usize {
    for i in input.windows(26) {
        let mut found = false;
        let e = i[i.len() - 1];
        for x in i {
            for y in i {
                if *x + *y == e && x != y {
                    found = true;
                }
            }
        }
        if !found {
            return e;
        }
    }
    unreachable!()
}

fn solve2(input: Vec<usize>, number: usize) -> usize {
    for (lower, _) in input.iter().enumerate() {
        let mut len = 2;
        let calc = |len| {
            input
                .iter()
                .skip(lower)
                .collect::<Vec<_>>()
                .windows(len)
                .next()
                .unwrap()
                .iter()
                .map(|x| **x)
                .sum()
        };
        let mut sum: usize = calc(len);
        while sum < number {
            len += 1;
            sum = calc(len);
        }
        if sum == number {
            let sl = input.clone().into_iter().skip(lower).collect::<Vec<_>>();
            return sl.windows(len).next().unwrap().iter().max().unwrap()
                + sl.windows(len).next().unwrap().iter().min().unwrap();
        }
    }
    unreachable!()
}
