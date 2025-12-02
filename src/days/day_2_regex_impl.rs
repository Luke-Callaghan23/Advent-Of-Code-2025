use fancy_regex::Regex;

pub fn star_one (input: String) -> String {
    let invalid_ids = input.split(',').enumerate()
        .fold(0u128, | invalid_acc, (_, curr_range)| {
            
            let mut  splt = curr_range.split('-');
            let sstr = splt.next().unwrap();
            let estr = splt.next().unwrap();

            let start = sstr.parse::<u128>().unwrap();
            let end = estr.parse::<u128>().unwrap();

            (start..=end).fold(invalid_acc, | range_acc, n | {
                let nstr = n.to_string();
                if nstr.len() % 2 != 0{
                    return range_acc;
                }

                let regex: Regex = Regex::new(r"^(\d+)\1$").unwrap();
                if regex.is_match(&nstr).unwrap() {
                    range_acc + n
                }
                else {
                    range_acc
                }

            })
        });

    invalid_ids.to_string()
}

pub fn star_two (input: String) -> String {
    let invalid_ids = input.split(',').enumerate()
        .fold(0u128, | invalid_acc, (_, curr_range)| {
            
            let mut  splt = curr_range.split('-');
            let sstr = splt.next().unwrap();
            let estr = splt.next().unwrap();

            let start = sstr.parse::<u128>().unwrap();
            let end = estr.parse::<u128>().unwrap();

            (start..=end).fold(invalid_acc, | range_acc, n | {
                let nstr = n.to_string();

                let regex: Regex = Regex::new(r"^(\d+)\1+$").unwrap();
                if regex.is_match(&nstr).unwrap() {
                    range_acc + n
                }
                else {
                    range_acc
                }

            })
        });

    invalid_ids.to_string()
}

