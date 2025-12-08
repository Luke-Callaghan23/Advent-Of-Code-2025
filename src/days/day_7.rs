use std::{cell::RefCell, collections::HashSet, rc::Rc};


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

    fn from (char_num: usize, width: usize) -> Self {
        RowCol(char_num / width, char_num % width)
    }
}

struct TachyonManifold {
    splitters: HashSet<RowCol>,
    tachyons: HashSet<RowCol>,
    width: usize,
    height: usize,
}

struct QuantumTachyonManifold {
    splitters: Rc<HashSet<RowCol>>,
    tachyon: RowCol,
    width: usize,
    height: usize,
}

enum CatInABox {
    Down(QuantumTachyonManifold),
    Split(Option<QuantumTachyonManifold>, Option<QuantumTachyonManifold>),
    JobsDone
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

pub fn star_two (input: String) -> String {
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

