
#[derive(Copy, Clone)]
enum Op {
    Mult,
    Add
}

impl Op {
    fn from (chr: char) -> Self {
        if chr == '*' {
            Self::Mult
        }
        else if chr == '+' {
            Self::Add
        }
        else {
            panic!("The only acceptable characters for operations are '*' and '+'!");
        }
    }
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Op::Mult => '*',
            Op::Add => '+',
        })
    }
}

struct Column {
    values: Vec<u128>,
    operation: Op,
}

impl Column {
    fn solve (&self) -> u128 {
        let mut solve: u128 = match self.operation {
            Op::Mult => 1,
            Op::Add => 0,
        };
        for val in &self.values {
            solve = match self.operation {
                Op::Mult => solve * val,
                Op::Add => solve + val,
            };
        }
        solve
    }
}

impl std::fmt::Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, val) in self.values.iter().enumerate() {
            if index != self.values.len() - 1 {
                write!(f, "{} {} ", val, self.operation)?
            }
            else {
                write!(f, "{} ", val)?
            }
        }
        write!(f, "= {}", self.solve())
    }
}

enum Worksheet {
    Human ( HumanWorksheet ),
    Cephalopod ( CephalopodWorksheet )
}


struct HumanWorksheet {
    columns: Vec<Column>
}

struct CephalopodWorksheet {
    columns: Vec<Column>
}

impl Worksheet {
    fn solve (&self) -> u128 {
        let columns = match self {
            Worksheet::Human(human_worksheet) => {
                &human_worksheet.columns
            },
            Worksheet::Cephalopod(cephalopod_worksheet) => {
                &cephalopod_worksheet.columns
            },
        };

        columns.iter().map(| col | {
            col.solve()
        }).sum()
    }
}

impl HumanWorksheet {
    fn eat_number_skip_whitespace (line: &str) -> (Option<u128>, &str) {
        let mut num: Option<u128> = None;
        let nonnum: Option<usize> = line.chars().enumerate().find_map(| (idx, chr) | {
            // Only if we haven't found a digit yet, we can skip the whitespace
            if chr.is_whitespace() && num.is_none() {
                return None;
            }
            if chr.is_numeric() {
                let digit = chr.to_digit(10).unwrap();
                num = Some(num.unwrap_or(0) * 10 + digit as u128);
                return None;
            }
            return Some(idx);
        });
        if let Some(idx) = nonnum {
            return (num, &line[idx..]);
        }
        else {
            return (num, &line[line.len()..]);
        }
    }

    fn eat_next_char_skip_whitespace (line: &str) -> (Option<char>, &str) {
        let mut opt_chr: Option<char> = None;
        let non_ws: Option<usize> = line.chars().enumerate().find_map(| (idx, chr) | {
            if chr.is_whitespace() {
                return None;
            }
            opt_chr = Some(chr);
            return Some(idx + 1);
        });

        if let Some(idx) = non_ws {
            return (opt_chr, &line[idx..]);
        }
        else {
            return (opt_chr, &line[line.len()..]);
        }
    }

    fn from (input: String) -> Self {
        
        let mut ops: Vec<Op> = Vec::new();
        let mut numbers: Vec<u128> = Vec::new();

        let mut last_width: Option<usize> = None;
        for (line_index, line) in input.lines().enumerate() {

            let mut width = 0;
            
            let mut line_slice = line;
            while line_slice.len() > 0 {

                if line_slice.contains('+') || line_slice.contains('*') {
                    let (chr, slice) = HumanWorksheet::eat_next_char_skip_whitespace(line_slice);
                    if let Some(chr) = chr {
                        ops.push(Op::from(chr));
                        width += 1;
                    }
                    line_slice = slice;
                }
                else {
                    let ( num, slice ) = HumanWorksheet::eat_number_skip_whitespace(line_slice);
                    if let Some(num) = num {
                        numbers.push(num);
                        width += 1;
                    }
                    line_slice = slice;
                }
            }
            
            if let Some(last_width) = last_width {
                if last_width != width {
                    panic!("Jagged worksheet detected! Line {} has {} elements, but the previous line had {}", line_index + 1, width, last_width);
                }
            }
            last_width = Some(width);


        }

        if last_width.is_none() {
            panic!("Width of worksheet couldn't be found!");
        }

        
        let width = last_width.unwrap();
        let mut columns: Vec<Column> = Vec::with_capacity(width);

        for col in 0..width {
            let op = ops.get(col).unwrap();
            columns.push(Column { values: Vec::new(), operation: *op });
        }

        for (idx, num) in numbers.iter().enumerate() {
            let col_num = idx % width;
            let col = columns.get_mut(col_num).unwrap();
            col.values.push(*num);
        }

        Self {
            columns: columns
        }
    }
}

