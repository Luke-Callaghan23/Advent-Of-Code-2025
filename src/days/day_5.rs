use std::{ops::RangeInclusive};

struct StoreRoom {
    freshness_ranges: Vec<RangeInclusive<u64>>,
    ingredients: Vec<u64>
}

impl StoreRoom {

    fn eat_number (line: &str) -> (u64, &str) {
        let mut num: u64 = 0;
        let nonnum: Option<usize> = line.chars().enumerate().find_map(| (idx, chr) | {
            if chr.is_numeric() {
                let digit = chr.to_digit(10).unwrap();
                num = num * 10 + digit as u64;
                return None;
            }
            return Some(idx);
        });
        if let Some(idx) = nonnum {
            return (num as u64, &line[idx..]);
        }
        else {
            return (num as u64, &line[line.len()..]);
        }
    }

    fn from (input: String) -> Self {
        let mut lines = input.lines().peekable();
        let mut freshness_ranges: Vec<RangeInclusive<u64>> = Vec::new();
        let mut ingredients: Vec<u64> = Vec::new();

        let mut line_idx = 0;
        let mut getting_ranges = true;
        loop {
            line_idx += 1;

            let line = lines.peek();
            if line.is_none() {
                break;
            }

            let line = lines.next().unwrap();
            if line.len() == 0 {
                getting_ranges = false;
                continue;
            }

            let first_char = line.chars().next().expect("There must be a first character");
            assert!(first_char.is_numeric(), "First character on every line must be a number but we got {} on line {} (line='{}')", first_char, line_idx, line);

            if getting_ranges {
                let (start, next) = StoreRoom::eat_number(line);
                assert!(next.len() > 0, "First part of range must be followed by second");
                assert!(next.chars().next().unwrap() == '-', "First character after range start must be '-'");
                let (end, empty) = StoreRoom::eat_number(&next[1..]);
                assert!(empty.len() == 0, "Line must be empty after end range finishes");
                freshness_ranges.push(start..=end);
            }
            else {
                let (ingredient, empty) = StoreRoom::eat_number(line);
                assert!(empty.len() == 0, "Line must be empty after ingredient");
                ingredients.push(ingredient);
            }
        }

        StoreRoom { freshness_ranges , ingredients }
    }
}



pub fn star_one (input: String) -> String {
    let StoreRoom {
        ingredients,
        freshness_ranges
    } = StoreRoom::from(input);

    ingredients.into_iter().fold(0usize, | fresh_acc, ingredient | {
        let fresh = freshness_ranges.iter().any(| range | {
            range.contains(&ingredient)
        });
        return fresh_acc + if fresh { 1 } else { 0 }
    }).to_string()
}

pub fn star_two (input: String) -> String {
    let StoreRoom {
        ingredients: _ingredients,
        mut freshness_ranges
    } = StoreRoom::from(input);

    freshness_ranges.sort_by(| range_a, range_b | range_a.start().cmp(&range_b.start()));

    let mut merged_last_range = false;

    let mut merged_ranges: Vec<RangeInclusive<u64>> = Vec::new();
    let mut outer_idx = 0;
    while outer_idx < freshness_ranges.len() - 1 {
        let outer_range = &freshness_ranges[outer_idx];

        let merged_start = *outer_range.start();
        let mut merged_end = *outer_range.end();

        for inner_range in &freshness_ranges[(outer_idx+1)..] {
            if *inner_range.start() <= merged_end {
                if merged_end < *inner_range.end() {
                    merged_end = *inner_range.end();
                }
                outer_idx += 1;

                if outer_idx + 1 == freshness_ranges.len() {
                    merged_last_range = true;
                }

            }
            else { break }
        }

        merged_ranges.push(merged_start..=merged_end);
        outer_idx += 1;
    }

    if !merged_last_range {
        merged_ranges.push(freshness_ranges.last().unwrap().clone());
    }

    merged_ranges.iter().fold(0u128, | total_acc, merged_range | {
        total_acc + (merged_range.end() - merged_range.start()) as u128 + 1
    }).to_string()
}

