use std::{collections::HashSet, thread::sleep, time::Duration, u16, usize};


struct Machine {
    desired_state: u16,
    state: u16,
    buttons: Vec<u16>,
    joltage_requirement: [u8; 16],
    max_joltage_depth: usize
}

impl Machine {
    fn from (line: &str) -> Self {

        let machine_reg = regex::Regex::new(r"\[(?<desired_state>[.#]+)\] (?<buttons>(\((\d+,?)+\) )+)\{(?<joltage_requirement>(\d+,?)+)\}").unwrap();
        let cap = machine_reg.captures(line).unwrap();
        let desired_states = cap.name("desired_state").unwrap();
        let buttons = cap.name("buttons").unwrap();
        let joltage_requirement = cap.name("joltage_requirement").unwrap();

        let initial_state = 0u16;
        let desired_state = desired_states.as_str().chars().rev().fold(0u16, | mut acc, state | {
            acc = acc << 1;
            if state == '#' {
                acc + 1
            }
            else {
                acc
            }
        });

        let buttons: Vec<u16> = buttons
            .as_str()
            .split(" ")
            .filter(| button | button.len() > 0)
            .map(| button | {
                let button = &button[1..button.len()-1];
                let indicators = button.split(',');
                
                indicators.fold(0u16, | acc, indicator | {
                    acc | (1 << indicator.parse::<u16>().unwrap())
                })
            }).collect();

        let mut max_joltage_depth = 0;
        let mut joltage_requirement_arr = [0; 16];

        let joltage_requirement_str = joltage_requirement.as_str().split(",");
        for (idx, jrs) in joltage_requirement_str.into_iter().enumerate() {
            let joltage = jrs.parse::<u8>().unwrap();
            joltage_requirement_arr[idx] = joltage;
            max_joltage_depth += joltage as usize;
        }

        Machine {
            buttons,
            desired_state,
            joltage_requirement: joltage_requirement_arr,
            max_joltage_depth,
            state: initial_state
        }
    }

    fn get_min_indicator_button_presses (self) -> usize {
        self.get_min_indicator_presses_recurse(self.desired_state, None, u16::MAX, 0, HashSet::new())
    }

    // Naive recursive memoized search for pt 1 -- ~15 second runtime on my machine
    fn get_min_indicator_presses_recurse (&self, state: u16, mut min: Option<usize>, prev_choice: u16, depth: usize, memo: HashSet<u16>) -> usize {
        if depth == 10 {
            return usize::MAX;
        }

        if let Some(min) = min {
            if min <= depth {
                return usize::MAX;
            }
        }

        if state == 0 {
            return depth;
        }

        for bidx in 0..self.buttons.len() {
            let button = self.buttons[bidx];

            if button == prev_choice {
                continue;
            }

            let next_state = state ^ button;
            if memo.contains(&next_state) {
                continue;
            }

            let mut memo = memo.clone();
            memo.insert(next_state);
            let res = self.get_min_indicator_presses_recurse(next_state, min, button, depth + 1, memo);

            if let Some(min_val) = min {
                if res < min_val {
                    min = Some(res);
                }
            }
            else {
                min = Some(res);
            }
        }
        return min.unwrap();
    }

    // Iterative version -- 15 seconds as well... *shrug*
    fn get_min_indicator_button_presses_iterative (self) -> usize {
        let mut min: Option<usize> = None;

        let dp: [Vec<(          // All explorable states at depth
            u16,                    // Current state
            u16,                    // previous choice
            HashSet<u16>,           // Tracks states visited
        )>; 10] = std::array::from_fn(|_| vec![(self.desired_state, u16::MAX, HashSet::new())]);

        for depth in 0..10 {

            if let Some(min_val) = min {
                if min_val < depth {
                    return min_val;
                }
            }

            for (state, prev_choice, memo) in &dp[depth] {

                if *state == 0 {
                    return depth;
                }

                for bidx in 0..self.buttons.len() {
                    let button = self.buttons[bidx];
        
                    if button == *prev_choice {
                        continue;
                    }
        
                    let next_state = state ^ button;
                    if memo.contains(&next_state) {
                        continue;
                    }
        
                    let mut memo = memo.clone();
                    memo.insert(next_state);
                    let res = self.get_min_indicator_presses_recurse(next_state, min, button, depth + 1, memo);
        
                    if let Some(min_val) = min {
                        if res < min_val {
                            min = Some(res);
                        }
                    }
                    else {
                        min = Some(res);
                    }
                }
            }

        }
        return min.unwrap();
    }




    fn get_min_joltage_requirement_presses (self) -> usize {
        self.get_min_joltage_requirement_presses_recurse([0; 16], None, 0)
    }

    // Naive recursive memoized search for pt 1 -- ~15 second runtime on my machine
    fn get_min_joltage_requirement_presses_recurse (&self, state: [u8; 16], mut min: Option<usize>, depth: usize) -> usize {
        if depth >= self.max_joltage_depth / 10 {
            return usize::MAX;
        }

        if let Some(min) = min {
            if min <= depth {
                return usize::MAX;
            }
        }

        if state == self.joltage_requirement {
            return depth;
        }

        for bidx in 0..self.buttons.len() {
            let mut button = self.buttons[bidx];

            let mut valid_press = true;
            
            let mut next_state = state.clone();
            let mut joltage_idx = 0;
            while button > 0 {
                if button & 1 == 1 {
                    next_state[joltage_idx] += 1;

                    if next_state[joltage_idx] > self.joltage_requirement[joltage_idx] {
                        valid_press = false;
                        break;
                    }
                }
                joltage_idx += 1;
                button >>= 1;
            }

            if !valid_press {
                continue;
            }

            let res = self.get_min_joltage_requirement_presses_recurse(next_state, min, depth + 1);
            if let Some(min_val) = min {
                if res < min_val {
                    min = Some(res);
                }
            }
            else {
                min = Some(res);
            }
        }
        return min.unwrap_or(usize::MAX);
    }
}

fn fmt_bstring (num: u16 ) -> String {
    format!("{:#015b}", num)[2..].replace("0", ".").replace("1", "#")
}

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        

        write!(f, "||{}|| ", fmt_bstring(self.state))?;

        write!(f, "[{}] ", fmt_bstring(self.desired_state))?;
        for button in &self.buttons {
            write!(f, "({}) ", fmt_bstring(*button))?;
        }

        write!(f, "{{")?;
        for (idx, req) in self.joltage_requirement.iter().enumerate() {
            if idx != self.joltage_requirement.len() - 1 {
                write!(f, "{}, ", req)?
            }
            else {
                write!(f, "{}", req)?
            }
        }

        write!(f, "}}")
    }
}

fn count_bits_in_num (mut num: u16) -> u16 {
    let mut count = 0;
    while num > 0 {
        count += num & 1;
        num >>= 1;
    }
    count
}


pub fn star_one (input: String) -> String {
    input
        .lines()
        .map(Machine::from)
        .map(Machine::get_min_indicator_button_presses)
        .sum::<usize>()
        .to_string()
}

pub fn star_two (input: String) -> String {
    input
        .lines()
        .map(Machine::from)
        .map(Machine::get_min_joltage_requirement_presses)
        .sum::<usize>()
        .to_string()
}

