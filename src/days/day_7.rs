use std::{collections::HashSet, rc::Rc};


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct RowCol (usize, usize);

impl std::fmt::Display for RowCol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(row={}, col={})", self.0, self.1)
    }
}

impl RowCol {
    fn from_optional_width (char_num: usize, width: Option<usize>) -> Self {
        if let Some(width) = width {
            RowCol(char_num / width, char_num % width)
        }
        else {
            RowCol(0, char_num)
        }
    }

    fn from_coord (row: usize, col: usize) -> Self {
        RowCol(row, col)
    }
}

struct TachyonManifold {
    splitters: HashSet<RowCol>,
    tachyons: HashSet<RowCol>,
    width: usize,
    height: usize,
}


impl TachyonManifold {
    fn from (input: String) -> Self {
        let mut width: Option<usize> = None;
        let mut start: Option<RowCol> = None;
        let (total_chars, splitters) = input.chars().fold((0, HashSet::<RowCol>::new()), | (char_num, mut splitters), chr | {
            if chr == '\n' || chr == '\r' {
                if width.is_none() {
                    width = Some(char_num);
                }
                return (char_num, splitters);
            }

            if chr == 'S' {
                let pos= RowCol::from_optional_width(char_num, width);
                if let Some(start) = &start {
                    panic!("Multiple starting spots found! There is an 'S' {} (character {}), but we previously found one at {}", pos, char_num, start)
                }
                start = Some(pos);
            }

            if chr == '^' {
                splitters.insert(RowCol::from_optional_width(char_num, width));
            }

            (char_num + 1, splitters)
        });

        // Unpack the start coordinate and insert it into a hash set (because the splitters will cause new beams to appear)
        let tachyons = if let Some(start) = start {
            HashSet::<RowCol>::from([ start ])
        }
        else {
            panic!("Could not find the starting position of the first Tachyon!  No 'S' characters were found!");
        };

        let width = if let Some(width) = width {
            width
        }
        else {
            panic!("Could not calculate the width of the TachyonManifold! This is likely because there were no newlines (\\n) or carriage returns (\\n) in the input!");
        };

        // Grid must be equal columns per row
        let height = total_chars / width;
        TachyonManifold { splitters, tachyons, width, height }
    }

    fn step (self) -> (Self, usize, bool) {
        let TachyonManifold {
            splitters,
            tachyons,
            width,
            height,
        } = self;

        let mut splits = 0;
        let mut next_tachyons = HashSet::<RowCol>::new();

        let initial_tachyons_count = tachyons.len();
        let mut tachyons_reached_bottom = 0;
        for RowCol(trow, tcol) in tachyons {
            if trow == height - 1 {
                tachyons_reached_bottom += 1;
                continue;
            }

            let next = RowCol(trow + 1, tcol);
            if splitters.contains(&next) {
                if tcol != 0 {
                    next_tachyons.insert(RowCol(trow + 1, tcol - 1));
                }
                if tcol != width - 1 {
                    next_tachyons.insert(RowCol(trow + 1, tcol + 1));
                }
                splits += 1;
            }
            else {
                next_tachyons.insert(next);
            }
        }

        let can_continue = initial_tachyons_count != tachyons_reached_bottom;

        (
            TachyonManifold {
                splitters: splitters,
                tachyons: next_tachyons,
                width,
                height,
            },
            splits,
            can_continue
        )
    }
}

impl std::fmt::Display for TachyonManifold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let pos = RowCol::from_coord(row, col);

                if self.splitters.contains(&pos) {
                    write!(f, "^")?
                }
                else if self.tachyons.contains(&pos) {
                    write!(f, "|")?
                }
                else {
                    write!(f, ".")?
                }
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}

pub fn star_one (input: String) -> String {
    let mut split_count = 0;
    let mut tachyon_manifold = TachyonManifold::from(input);
    loop {
        let (ntm, splits, can_continue) = tachyon_manifold.step();
        tachyon_manifold = ntm;
        split_count += splits;

        if !can_continue {
            break;
        }
    }
    split_count.to_string()
}



struct QuantumTachyonManifold {
    splitters: Rc<HashSet<RowCol>>,
    tachyon: RowCol,
    width: usize,
    height: usize,
}

#[allow(unused)]
enum CatInABox {
    Down(QuantumTachyonManifold),
    Split(Option<QuantumTachyonManifold>, Option<QuantumTachyonManifold>),
    JobsDone
}

#[allow(unused)]
impl QuantumTachyonManifold {

    fn from (binary_tachyon_manifold: TachyonManifold) -> Self {
        let TachyonManifold {
            splitters,
            tachyons,
            width,
            height,
        } = binary_tachyon_manifold;

        let splitters_ref = Rc::new(splitters);
        let tachyon = tachyons.into_iter().next().expect("There must be at least one tachyon in the original tachyon manifold to create a quantum tachyon mnaifold from it");
        QuantumTachyonManifold { splitters: splitters_ref, tachyon, width, height }
    }

    fn step_quantumly (self) -> CatInABox {

        let QuantumTachyonManifold {
            splitters,
            tachyon,
            width,
            height,
        } = self;

        // step_quantumly assumes only one tachyon at a time
        let RowCol(trow, tcol) = tachyon;
        
        if trow == height - 1 {
            return CatInABox::JobsDone;
        }

        let next = RowCol(trow + 1, tcol);
        if splitters.contains(&next) {
            let left = if tcol != 0 {
                Some(QuantumTachyonManifold {
                    splitters: splitters.clone(),
                    tachyon: RowCol(trow + 1, tcol - 1),
                    width,
                    height,
                })
            }
            else { None };

            let right = if tcol != width - 1 {
                Some(QuantumTachyonManifold {
                    splitters: splitters.clone(),
                    tachyon: RowCol(trow + 1, tcol + 1),
                    width,
                    height,
                })
            }
            else { None };

            CatInABox::Split(left, right)
        }
        else {
            CatInABox::Down(QuantumTachyonManifold {
                splitters: splitters,
                tachyon: next,
                width,
                height,
            })
        }
    }

}

