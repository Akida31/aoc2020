fn main() {
    let input: Vec<_> = include_str!("input.txt")
        .split('\n')
        .map(|x| {
            let mut split = x.split(' ');
            let ins = split.next().unwrap();
            let val = split.next().unwrap().parse::<isize>().unwrap();
            Some((ins, val))
        })
        .collect();

    println!("Day 7, Solution 1: {}", solve1(input.clone()));
    println!("Day 7, Solution 2: {}", solve2(input));
}

fn solve1(input: Vec<Option<(&str, isize)>>) -> isize {
    run_program(input).unwrap_err()
}

fn solve2(input: Vec<Option<(&str, isize)>>) -> isize {
    for (i, ins) in input.clone().iter().enumerate() {
        let ins = match ins.unwrap().0 {
            "jmp" => "nop",
            "nop" => "jmp",
            _ => continue,
        };
        let mut input = input.clone();
        input[i] = Some((ins, input[i].unwrap().1));
        if let Ok(acc) = run_program(input) {
            return acc;
        }
    }
    unreachable!()
}

fn run_program(mut input: Vec<Option<(&str, isize)>>) -> Result<isize, isize> {
    let mut acc = 0;
    let mut ptr = 0;
    while let Some((ins, val)) = input[ptr as usize] {
        input[ptr as usize] = None;
        match ins {
            "acc" => {
                acc += val;
                ptr += 1
            }
            "jmp" => ptr += val,
            _ => ptr += 1,
        }
        if ptr == input.len() as isize {
            return Ok(acc);
        }
    }
    Err(acc)
}
