import os

template = '''

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

    #[structopt(short, long, default_value="./data", help="Root folder of data directory.  Should contain 25 files [1...25] each with an \\"example.txt\\" and \\"full.txt\\" file for running with AOC data.")]
    input_folder: String,
}

fn parse_day(s: &str) -> Result<u8, &'static str> {
    let num: u8 = s.parse().map_err(|_| "Not a valid number")?;
    match num {
        1..26 => Ok(num),
        _ => Err("--day must be between 1 and 25"),
    }
}

fn format_duration(duration: Duration) -> String {
    let total_millis = duration.as_millis();

    let hours = (total_millis / (1000 * 60 * 60)) % 24;
    let minutes = (total_millis / (1000 * 60)) % 60;
    let seconds = (total_millis / 1000) % 60;
    let milliseconds = total_millis % 1000;

    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, milliseconds)
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
        [[match_code]]
        _ => panic!("Unexpected value for --day: {}", day)
    };

    let elapsed = start.elapsed();
    println!("Result:\n{result}");
    println!("Elapsed: {}", format_duration(elapsed));
}

'''

match_template = '''{day} => {{
            if !is_second_test {{
                days::day_{day}::star_one(file_contents)
            }}
            else {{
                days::day_{day}::star_two(file_contents)
            }}
        }}'''

joiner = ',\n        '

matches = []
for day in range(1, 26):
    matches.append(match_template.format(day=day))

match_code = joiner.join(matches)
main = template.replace('[[match_code]]', match_code)

main_file = os.path.join('src', 'main.rs') 
with open(main_file, 'w') as f:
    f.write(main)