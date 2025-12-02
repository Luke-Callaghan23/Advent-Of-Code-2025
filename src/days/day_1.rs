// Max = 99
#[derive(PartialEq, Eq)]
enum Direction {
    R, // R => Positive
    L  // L => Negative
}

impl Direction {
    fn from(c: char) -> Self {
        if c == 'R' {
            Direction::R
        }
        else if c == 'L' {
            Direction::L
        }
        else {
            panic!("Direction::from expects only 'R' or 'L' character inputs, but recieved {}", c);
        }
    }

    fn get_sign (&self) -> i32 {
        if *self == Direction::L {
            -1
        }
        else {
            1
        }
    }
}

pub fn star_one (input: String) -> String {
    let start = 50;

    let min = 0;
    let max = 99;
    let digits = 100;

    let (count_landed_at_zero, _) = input.lines()
        .enumerate()
        .fold((0usize, start), | (count_acc, dial_position), (line_num, curr_line) | {

            let mut chars = curr_line.chars();
            let direction = Direction::from(chars.next().unwrap());

            let value_str: String = chars.collect();
            let value_res = value_str.parse::<u16>();
            let value = match value_res {
                Ok(parsed) => parsed,
                Err(err) => panic!("Parsing error for line {line_num} value '{value_str}': {err}"),
            } as i32;

            let value = value % digits;

            let signed_val = direction.get_sign() * value;
            let rotation_res = dial_position + signed_val;

            let next: i32 = if rotation_res > max {
                rotation_res % digits
            }
            else if rotation_res < min {
                let post_rotate = max - ((rotation_res * -1) % digits) + 1;
                if post_rotate != 100 {
                    post_rotate
                }
                else { 0 }
            }
            else {
                rotation_res
            };

            return (count_acc + if next == 0 { 1 } else { 0 }, next);
        });

    return count_landed_at_zero.to_string();
}

pub fn star_two (input: String) -> String {
    let start = 50;

    let min = 0;
    let max = 99;
    let digits = 100;

    let (count_landed_at_zero, dial) = input.lines()
        .enumerate()
        .fold((0usize, start), | (count_acc, dial_position), (line_num, curr_line) | {

            println!("{}", dial_position);

            let mut chars = curr_line.chars();
            let direction = Direction::from(chars.next().unwrap());

            let value_str: String = chars.collect();
            let value_res = value_str.parse::<u16>();
            let value = match value_res {
                Ok(parsed) => parsed,
                Err(err) => panic!("Parsing error for line {line_num} value '{value_str}': {err}"),
            } as i32;

            print!("    val={value} ");

            let mut passed_zero = value / digits;
            let value = value % digits;

            let signed_val = direction.get_sign() * value;
            let rotation_res = dial_position + signed_val;

            print!("signed={signed_val} addition={rotation_res}");

            if dial_position != 0 && rotation_res == 0 {
                passed_zero += 1;
            }

            let next: i32 = if rotation_res > max {
                let post_rotate = rotation_res % digits;

                if dial_position != 0 {
                    passed_zero += 1;
                }

                if post_rotate != 0 {
                    post_rotate
                }
                else {
                    0
                }
            }
            else if rotation_res < min {
                let post_rotate = max - ((rotation_res * -1) % digits) + 1;

                if dial_position != 0 {
                    passed_zero += 1;
                }

                if post_rotate != 100 {
                    post_rotate
                }
                else { 
                    0 
                }
            }
            else {
                rotation_res
            };

            // 5426 - 5956

            println!(" next_val={next} add_acc={passed_zero}");

            let count_passed_zero = count_acc + passed_zero as usize;

            return (count_passed_zero, next);
        });

    println!("{}", dial);

    return count_landed_at_zero.to_string();
}

