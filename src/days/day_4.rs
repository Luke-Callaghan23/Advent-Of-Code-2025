
struct Room {
    tp_or_not_tp: Vec<bool>,
    width: usize,
    height: usize
}

impl std::fmt::Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (spot, tp) in self.tp_or_not_tp.iter().enumerate() {
            if *tp {
                write!(f, "@")?;
            }
            else {
                write!(f, ".")?;
            }

            let eol = spot % self.width - 1 == 0;
            if eol {
                writeln!(f, "")?;
            }
        }
        write!(f, "")
    }
}

impl Room {
    fn from (grid: String) -> Self {
        let mut grid_width: Option<usize> = None;
        let mut width = 0;

        let mut tp_or_not_tp: Vec<bool> = Vec::with_capacity(grid.len());

        // Start at 1 because we start on the first line
        let mut height: usize = 1;
        grid.chars().enumerate().for_each(| (spot, tp) | {
            if tp == '\r' { return }
            if tp == '\n' {
                if let Some(known_width) = grid_width {
                    assert!(width == known_width, "Jagged grid detected!  Expected all rows to be of length {}, but row {} has {} columns", known_width, height, width);
                }
                else {
                    grid_width = Some(width);
                }
                width = 0;
                height += 1;
                return;
            }
            width += 1;

            if tp == '.' {
                tp_or_not_tp.push(false);
            }
            else if tp == '@' {
                tp_or_not_tp.push(true);
            }
            else {
                panic!("The only character allowed in the grid are \\r, \\n, '@', and '.', but we got '{}' at index {}", tp, spot);
            }
        });

        if grid_width.is_none() {
            panic!("Width of grid not found!  No newlines detected!");
        }

        Room {
            tp_or_not_tp: tp_or_not_tp,
            width: grid_width.unwrap(),
            height: height
        }
    }

    fn get_index (&self, y: usize, x: usize) -> usize {
        return y * self.width + x;
    }

    fn get_yx (&self, spot: usize) -> (usize, usize) {
        let y = spot / self.width;
        let x = spot % self.width;
        return ( y, x );
    }

    fn get_neighbors (&self, spot: usize) -> [ Option<usize>; 8 ] {

        let ( y, x ) = self.get_yx(spot);

        let at_top = y == 0;
        let at_left = x == 0;
        let at_bottom = y == self.height - 1;
        let at_right = x == self.width - 1;

        let mut neighbors = [ None; 8 ];
        let mut neighbor_index = 0;

        for y_diff in [ -1i16, 0, 1 ] {
            if at_top && y_diff == -1 { 
                neighbor_index += 3;
                continue; 
            }
            if at_bottom && y_diff == 1 { continue; }

            for x_diff in [ -1i16, 0, 1] {
                if at_left && x_diff == -1 { neighbor_index += 1; continue; }
                if at_right && x_diff == 1 { neighbor_index += 1; continue; }
                if y_diff == 0 && x_diff == 0 {
                    continue;
                }

                let y_index = (y as i16 + y_diff) as usize;
                let x_index = (x as i16 + x_diff) as usize;
                let offset_idx = self.get_index(y_index, x_index);

                neighbors[neighbor_index] = Some(offset_idx);
                neighbor_index+=1;
            }
        }

        neighbors
    }
}



pub fn star_one (input: String) -> String {
    let tp_room = Room::from(input);
    tp_room.tp_or_not_tp.iter().enumerate().fold(0usize, | accessible_acc, (spot, tp) | {
        accessible_acc + if *tp {
            let neighbors = tp_room.get_neighbors(spot);
            let tp_neighbors: usize = neighbors.into_iter()
                .map(| neighbor_index | {
                    if let Some(neigbor_index) = neighbor_index {
                        if tp_room.tp_or_not_tp[neigbor_index] { 1usize }
                        else { 0 }
                    }
                    else { 0 }
                })
                .sum();
            if tp_neighbors < 4 { 1 } else { 0 }
        }
        else { 0 }
    }).to_string()
}

pub fn star_two (input: String) -> String {
    let mut tp_room = Room::from(input);
    let mut removed_count = 0;

    loop {
        let removed: Vec<usize> = tp_room.tp_or_not_tp.iter().enumerate().filter_map(| (spot, tp) | {
            if *tp {
                let neighbors = tp_room.get_neighbors(spot);
                let tp_neighbors: usize = neighbors.into_iter()
                    .map(| neighbor_index | {
                        if let Some(neigbor_index) = neighbor_index {
                            if tp_room.tp_or_not_tp[neigbor_index] { 1usize }
                            else { 0 }
                        }
                        else { 0 }
                    })
                    .sum();
                if tp_neighbors < 4 {
                    Some(spot)
                }
                else { None }
            }
            else { None }
        }).collect();

        if removed.len() == 0 {
            break;
        }

        for rem in &removed {
            tp_room.tp_or_not_tp[*rem] = false;
        }
        removed_count += removed.len();
    }
    return removed_count.to_string();
}

