pub fn run() {
    let input = crate::rf::read_to("src/day1/input.txt");
    println!("Solution: {}", solve(input));
}

fn solve(input: Vec<usize>) -> usize {
    for i in input.clone() {
        for k in input.clone() {
            for j in input.clone() {
                if i + j + k == 2020 {
                    return i * j * k;
                }
            }
        }
    }
    unreachable!("no working numbers")
}
