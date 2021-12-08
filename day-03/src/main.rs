use structopt::StructOpt;

/// Rust solver for https://adventofcode.com/2021/day/3
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

enum Instrument {
    O2Meter,
    CO2Meter,
}

fn main() {
    let args = Cli::from_args();
    let input = std::fs::read_to_string(&args.path).expect("could not read file");
    let content = input.lines().collect::<Vec<&str>>();
    // gamma = left-to-right most common bits
    let gamma = parse_gamma(&content);
    // epsilon = "invert" gamma -> 10110 becomes 01001
    let epsilon = invert_binary_string(&gamma);
    let mut result = calc_result(&gamma, &epsilon);

    println!("{0: <10} {1: <10}", "[Part I]", result);

    let o2 = get_meter_reading(&content, Instrument::O2Meter);
    let co2 = get_meter_reading(&content, Instrument::CO2Meter);

    result = calc_result(o2, co2);

    println!("{0: <10} {1: <10}", "[Part II]", result);
}

struct MeterReadings<'b> {
    values: Vec<&'b str>,
    index: usize,
}

fn get_meter_reading<'b>(content: &Vec<&'b str>, instrument: Instrument) -> &'b str {
    let mut readings = MeterReadings {
        values: content.to_vec(),
        index: 0,
    };
    while readings.values.len() > 1 {
        readings.values = filter_values(readings.values, readings.index, &instrument);
        readings.index = readings.index + 1
    }

    readings.values[0]
}

fn calc_result(gamma: &str, epsilon: &str) -> isize {
    let a = isize::from_str_radix(gamma, 2).unwrap();
    let b = isize::from_str_radix(epsilon, 2).unwrap();
    a * b
}

fn parse_gamma<'a>(content: &Vec<&'a str>) -> String {
    let mut position_holder: Vec<Vec<char>> = Vec::new();
    // most common bit
    let mut mcb = Vec::new();
    for line in content.iter() {
        // position_holder contains nested vector for each position, push new item if exists
        for (index, character) in line.char_indices() {
            if position_holder.get(index).is_some() {
                position_holder.get_mut(index).unwrap().push(character);
            } else {
                position_holder.insert(index, Vec::new());
                position_holder.get_mut(index).unwrap().push(character);
            }
        }
    }

    for position in position_holder.iter() {
        let mut count1: i32 = 0;
        let mut count0: i32 = 0;
        for entry in position.iter() {
            let val: bool = entry.eq_ignore_ascii_case(&char::from_digit(1, 2).unwrap());
            if val {
                count1 = count1 + 1;
            } else {
                count0 = count0 + 1;
            }
        }
        mcb.push(if count1 > count0 { "1" } else { "0" });
    }

    let result = mcb.join("").to_string();
    result
}

fn filter_values<'b>(content: Vec<&'b str>, index: usize, instrument: &Instrument) -> Vec<&'b str> {
    let digit = match instrument {
        Instrument::O2Meter => 1,
        Instrument::CO2Meter => 0,
    };

    let total_length = content.len();
    let digit_length = content
        .to_vec()
        .into_iter()
        .filter(|s| {
            s.chars()
                .nth(index)
                .unwrap()
                .eq_ignore_ascii_case(&char::from_digit(digit, 2).unwrap())
        })
        .count();

    let which: usize = match instrument {
        Instrument::O2Meter => {
            if digit_length >= (total_length - digit_length) {
                1
            } else {
                0
            }
        }
        Instrument::CO2Meter => {
            if digit_length <= (total_length - digit_length) {
                0
            } else {
                1
            }
        }
    };

    content
        .into_iter()
        .filter(|s| {
            s.chars()
                .nth(index)
                .unwrap()
                .eq_ignore_ascii_case(&char::from_digit(which.try_into().unwrap(), 2).unwrap())
        })
        .collect()
}

fn invert_binary_string(input: &String) -> String {
    let mut result: String = "".to_string();

    for c in input.chars() {
        if c.eq_ignore_ascii_case(&char::from_digit(1, 2).unwrap()) {
            result = result + "0";
        } else if c.eq_ignore_ascii_case(&char::from_digit(0, 2).unwrap()) {
            result = result + "1";
        }
    }
    result
}

fn invert_binary_string_alternative(input: &String) -> String {
    let num = !u8::from_str_radix(input, 2).unwrap();
    format!("{:b}", num)[3..].to_string()
}

fn invert_binary_string_another(input: &String) -> String {
    String::from_utf8(
        input
            .to_owned()
            .into_bytes()
            .iter()
            .map(|d| d ^ 1)
            .collect(),
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inverts_mcb_lcb() {
        assert_eq!(invert_binary_string(&"10110".to_string()), "01001");
        assert_eq!(
            invert_binary_string_alternative(&"10110".to_string()),
            "01001"
        );
        assert_eq!(invert_binary_string_another(&"10110".to_string()), "01001");
    }

    #[test]
    fn processes_input_correctly() {
        let test_input = std::fs::read_to_string("src/test.txt").unwrap();
        let vec = test_input.lines().collect::<Vec<&str>>();
        let gamma = parse_gamma(&vec);
        assert_eq!(gamma, "10110");
        assert_eq!(isize::from_str_radix(&gamma, 2).unwrap(), 22);
    }

    #[test]
    fn calculates_result() {
        let check = 198;
        let gamma = "10110".to_string();
        let epsilon = "01001".to_string();
        assert_eq!(calc_result(&gamma, &epsilon), check);
    }
}