// Totally made this naive implementation on purpose to show what a less experienced programmer would do
fn _star_two_naive (input: String) -> String {
    let mut parallel_universes = 1;
    let root_universe = TachyonManifold::from(input);
    let root_universe_quantum = QuantumTachyonManifold::from(root_universe);
    let mut current_universes = vec![ root_universe_quantum ];

    loop {
        if current_universes.len() == 0 {
            break;
        }

        println!("Current universes: {}", current_universes.len());
        println!("Row = {}", current_universes.get(0).unwrap().tachyon.0);
        println!("Height = {}", current_universes.get(0).unwrap().height);
        println!("Mem = {} GB", (std::mem::size_of::<QuantumTachyonManifold>() * current_universes.capacity()) as f64 / (1024f64 * 1024f64 * 1024f64));
    
        let mut step_universes: Vec<QuantumTachyonManifold> = Vec::new();
        for tachyon_manifold in current_universes {
            let quantum_step = tachyon_manifold.step_quantumly();
            match quantum_step {
                CatInABox::JobsDone => {},
                CatInABox::Down(tachyon_manifold) => step_universes.push(tachyon_manifold),
                CatInABox::Split(left, right) => {
                    let _ = left.and_then(| left | Some(step_universes.push(left)));
                    let _ = right.and_then(| right | Some(step_universes.push(right)));
                    parallel_universes += 1;
                },
            }
        }
        current_universes = step_universes;
    }
    parallel_universes.to_string()
}


struct EfficientQuantumTachyonManifold {
    splitters: Vec<HashSet<usize>>,
    initial_tachyon: RowCol,
    width: usize,
    height: usize,
}

impl EfficientQuantumTachyonManifold {
    fn from (binary_tachyon_manifold: TachyonManifold) -> Self {
        let TachyonManifold {
            splitters,
            tachyons,
            width,
            height,
        } = binary_tachyon_manifold;

        let tachyon = tachyons.into_iter().next().expect("There must be at least one tachyon in the original tachyon manifold to create a(n efficient) quantum tachyon mnaifold from it");

        let mut rowise_splitters: Vec<HashSet<usize>> = Vec::with_capacity(height);
        for _ in 0..height {
            rowise_splitters.push(HashSet::new());
        }

        for split in splitters {
            let RowCol(row, col) = split;
            let row_hash = rowise_splitters.get_mut(row).expect("All rows do not have a hash set");
            row_hash.insert(col);
        }
        EfficientQuantumTachyonManifold { splitters: rowise_splitters, initial_tachyon: tachyon, width, height }
    }

    fn step_to_bottom_quantumly (self) -> usize {
        let EfficientQuantumTachyonManifold {
            splitters,
            initial_tachyon,
            width,
            height,
        } = self;

        let RowCol(start_row, start_tachyon) = initial_tachyon;

        // Counts the number of tachyons on this row (indexed by column) in all possible universes
        // Since we just do one pass going down the map, we really need to keep track of is where in the (current) row all the tachyons are located
        // Also, since there are never any tachyons right next to each other (I believe this should be true across all inputs), we can always use the
        //      the same `tachyons_on_row` without worrying about overwriting any colliding data
        let mut tachyons_on_row = vec![0usize; width];
        
        // Start out with one tachyon in the starting spot
        tachyons_on_row[start_tachyon] = 1;
        
        // Prevent off-by-one errors :)
        let mut parallel_universes_explored = 1;
        
        for row in (start_row + 1)..height {

            // All the splitters on this row
            let row_splitters = &splitters[row];

            // Iterate over all splitters on the current row
            // Because the tachyons do nothing but go down unless if they hit a splitter, we can save some cycles by only checking
            //      the splitters on the row and checking if any of them collide with tachyons (instead of the other way around)
            for splitter_col in row_splitters {
                let splitter_col = *splitter_col;

                // Get the count of tachyons currently in this row, in this splitter's column
                let tachyons_in_spot = tachyons_on_row[splitter_col];
                
                if tachyons_in_spot == 0 {
                    // If there are no collisions with the current splitter in the currnent row, skip
                    continue;
                }

                // If there is a collision all the tachyons in this spot effectively double
                // And [count of tachyons] more parallel universes are explored
                parallel_universes_explored += tachyons_in_spot;
                
                // If there is space to go left, ADD the count of tachyons into the space to the left
                // ADD the tachyons, not replace, because there may already be tachyons in the spot to the left
                if splitter_col != 0 {
                    tachyons_on_row[splitter_col - 1] += tachyons_in_spot;
                }

                // Same with the right
                if splitter_col != width - 1 {
                    tachyons_on_row[splitter_col + 1] += tachyons_in_spot;
                }

                // After the splitter, there is no longer a beam in this spot
                tachyons_on_row[splitter_col] = 0;

            }
        }

        parallel_universes_explored
    }
}

pub fn star_two (input: String) -> String {
    let binary_tachyon_manifold = TachyonManifold::from(input);
    let efficient_quantum_tachyon_manifold = EfficientQuantumTachyonManifold::from(binary_tachyon_manifold);
    efficient_quantum_tachyon_manifold.step_to_bottom_quantumly().to_string()
}
