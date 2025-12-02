
pub fn star_one (input: String) -> String {
    let invalid_ids = input.split(',').enumerate()
        .fold(0u128, | invalid_acc, (range_num, curr_range)| {
            
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

                let chunk_size  = nstr.len() / 2;

                let chunks_in_str = nstr.len() / chunk_size;

                let mut valid = true;
                let mut prev_chunk = &nstr[0..chunk_size];


                for chunk_idx in 1..chunks_in_str {
                    let chunk = &nstr[(chunk_idx*chunk_size)..(chunk_idx*chunk_size)+chunk_size];
                    if prev_chunk == chunk {
                        prev_chunk = chunk;
                    }
                    else {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    return range_acc + n;
                }
                return range_acc;
            })
        });

    invalid_ids.to_string()
}

pub fn star_two (input: String) -> String {
    let invalid_ids = input.split(',').enumerate()
        .fold(0u128, | invalid_acc, (range_num, curr_range)| {
            
            let mut  splt = curr_range.split('-');
            let sstr = splt.next().unwrap();
            let estr = splt.next().unwrap();

            let start = sstr.parse::<u128>().unwrap();
            let end = estr.parse::<u128>().unwrap();

            (start..=end).fold(invalid_acc, | range_acc, n | {
                let nstr = n.to_string();

                for chunk_size in 1..=(nstr.len() / 2) {
                    if nstr.len() % chunk_size != 0 {
                        continue;
                    }

                    let chunks_in_str = nstr.len() / chunk_size;

                    let mut valid = true;
                    let mut prev_chunk = &nstr[0..chunk_size];
                    for chunk_idx in 0..chunks_in_str {
                        let chunk = &nstr[(chunk_idx*chunk_size)..(chunk_idx*chunk_size)+chunk_size];
                        if prev_chunk == chunk {
                            prev_chunk = chunk;
                        }
                        else {
                            valid = false;
                            break;
                        }
                    }

                    if valid {
                        return range_acc + n;
                    }
                }
                return range_acc;
            })
        });

    invalid_ids.to_string()
}