impl std::fmt::Display for Worksheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cols = match self {
            Worksheet::Human(human_worksheet) => &human_worksheet.columns,
            Worksheet::Cephalopod(cephalopod_worksheet) => &cephalopod_worksheet.columns,
        };

        for col in cols {
            writeln!(f, "{}", col)?;
        }
        Ok(())
    }

}


impl CephalopodWorksheet {
    fn from(input: String) -> Self {

        let lines: Vec<String> = input.lines().map(String::from).collect();

        let mut op_stack: Vec<Op> = Vec::new();

        let mut first = true;
        let mut prev = 0;

        let mut last_idx = 0;
        let last_line = lines.last().unwrap();
        let mut nums_per_column: Vec<usize> = last_line.chars().enumerate().filter_map(| (idx, chr) | {
            if first { 
                op_stack.push(Op::from(chr));
                first = false;
                None 
            }
            else if chr == '+' || chr == '*' {
                last_idx = idx;
                op_stack.push(Op::from(chr));
                let ret = idx - 1 - prev;
                prev = idx;
                Some(ret)
            }
            else { None }
        }).collect();

        
        // Never added the last column in the loop above, but we know it
        let mut col_size = last_line.len() - last_idx;
        // println!("{col_size}");
        let mut num_index = 0;
        let mut col: Vec<u128> = vec![ 0; col_size ];
        let mut x_back = 0;
        let mut cols: Vec<Column> = Vec::new();
        'outer: loop {
            
            for y in 0..lines.len() {
                let row = lines.get(y).unwrap()[..].as_bytes();
                let x = row.len() - 1 - x_back;

                let chr = row[x];
                if chr == b'*' ||  chr == b'+' || y == lines.len() - 1 {

                    if chr == b'+' || chr == b'*' {
                        let c= Column { values: col.clone(), operation: Op::from(chr as char) };
                        cols.push(c);
                        
                        if nums_per_column.len() == 0 {
                            break 'outer;
                        }

                        col_size = nums_per_column.pop().unwrap();
                        col = vec![ 0; col_size ];
                        x_back += 1;
                        num_index = 0;
                    }
                    else {
                        num_index += 1;
                    }
                    break;
                }
                else if chr == b' ' {
                    continue;
                }
                else {
                    let num = col.get_mut(num_index).expect("Catastrophic failure!!!");
                    let digit = chr - b'0';
                    *num = *num * 10 + digit as u128;
                }
            }
            x_back += 1;

        }


        // todo!()
        // let input: String = input.chars().rev().collect();
        // let human_ws = HumanWorksheet::from(input);

        // let mut ceph_columns = Vec::with_capacity(human_ws.columns.len());

        // // 
        // for col in human_ws.columns {
        //     let max_per_col = *col.values.iter().max().unwrap();
        //     let max_human_digits = max_per_col.ilog10() as u8 + 1;

        //     let mut ceph_col: Vec<u128> = Vec::with_capacity(col.values.len());
            
        //     for curr_human_digit_idx in 0..max_human_digits {
        //         let mut ceph_val: u128 = 0;
        //         for (_col_idx, human_val) in col.values.iter().rev().enumerate() {
        //             let human_div = human_val / 10u128.pow(max_human_digits as u32 - curr_human_digit_idx as u32 - 1);
        //             let digit = human_div % (10);
        //             ceph_val = ceph_val * 10 + digit;
        //             println!("{:0width$}[{}] -- {} ({})", human_val, curr_human_digit_idx, digit, ceph_val, width=(max_human_digits as usize));
        //         }
        //         println!("");
        //         ceph_col.push(ceph_val);
        //     }

        //     // todo!();

        //     println!("=====================\n");
            
        //     ceph_columns.push(Column { 
        //         values: ceph_col,
        //         operation: col.operation
        //     })
        // }


        // CephalopodWorksheet { columns: ceph_columns }

        CephalopodWorksheet { columns: cols }
    }
}


pub fn star_one (input: String) -> String {
    let human_ws = HumanWorksheet::from(input);
    let ws = Worksheet::Human(human_ws);
    ws.solve().to_string()
}

pub fn star_two (input: String) -> String {
    let cephalopod_ws = CephalopodWorksheet::from(input);
    let ws = Worksheet::Cephalopod(cephalopod_ws);
    ws.solve().to_string()
}

