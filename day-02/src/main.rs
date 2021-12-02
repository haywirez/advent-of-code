use structopt::StructOpt;

/// Rust solver for https://adventofcode.com/2021/day/2
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

struct Position {
    x: isize,
    y: isize,
    aim: isize,
}

fn main() {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let mut position = Position { x: 0, y: 0, aim: 0 };
    for line in content.lines() {
        let mut split = line.split_whitespace();
        let instruction = split.next().unwrap().to_string();
        let unit: isize = split.next().unwrap().parse().unwrap();

        if instruction.eq_ignore_ascii_case("forward") {
            position.y = position.y + unit;
        } else if instruction.eq_ignore_ascii_case("up") {
            // depth is negative
            position.x = position.x - unit;
        } else if instruction.eq_ignore_ascii_case("down") {
            position.x = position.x + unit;
        }
    }

    println!("{0: <10} {1: <10}", "[Part I]", position.x * position.y);

    // Part II

    position.x = 0;
    position.y = 0;
    for line in content.lines() {
        let mut split = line.split_whitespace();
        let instruction = split.next().unwrap().to_string();
        let unit: isize = split.next().unwrap().parse().unwrap();

        if instruction.eq_ignore_ascii_case("forward") {
            position.y = position.y + unit;
            position.x = position.x + (unit * position.aim);
        } else if instruction.eq_ignore_ascii_case("up") {
            // depth is negative
            position.aim = position.aim - unit;
        } else if instruction.eq_ignore_ascii_case("down") {
            position.aim = position.aim + unit;
        }
    }

    println!("{0: <10} {1: <10}", "[Part II]", position.x * position.y);
}
