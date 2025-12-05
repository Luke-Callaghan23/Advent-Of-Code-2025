

mod days;

use std::{fs, path::Path};
use std::time::{Duration, Instant};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long, parse(try_from_str = parse_day), help="Date to run.")]
    day: u8,

    #[structopt(long, help="Flag to tell whether to use the example data set or the full data.")]
    example: bool,

    #[structopt(short,  long, help="Flag to tell whether to run the second test or not.")]
    second: bool,

    #[structopt(short, long, default_value="./data", help="Root folder of data directory.  Should contain 25 files [1...25] each with an \"example.txt\" and \"full.txt\" file for running with AOC data.")]
    input_folder: String,

    #[structopt(short = "o", long = "other", help="Flag to tell whether to run the \"other\" implementation.  Only applicable for days where you've done another implementation.")]
    other_impl: bool,
}

fn parse_day(s: &str) -> Result<u8, &'static str> {
    let num: u8 = s.parse().map_err(|_| "Not a valid number")?;
    match num {
        1..26 => Ok(num),
        _ => Err("--day must be between 1 and 25"),
    }
}

fn format_duration(duration: Duration) -> String {
    let total_nanos = duration.as_nanos();

    let hours = (total_nanos / (1_000_000_000 * 60 * 60)) % 24;
    let minutes = (total_nanos / (1_000_000_000 * 60)) % 60;
    let seconds = (total_nanos / 1_000_000_000) % 60;

    let nanoseconds = total_nanos % 1_000_000_000;

    // Add commas to the nano seconds
    // Maybe there's a better way to do this, but for now I'm just reversing the string, replacing every three pairs
    //      of digits with those same digits plus a comma, then reversing again
    let nano_str = nanoseconds.to_string().chars().rev().collect::<String>();
    let reg = regex::Regex::new(r"(\d{3})").unwrap();
    let nano_comma_str= reg.replace_all(&nano_str, "$1,").chars().rev().collect::<String>();

    format!("{:02}:{:02}:{:02} {} nanos", hours, minutes, seconds, nano_comma_str)
}

fn main() {
    let opt = Opt::from_args();
    let day = opt.day;
    let is_example_file = opt.example;
    let is_second_test = opt.second;

    let data_folder_root = opt.input_folder;
    let mut path = Path::new(&data_folder_root).join(day.to_string());
    if is_example_file {
        path = path.join("example.txt")
    }
    else {
        path = path.join("full.txt")
    }

    let file_read_result = fs::read_to_string(&path);
    let file_contents = match file_read_result {
        Ok(contents) => contents,
        Err(err) => panic!("An error occurred while reading the data file at '{}': {}", path.as_os_str().to_str().unwrap(), err),
    };

    let start = Instant::now();

    let result = match day {
        1 => {
            if !is_second_test {
                days::day_1::star_one(file_contents)
            }
            else {
                days::day_1::star_two(file_contents)
            }
        },
        2 => {
            if !is_second_test {
                if opt.other_impl {
                    days::day_2_regex_impl::star_one(file_contents)
                }
                else {
                    days::day_2::star_one(file_contents)
                }
            }
            else {
                if opt.other_impl {
                    days::day_2_regex_impl::star_two(file_contents)
                }
                else {
                    days::day_2::star_two(file_contents)
                }
            }
        },
        3 => {
            if !is_second_test {
                days::day_3::star_one(file_contents)
            }
            else {
                days::day_3::star_two(file_contents)
            }
        },
        4 => {
            if !is_second_test {
                days::day_4::star_one(file_contents)
            }
            else {
                days::day_4::star_two(file_contents)
            }
        },
        5 => {
            if !is_second_test {
                days::day_5::star_one(file_contents)
            }
            else {
                days::day_5::star_two(file_contents)
            }
        },
        6 => {
            if !is_second_test {
                days::day_6::star_one(file_contents)
            }
            else {
                days::day_6::star_two(file_contents)
            }
        },
        7 => {
            if !is_second_test {
                days::day_7::star_one(file_contents)
            }
            else {
                days::day_7::star_two(file_contents)
            }
        },
        8 => {
            if !is_second_test {
                days::day_8::star_one(file_contents)
            }
            else {
                days::day_8::star_two(file_contents)
            }
        },
        9 => {
            if !is_second_test {
                days::day_9::star_one(file_contents)
            }
            else {
                days::day_9::star_two(file_contents)
            }
        },
        10 => {
            if !is_second_test {
                days::day_10::star_one(file_contents)
            }
            else {
                days::day_10::star_two(file_contents)
            }
        },
        11 => {
            if !is_second_test {
                days::day_11::star_one(file_contents)
            }
            else {
                days::day_11::star_two(file_contents)
            }
        },
        12 => {
            if !is_second_test {
                days::day_12::star_one(file_contents)
            }
            else {
                days::day_12::star_two(file_contents)
            }
        },
        13 => {
            if !is_second_test {
                days::day_13::star_one(file_contents)
            }
            else {
                days::day_13::star_two(file_contents)
            }
        },
        14 => {
            if !is_second_test {
                days::day_14::star_one(file_contents)
            }
            else {
                days::day_14::star_two(file_contents)
            }
        },
        15 => {
            if !is_second_test {
                days::day_15::star_one(file_contents)
            }
            else {
                days::day_15::star_two(file_contents)
            }
        },
        16 => {
            if !is_second_test {
                days::day_16::star_one(file_contents)
            }
            else {
                days::day_16::star_two(file_contents)
            }
        },
        17 => {
            if !is_second_test {
                days::day_17::star_one(file_contents)
            }
            else {
                days::day_17::star_two(file_contents)
            }
        },
        18 => {
            if !is_second_test {
                days::day_18::star_one(file_contents)
            }
            else {
                days::day_18::star_two(file_contents)
            }
        },
        19 => {
            if !is_second_test {
                days::day_19::star_one(file_contents)
            }
            else {
                days::day_19::star_two(file_contents)
            }
        },
        20 => {
            if !is_second_test {
                days::day_20::star_one(file_contents)
            }
            else {
                days::day_20::star_two(file_contents)
            }
        },
        21 => {
            if !is_second_test {
                days::day_21::star_one(file_contents)
            }
            else {
                days::day_21::star_two(file_contents)
            }
        },
        22 => {
            if !is_second_test {
                days::day_22::star_one(file_contents)
            }
            else {
                days::day_22::star_two(file_contents)
            }
        },
        23 => {
            if !is_second_test {
                days::day_23::star_one(file_contents)
            }
            else {
                days::day_23::star_two(file_contents)
            }
        },
        24 => {
            if !is_second_test {
                days::day_24::star_one(file_contents)
            }
            else {
                days::day_24::star_two(file_contents)
            }
        },
        25 => {
            if !is_second_test {
                days::day_25::star_one(file_contents)
            }
            else {
                days::day_25::star_two(file_contents)
            }
        }
        _ => panic!("Unexpected value for --day: {}", day)
    };

    let elapsed = start.elapsed();
    println!("Result: \n{result}");
    println!("Elapsed: {}", format_duration(elapsed));
}

