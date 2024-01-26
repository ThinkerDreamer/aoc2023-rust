use std::fmt::{self, Display};
use std::fs::read_to_string;
use std::iter::from_fn;

fn main() {
    let d1p1_result = d1p1();
    println!("{d1p1_result}");
    let d1p2_result = d1p2();
    println!("{d1p2_result}");
}

fn d1p1() -> u64 {
    let input_lines: Vec<u64> = read_to_string("./inputs/D1input.txt")
        .expect("couldn't read to string")
        .lines()
        .map(parse_string)
        .collect();
    input_lines.iter().sum::<u64>()
}

fn d1p2() -> u64 {
    let input_lines: Vec<u64> = read_to_string("./inputs/D1input.txt")
        .expect("couldn't read to string")
        .lines()
        .map(process_line)
        .collect();
    input_lines.iter().sum::<u64>()
}

#[derive(Debug)]
struct InvalidNumberError;

impl fmt::Display for InvalidNumberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid number")
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Number {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}
impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::One => write!(f, "one"),
            Number::Two => write!(f, "two"),
            Number::Three => write!(f, "three"),
            Number::Four => write!(f, "four"),
            Number::Five => write!(f, "five"),
            Number::Six => write!(f, "six"),
            Number::Seven => write!(f, "seven"),
            Number::Eight => write!(f, "eight"),
            Number::Nine => write!(f, "nine"),
        }
    }
}
impl TryFrom<&char> for Number {
    type Error = InvalidNumberError;
    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            '1' => Ok(Number::One),
            '2' => Ok(Number::Two),
            '3' => Ok(Number::Three),
            '4' => Ok(Number::Four),
            '5' => Ok(Number::Five),
            '6' => Ok(Number::Six),
            '7' => Ok(Number::Seven),
            '8' => Ok(Number::Eight),
            '9' => Ok(Number::Nine),
            _ => Err(InvalidNumberError),
        }
    }
}
impl TryFrom<u64> for Number {
    type Error = InvalidNumberError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Number::One),
            2 => Ok(Number::Two),
            3 => Ok(Number::Three),
            4 => Ok(Number::Four),
            5 => Ok(Number::Five),
            6 => Ok(Number::Six),
            7 => Ok(Number::Seven),
            8 => Ok(Number::Eight),
            9 => Ok(Number::Nine),
            _ => Err(InvalidNumberError),
        }
    }
}
impl TryFrom<&str> for Number {
    type Error = InvalidNumberError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "one" => Ok(Number::One),
            "two" => Ok(Number::Two),
            "three" => Ok(Number::Three),
            "four" => Ok(Number::Four),
            "five" => Ok(Number::Five),
            "six" => Ok(Number::Six),
            "seven" => Ok(Number::Seven),
            "eight" => Ok(Number::Eight),
            "nine" => Ok(Number::Nine),
            _ => Err(InvalidNumberError),
        }
    }
}
impl From<Number> for &str {
    fn from(value: Number) -> &'static str {
        match value {
            Number::One => "one",
            Number::Two => "two",
            Number::Three => "three",
            Number::Four => "four",
            Number::Five => "five",
            Number::Six => "six",
            Number::Seven => "seven",
            Number::Eight => "eight",
            Number::Nine => "nine",
        }
    }
}
impl From<Number> for u64 {
    fn from(value: Number) -> u64 {
        match value {
            Number::One => 1,
            Number::Two => 2,
            Number::Three => 3,
            Number::Four => 4,
            Number::Five => 5,
            Number::Six => 6,
            Number::Seven => 7,
            Number::Eight => 8,
            Number::Nine => 9,
        }
    }
}

/// Takes a line of input and returns a u64 of the first and last number in the line
fn parse_string(input: &str) -> u64 {
    let first_num = input
        .chars()
        .find(|c| (c.is_numeric() || std::convert::TryInto::<Number>::try_into(c).is_ok()));
    let first_num = match first_num {
        Some(num) => num,
        None => panic!("invalid string"),
    };
    let last_num = input.chars().rev().find(|c| c.is_numeric());
    let last_num = match last_num {
        Some(num) => num,
        None => panic!("invalid string"),
    };
    let mut full_num: String = "".to_string();
    full_num.push(first_num);
    full_num.push(last_num);
    match str::parse::<u64>(&full_num) {
        Ok(num) => num,
        Err(_) => panic!("didn't work!"),
    }
}

fn process_line(line: &str) -> u64 {
    let mut index = 0;
    let line_iter = from_fn(move || {
        let reduced_line = &line[index..];
        let result = if reduced_line.starts_with("one") {
            Some('1')
        } else if reduced_line.starts_with("two") {
            Some('2')
        } else if reduced_line.starts_with("three") {
            Some('3')
        } else if reduced_line.starts_with("four") {
            Some('4')
        } else if reduced_line.starts_with("five") {
            Some('5')
        } else if reduced_line.starts_with("six") {
            Some('6')
        } else if reduced_line.starts_with("seven") {
            Some('7')
        } else if reduced_line.starts_with("eight") {
            Some('8')
        } else if reduced_line.starts_with("nine") {
            Some('9')
        } else {
            let result = reduced_line.chars().next();
            result
        };
        index += 1;
        result
    });
    let mut full_num: String = "".to_string();
    for c in line_iter {
        match c.is_numeric() {
            true => full_num.push(c),
            false => continue,
        }
    }
    let first = full_num.chars().next().expect("no first char");
    let last = full_num.chars().last().expect("no last char");
    match str::parse::<u64>(format!("{first}{last}").as_str()) {
        Ok(num) => num,
        Err(_) => panic!("didn't work!"),
    }
}

#[test]
fn test_d1p1() {
    let test_string = "1abc2";
    let output = parse_string(test_string);
    assert_eq!(output, 12);
}

#[test]
fn test_d1p1_2() {
    let test_string = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    let expected_arr = [29, 83, 13, 24, 42, 14, 76];
    for line in test_string.lines().enumerate() {
        let output = process_line(line.1);
        assert_eq!(output, expected_arr[line.0]);
    }
}
