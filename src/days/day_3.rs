
fn get_bank_joltage (bank: &[u8], batteries_count: usize) -> u64 {
    let mut remaining_batteries = batteries_count;
    let mut current_idx = 0;

    let mut joltage: u64 = 0;
    while current_idx < bank.len() && remaining_batteries > 0 {
        let best_start_idx = get_best_start(&bank[current_idx..], remaining_batteries);
        joltage = joltage * 10 + (bank[current_idx + best_start_idx]) as u64;

        current_idx = current_idx + best_start_idx + 1;
        remaining_batteries -= 1;
    }
    return joltage;
}

fn get_best_start (bank: &[u8], batteries_count: usize) -> usize {
    let mut best_start = 0;
    let mut best_start_idx = 0;
    let mut idx = 0;
    while idx + batteries_count - 1 < bank.len() {
        let cbat = bank[idx];
        if cbat > best_start {
            best_start = cbat;
            best_start_idx = idx;
        }
        idx += 1;
    }
    return best_start_idx;
}


pub fn star_one (input: String) -> String {
    input.lines().fold(0u64, | joltage_acc, bank | {
        let bank_bytes = bank.as_bytes().iter().map(| battery_chr | {
            return battery_chr - b'0';
        }).collect::<Vec<u8>>();

        return joltage_acc + get_bank_joltage(&bank_bytes[..], 2);
    }).to_string()
}


pub fn star_two (input: String) -> String {
    input.lines().fold(0u64, | joltage_acc, bank | {
        let bank_bytes = bank.as_bytes().iter().map(| battery_chr | {
            return battery_chr - b'0';
        }).collect::<Vec<u8>>();

        return joltage_acc + get_bank_joltage(&bank_bytes[..], 12);
    }).to_string()
}

