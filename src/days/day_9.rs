use std::collections::{BTreeMap, HashMap, HashSet};
use owo_colors::{OwoColorize, colors::xterm::PigmentIndigo, colors::*};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point(usize, usize);

struct GreenPoint {
    r1: Point,
    r2: Point,

    position: Point,
}

impl Point {
    fn from (line: &str) -> Self {
        let mut xy = line.split(",");
        let x = usize::from_str_radix(xy.next().unwrap(), 10).unwrap();
        let y = usize::from_str_radix(xy.next().unwrap(), 10).unwrap();
        Point(x, y)
    }

    fn shares_a_line_with (&self, other: &Point) -> bool {
        self.0 == other.0 || self.1 == other.1
    }
}

struct Floor {
    points: Vec<Point>
}

impl Floor {
    fn from (input: String) -> Self {
        let points = input.lines().map(Point::from).collect::<Vec<Point>>();
        Floor { points }
    }
}


fn get_pairs (points: &Vec<Point>) -> Vec<(&Point, &Point)> {
    let mut pairs: Vec<(&Point, &Point)> = Vec::new();
    for outer in 0..points.len() {
        for inner in outer+1..points.len() {
            pairs.push((&points[outer], &points[inner]));
        }
    }
    pairs
}



pub fn star_one (input: String) -> String {
    let floor = Floor::from(input);
    let mut max_area = 0;
    for (Point(p1_x, p1_y), Point(p2_x, p2_y)) in get_pairs(&floor.points) {
        let rect_x = (p1_x.max(p2_x) - p1_x.min(p2_x)) + 1;
        let rect_y = (p1_y.max(p2_y) - p1_y.min(p2_y)) + 1;
        let area = rect_x * rect_y;
        if area > max_area {
            max_area = area;
        }
    }
    max_area.to_string()
}

