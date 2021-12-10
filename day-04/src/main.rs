use structopt::StructOpt;

/// Rust solver for https://adventofcode.com/2021/day/4
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let input = std::fs::read_to_string(&args.path).expect("could not read file");

    let lines = input.lines().collect::<Vec<&str>>();

    // separate: first line is drawings, rest is bingo tables
    let drawn_numbers: Vec<i8> = lines[0]
        .split(',')
        .map(|s| s.parse::<i8>().unwrap())
        .collect();

    // multi-dimensional vector of rows, bingo table is 5x5
    let mut tables: Vec<Vec<i8>> = Vec::new();
    let mut table_index = 0;
    for (filtered_line_index, line) in lines[1..].iter().filter(|l| !l.is_empty()).enumerate() {
        let _temp = line
            .chars()
            .enumerate()
            .filter(|(i, _)| (i + 1) % 3 != 0)
            .map(|(_, e)| e)
            .collect::<Vec<char>>();

        // parse digits in line from a pair of chars, put in table
        let mut digits_in_line: Vec<i8> = _temp
            .chunks(2)
            .map(|chars| {
                format!("{}{}", chars[0], chars[1])
                    .trim()
                    .parse::<i8>()
                    .unwrap()
            })
            .collect();

        if tables.len() < (table_index + 1) {
            tables.push(Vec::new());
        }

        tables[table_index].append(&mut digits_in_line);

        if (filtered_line_index + 1) % 5 == 0 {
            table_index = table_index + 1;
        }
    }

    let mut table_marks: Vec<Vec<bool>> = Vec::new();
    let mut winner: Option<&Vec<i8>> = None;
    let mut winner_index: Option<usize> = None;
    let mut last_drawn: Option<&i8> = None;

    // in each draw, mark hits in each buffer. check if either full row or full column done, if yes, mark as winning
    for number in &drawn_numbers {
        for (table_index, table) in &mut tables.iter().enumerate() {
            if table_marks.len() < (table_index + 1) {
                table_marks.push(vec![false; 25]);
            }

            for (digit_index, digit) in table.iter().enumerate() {
                if &digit == &number {
                    match winner {
                        None => {
                            table_marks[table_index][digit_index] = true;

                            if check_if_a_table_won(&table_marks[table_index]) {
                                last_drawn = Some(digit);
                                winner = Some(table);
                                winner_index = Some(table_index);

                                break;
                            }
                        }
                        Some(_) => {
                            break;
                        }
                    }
                }
            }
        }
    }

    // sum all unmarked numbers in winning table. multiply this sum by the last number drawn
    let result = winner
        .unwrap()
        .iter()
        .enumerate()
        .map(|(i, n)| {
            if table_marks[winner_index.unwrap()][i] {
                0
            } else {
                i32::from(*n)
            }
        })
        .sum::<i32>()
        * i32::from(*last_drawn.unwrap());

    println!("{0: <10} {1: <10}", "[Part I]", result);
}

fn check_if_a_table_won(marks: &Vec<bool>) -> bool {
    let mut did_it_win = false;

    for horizontal in marks.chunks(5) {
        if horizontal.iter().all(|b| *b) {
            did_it_win = true
        }
    }

    // TODO: test!
    for column_n in 1..5 {
        if marks.iter().skip(column_n - 1).step_by(5).all(|b| *b) {
            did_it_win = true
        }
    }

    did_it_win
}
