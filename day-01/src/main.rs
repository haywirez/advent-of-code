use structopt::StructOpt;

/// Rust solver for https://adventofcode.com/2021/day/1
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let mut previous: u32 = 0;
    let mut init: bool = false;
    let mut count: u32 = 0;

    // Part I
    // Single value

    for line in content.lines() {
        let current: u32 = line.parse().unwrap();
        let increased: bool = current > previous && init;
        if increased {
            count = count + 1;
        }
        previous = current;
        init = true;
    }
    println!("{0: <10} {1: <10}", "[Part I]", count);

    // Part II
    // Sliding window

    previous = 0;
    count = 0;
    init = false;
    let mut before_previous: u32 = 0;
    let mut previous_window: u32 = 0;
    let mut buffer = 0;
    for line in content.lines() {
        let current: u32 = line.parse().unwrap();
        let current_window = current + previous + before_previous;
        let increased: bool = current_window > previous_window && init;
        if increased && (buffer == 3 || !init) {
            count = count + 1;
        }
        before_previous = previous;
        previous = current;
        previous_window = current_window;
        init = true;
        if buffer < 3 {
            buffer = buffer + 1;
        }
    }
    println!("{0: <10} {1: <10}", "[Part II]", count);
}