struct Bounds {
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

impl Bounds {
    fn from (point_set: &HashSet<Point>) -> Self {
        assert!(point_set.len() > 0, "Bounds for a point set must recieve a non-empty point set");

        let cmp_x = | a: &&Point, b: &&Point | a.0.cmp(&b.0);
        let cmp_y = | a: &&Point, b: &&Point | a.1.cmp(&b.1);

        let min_x = point_set.iter().min_by(cmp_x).unwrap().0;
        let max_x = point_set.iter().max_by(cmp_x).unwrap().0;
        let min_y = point_set.iter().min_by(cmp_y).unwrap().1;
        let max_y = point_set.iter().max_by(cmp_y).unwrap().1;
        return Bounds {
            min_x: min_x,
            max_x: max_x,
            min_y: min_y,
            max_y: max_y
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RedPointKind {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight
}

#[derive(Debug)]
struct ChristmasFloor {
    points: Vec<Point>,
    points_set: HashSet<Point>,
    green_points_set: HashSet<Point>,
    red_tile_kinds: HashMap<Point, RedPointKind>,
    void: HashSet<Point>,
    solution: Option<(Point, Point)>,
    // x_ascending: Vec<Point>,
    // y_ascending: Vec<Point>,
    // x_ascending_map: BTreeMap<usize, Vec<Point>>,
    // y_ascending_map: BTreeMap<usize, Vec<Point>>,
}


impl ChristmasFloor {

    fn connect_points (prev_pt: Point, pt: &Point, green_pts: &mut HashSet<Point>, green_line_pts: &mut HashMap<Point, GreenPoint>) {
        let Point(prex, prey) = prev_pt;
        let Point(curx, cury) = *pt;

        let matchx = prex == curx;
        let matchy = prey == cury;

        assert!((matchx || matchy) && !(matchx && matchy), "Either the x or the y of the previous point MUST match with the current point");
        if matchx {
            let start_y = prey.min(cury) + 1;
            let end_y = prey.max(cury);

            for y in start_y..end_y {
                let line_pt = Point(prex, y);
                green_pts.insert(line_pt);
                green_line_pts.insert(line_pt, GreenPoint { 
                    r1: prev_pt, 
                    r2: *pt, 
                    position: line_pt
                });
            }
        }
        else {
            
            let start_x = prex.min(curx) + 1;
            let end_x = prex.max(curx);

            for x in start_x..end_x {
                let line_pt = Point(x, prey);
                green_pts.insert(line_pt);
                green_line_pts.insert(line_pt, GreenPoint { 
                    r1: prev_pt, 
                    r2: *pt, 
                    position: line_pt
                });
            }
        }
    }

    fn get_red_pt_kind (prev_pt: Point, pt: Point, next_pt: Point) -> RedPointKind {
        let Point(prev_x, prev_y) = prev_pt;
        let Point(curr_x, curr_y) = pt;
        let Point(next_x, next_y) = next_pt;

        let prev_match_x = prev_x == curr_x;
        let prev_match_y = prev_y == curr_y;

        let next_match_x = next_x == curr_x;
        let next_match_y = next_y == curr_y;

        assert!((prev_match_x || prev_match_y) && !(prev_match_x && prev_match_y), "Either the x or the y of the previous point MUST match with the current point");
        if prev_match_x {
            assert!(!next_match_x, "Cannot be going in a straight line for two points in a row");

            if prev_y > curr_y {
                if next_x > curr_x {
                    RedPointKind::TopLeft
                }
                else {
                    RedPointKind::TopRight
                }
            }
            else {
                if next_x > curr_x {
                    RedPointKind::BottomLeft
                }
                else {
                    RedPointKind::BottomRight
                }
            }
        }
        else {
            assert!(!next_match_y, "Cannot be going in a straight line for two points in a row");
            
            if prev_x > curr_x {
                if next_y > curr_y {
                    RedPointKind::TopLeft
                }
                else {
                    RedPointKind::BottomLeft
                }
            }
            else {
                if next_y > curr_y {
                    RedPointKind::TopRight
                }
                else {
                    RedPointKind::BottomRight
                }
            }
        }
    }


    fn shape_fill (bounds: &Bounds, red_pts: &HashSet<Point>, green_points: &HashSet<Point>, void_points: &mut HashSet<Point>) {
        let Bounds { min_x, max_x, min_y, max_y  } = &bounds;

        let original_pt = Point(*min_x - 1, *min_y - 1);
        let mut pts_q = vec![ original_pt ];
        loop {
            let pt = if let Some(pt) = pts_q.pop() { pt }
            else {
                return;
            };

            for xdiff in [ -1, 0, 1 ] {
                for ydiff in [ -1, 0, 1] {
                    if xdiff == ydiff && xdiff == 0 {
                        continue;
                    }

                    let neighbor = Point((pt.0 as i32 + xdiff) as usize, (pt.1 as i32 + ydiff) as usize);
                    if red_pts.contains(&neighbor) || green_points.contains(&neighbor) || void_points.contains(&neighbor) {
                        continue;
                    }

                    if neighbor.0 < (*min_x - 1) || neighbor.0 > (*max_x + 1) || neighbor.1 < (*min_y - 1) || neighbor.1 > (*max_y + 1) {
                        continue;
                    }
    
                    void_points.insert(neighbor);
                    pts_q.push(neighbor);
                }
            }
        }
    }

    fn from (floor: Floor) -> Self {
        let Floor { points} = floor;

        let mut points_iter = points.iter();
        let first_pt = *points_iter.next().unwrap();
        let mut prev_pt = first_pt;
        
        let mut green_pts = HashSet::<Point>::new();
        let mut green_line_pts = HashMap::<Point, GreenPoint>::new();

        let mut points_set: HashSet<Point> = HashSet::from([ first_pt ]);
        for pt in points_iter {
            points_set.insert(*pt);
            ChristmasFloor::connect_points(prev_pt, pt, &mut green_pts, &mut green_line_pts);
            prev_pt = *pt;
        }
        ChristmasFloor::connect_points(prev_pt, &first_pt, &mut green_pts, &mut green_line_pts);

        let mut red_pt_kinds = HashMap::new();
        for pt_idx in 0..points.len() {
            let prev = if pt_idx == 0 {
                points[points.len() - 1]
            }
            else {
                points[pt_idx - 1]
            };

            let next = if pt_idx == points.len() - 1 {
                points[0]
            }
            else {
                points[pt_idx + 1]
            };

            let pt = points[pt_idx];
            let pt_kind = ChristmasFloor::get_red_pt_kind(prev, pt, next);
            red_pt_kinds.insert(pt, pt_kind);
        }

        let mut void = HashSet::new();

        let bounds = Bounds::from(&points_set);
        // let middle = Point((bounds.max_x - bounds.min_x) / 2 + 1, (bounds.max_y - bounds.min_y) / 2 + 1);
        // ChristmasFloor::shape_fill(&bounds, &points_set, &green_pts, &mut void);
        
        ChristmasFloor { 
            points, 
            points_set,
            green_points_set: green_pts,
            solution: None,
            red_tile_kinds: red_pt_kinds,
            void: void,
            // x_ascending: x_ascending_vec,
            // y_ascending: y_ascending_vec,
            // x_ascending_map, 
            // y_ascending_map,
        }
    }


}

impl std::fmt::Display for ChristmasFloor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let Bounds { min_x, max_x, min_y, max_y  } = Bounds::from(&self.points_set);

        
        if !(max_x < 100 && min_y < 100) {
            write!(f, "Too big :)")
        }
        else {
            for y in (min_y - 1)..(max_y + 2) {
                for x in (min_x - 1)..(max_x + 2) {
                    let pt = Point(x, y);
                    
                    let sol_pt = if let Some((Point(sol1_x, sol1_y), Point(sol2_x, sol2_y))) = self.solution {
                        let max_x = sol1_x.max(sol2_x);
                        let max_y = sol1_y.max(sol2_y);

                        let min_x = sol1_x.min(sol2_x);
                        let min_y = sol1_y.min(sol2_y);

                        pt.0 >= min_x && pt.0 <= max_x && pt.1 >= min_y && pt.1 <= max_y 
                    }
                    else {
                        false
                    };
                    
                    if self.points_set.contains(&pt) {
                        let pt_kind = self.red_tile_kinds.get(&pt).expect("At the disco");
                        match pt_kind {
                            RedPointKind::TopLeft => write!(f, "{}", "#".cyan())?,
                            RedPointKind::BottomLeft => write!(f, "{}", "#".magenta())?,
                            RedPointKind::TopRight => write!(f, "{}", "#".yellow())?,
                            RedPointKind::BottomRight => write!(f, "{}", "#".red())?,
                        }
                    }
                    else if self.green_points_set.contains(&pt) {
                        write!(f, "{}", "O".green())?
                    }
                    else if sol_pt {
                        write!(f, "{}", "s".cyan())?
                    }
                    // else if self.void.contains(&pt) {
                    //     write!(f, "{}", ".".bg::<Black>())?
                    // }
                    else {
                        write!(f, "{}", ".".bg::<Black>())?
                        // write!(f, "{}", ".".bg::<BrightWhite>())?
                    }
                }
                writeln!(f, "")?

            }

            writeln!(f, "")?;
            write!(f, "{} = TopLeft, ", "#".bg::<Cyan>())?;
            write!(f, "{} = BottomLeft, ", "#".bg::<Magenta>())?;
            write!(f, "{} = TopRight, ", "#".bg::<Yellow>())?;
            write!(f, "{} = BottomRight", "#".bg::<Red>())?;
            writeln!(f, "")
        }
    }
}

pub fn star_two (input: String) -> String {
    let floor = Floor::from(input);
    let mut christmas_floor = ChristmasFloor::from(floor);
    // println!("{christmas_floor}");

    let mut max_areas: Vec<usize> = vec![];
    let mut possibles: Vec<usize> = vec![];
    let mut max_area = 0;
    let mut max_pts = None;
    let pairs = get_pairs(&christmas_floor.points);
    let pairs_len = pairs.len();
    for (idx, (Point(p1_x, p1_y), Point(p2_x, p2_y))) in pairs.iter().enumerate() {
        let max_x = *p1_x.max(p2_x);
        let max_y = *p1_y.max(p2_y);

        let min_x = *p1_x.min(p2_x);
        let min_y = *p1_y.min(p2_y);

        let top_left    = Point(min_x, min_y);
        let top_right     = Point(max_x, min_y);
        let bottom_left = Point(min_x, max_y);
        let bottom_right  = Point(max_x, max_y);
        let four_corners = HashSet::from([ top_right, top_left, bottom_right, bottom_left ]);

        /*
        Both # are corners of a VALID rectangle
        In this scenario the rect only has 2 red corners, but it should be counted
        I think it's also the biggest
              /=\
              | |
        /=====# |
        | .   . |
        | .   . |
        \=#.... |
          \=====/
        
        */
        // let green_corners = four_corners.difference(&christmas_floor.points_set).collect::<Vec<&Point>>();
        // if green_corners.len() != 1 {
        //     continue;
        // }

        // let green_corner = *green_corners[0];
        // if green_corner == top_left {
        //     // go up
        //     // go left
        // }
        // else if green_corner == top_right {
        //     // go up
        //     // go right
        // }
        // else if green_corner == bottom_left {
        //     // go down
        //     // go left
        // }
        // else if green_corner == bottom_right {
        //     // go down
        //     // go right
        // }


        let mut collide_count  = 0;

        let mut wrong_count = 0;
        let mut ok = true;

        let points = &christmas_floor.points;
        for red_pt_idx in 0..points.len() {

            let prev = if red_pt_idx == 0 {
                points[points.len() - 1]
            }
            else {
                points[red_pt_idx - 1]
            };

            let next = if red_pt_idx == points.len() - 1 {
                points[0]
            }
            else {
                points[red_pt_idx + 1]
            };

            let red_pt = points[red_pt_idx];

            let prev_pt_shares_a_line = four_corners.iter().find(| corner | prev.shares_a_line_with(*corner)).is_some();
            let red_pt_shares_a_line = four_corners.iter().find(| corner | red_pt.shares_a_line_with(*corner)).is_some();
            let next_pt_shares_a_line = four_corners.iter().find(| corner | next.shares_a_line_with(*corner)).is_some();

            // Check if the prev current and next red tiles and make a similar rectangle... or something line that.
            /*
            s = solution
            it chose 1 and 2
            should 1 and 2 be disqualified because 2-3-4-V (V for void tile) form a rectangle full contained within
                the rectangle 1 and 2 make?

            I think there is something there

            too sleepy for now

            ............
            ......#OOO#.
            ......O...O.
            .1OOOO#ss.O.
            .Osssssss.O.
            .4OOOOOO3.O.
            .sssssssO.O.
            .Gssssss2O#.
            ............
            
            
             */

            if !red_pt_shares_a_line && prev_pt_shares_a_line && next_pt_shares_a_line {
                if (p1_x, p1_y) == (&9,&5) && (p2_x, p2_y) == (&2,&3) || (p1_x, p1_y) == (&2,&3) && (p2_x, p2_y) == (&9,&5) {
                    println!("BROKEN BY: {:?}", red_pt);
                }
                ok = false;
                break;
            }

            // if four_corners.contains(red_pt) {
            //     let red_pt_kind = *christmas_floor.red_tile_kinds.get(red_pt).unwrap();
            //     if top_right == *red_pt {
            //         if RedPointKind::TopRight != red_pt_kind {
            //             wrong_count += 1;
            //             if wrong_count == 2 {
            //                 if (p1_x, p1_y) == (&9,&5) && (p2_x, p2_y) == (&2,&3) || (p1_x, p1_y) == (&2,&3) && (p2_x, p2_y) == (&9,&5) {
            //                     println!("BROKEN BY: {:?}", red_pt);
            //                 }
            //                 ok = false;
            //                 break;
            //             }
            //         }
            //     }
            //     if top_left == *red_pt {
            //         if RedPointKind::TopLeft != red_pt_kind {
            //             wrong_count += 1;
            //             if wrong_count == 2 {
            //                 if (p1_x, p1_y) == (&9,&5) && (p2_x, p2_y) == (&2,&3) || (p1_x, p1_y) == (&2,&3) && (p2_x, p2_y) == (&9,&5) {
            //                     println!("BROKEN BY: {:?}", red_pt);
            //                 }
            //                 ok = false;
            //                 break;
                            
            //             }
                        
            //         }
            //     }
            //     if bottom_right == *red_pt {
            //         if RedPointKind::BottomRight != red_pt_kind {
            //             wrong_count += 1;
            //             if wrong_count == 2 {
            //                 if (p1_x, p1_y) == (&9,&5) && (p2_x, p2_y) == (&2,&3) || (p1_x, p1_y) == (&2,&3) && (p2_x, p2_y) == (&9,&5) {
            //                     println!("BROKEN BY: {:?}", red_pt);
            //                 }
            //                 ok = false;
            //                 break;
                            
            //             }
                        
            //         }
            //     }
            //     if bottom_left == *red_pt {
            //         if RedPointKind::BottomLeft != red_pt_kind {
            //             wrong_count += 1;
            //             if wrong_count == 2 {
            //                 if (p1_x, p1_y) == (&9,&5) && (p2_x, p2_y) == (&2,&3) || (p1_x, p1_y) == (&2,&3) && (p2_x, p2_y) == (&9,&5) {
            //                     println!("BROKEN BY: {:?}", red_pt);
            //                 }
            //                 ok = false;
            //                 break;
                            
            //             }
                        
            //         }
            //     }
            // }

            
            // let collides = red_pt.0 >= min_x && red_pt.0 <= max_x && red_pt.1 >= min_y && red_pt.1 <= max_y;
            // if collides && four_corners.iter().find(| corner | red_pt.shares_a_line_with(*corner)).is_none() {
            //     collide_count += 1;
            // }

            // let red_corner_kinds = four_corners.iter()
            //     .filter(| corner | {
            //         christmas_floor.points_set.contains(*corner)
            //     })
            //     .map(| corner | {
            //         *christmas_floor.red_tile_kinds.get(corner).unwrap()
            //     })
            //     .collect::<Vec<RedPointKind>>();

            // for outer_ck_idx in 0..red_corner_kinds.len() {
            //     for inner_ck_idx in outer_ck_idx+1..red_corner_kinds.len() {
            //         let outer_ck = red_corner_kinds[outer_ck_idx];
            //         let inner_ck = red_corner_kinds[inner_ck_idx];
            //         if outer_ck == inner_ck {
            //             ok = false;
            //             break;
            //         }
            //     }
            // }

            // let collides = red_pt.0 >= min_x && red_pt.0 <= max_x && red_pt.1 >= min_y && red_pt.1 <= max_y;
            // if collides && !four_corners.contains(&red_pt) {
            //     if (p1_x, p1_y) == (&9,&5) && (p2_x, p2_y) == (&2,&3) || (p1_x, p1_y) == (&2,&3) && (p2_x, p2_y) == (&9,&5) {
            //         println!("BROKEN BY: {:?}", red_pt);
            //     }
            //     ok = false;
            //     break;
            // }
        }

        if ok && collide_count % 2 == 0 {
            let rect_x = (p1_x.max(p2_x) - p1_x.min(p2_x)) + 1;
            let rect_y = (p1_y.max(p2_y) - p1_y.min(p2_y)) + 1;
            
            let area = rect_x * rect_y;
            if area > 121344690 && area < 3651874776 {
                possibles.push(area);
            }

            if area > max_area {
                max_area = area;
                max_areas.push(max_area);
                max_pts = Some((Point(*p1_x, *p1_y), Point(*p2_x, *p2_y)));
            }
        }

        // 116146088 -- too low
        // 121344690 -- too low
        // 460606168 -- 
        // 4570245558
        // 3651874776 -- too high
        // 4570245558 -- too high
    }

    christmas_floor.solution = max_pts;
    println!("{christmas_floor}");

    possibles.sort();
    println!("{:?}", possibles);
    println!("{:?}", possibles.len());
    println!("{:?}", max_areas);
    println!("{:?}", max_pts);
    
    max_area.to_string()

    


}

