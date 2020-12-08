from sys import argv
from os import mkdir


def edit_cargo(day):
    with open("Cargo.toml", "a") as f:
        f.write(f"""[[bin]]
name = "day{day}"
path = "src/day{day}/mod.rs"
""")


def create_template(day):
    mkdir(f"src/day{day}")
    open(f"src/day{day}/input.txt", "a").close()
    with open(f"src/day{day}/mod.rs", "w") as f:
        f.write("""fn main() {
    let input = include_str!("input.txt");
    println!("Day @day@, Solution 1: {}", solve1(input.clone()));
    println!("Day @day@, Solution 2: {}", solve2(input));
}

fn solve1(input: &str) -> usize {
    unimplemented!()
}

fn solve2(input: &str) -> usize {
    unimplemented!()
}""".replace("@day@", str(day)))

def get_day():
    if len(argv) < 2:
        day = input("Day: ")
    else:
        day = argv[1]
    try:
        day = int(day)
    except ValueError:
        print("not a valid day [1..25]")
        day = None
    while not day:
        try:
            day = int(input("Day: "))
        except ValueError:
            print("not a valid day [1..25]")
            day = None
    return day

if __name__ == "__main__":
    day = get_day()
    edit_cargo(day)
    create_template(day)