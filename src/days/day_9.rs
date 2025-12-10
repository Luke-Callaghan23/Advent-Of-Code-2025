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
        for inner in 0..points.len() {
            if outer == inner {
                continue;
            }
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


#[derive(Debug)]
struct ChristmasFloor {
    points: Vec<Point>,
    points_set: HashSet<Point>,
    green_points_set: HashSet<Point>,
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


    fn shape_fill (original_pt: Point, bounds: &Bounds, red_pts: &HashSet<Point>, green_pts: &mut HashSet<Point>) {
        let Bounds { min_x, max_x, min_y, max_y  } = &bounds;

        let mut pts_q = vec![ original_pt ];
        loop {
            let pt = if let Some(pt) = pts_q.pop() { pt }
            else {
                return;
            };

            if pt.0 <= *min_x || pt.0 >= *max_x || pt.1 <= *min_y || pt.1 >= *max_y {
                panic!("At the disco");
            }

            for xdiff in [ -1, 0, 1 ] {
                for ydiff in [ -1, 0, 1] {
                    if xdiff == ydiff && xdiff == 0 {
                        continue;
                    }
    
                    let neighbor = Point((pt.0 as i32 + xdiff) as usize, (pt.1 as i32 + ydiff) as usize);
                    if red_pts.contains(&neighbor) || green_pts.contains(&neighbor) {
                        continue;
                    }
    
                    green_pts.insert(neighbor);
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

        let bounds = Bounds::from(&points_set);
        let middle = Point((bounds.max_x - bounds.min_x) / 2 + 1, (bounds.max_y - bounds.min_y) / 2 + 1);
        ChristmasFloor::shape_fill(middle, &bounds, &points_set, &mut green_pts);
        
        ChristmasFloor { 
            points, 
            points_set,
            green_points_set: green_pts,
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
            write!(f, "{:?}", self)
        }
        else {
            for y in (min_y - 1)..(max_y + 2) {
                for x in (min_x - 1)..(max_x + 2) {
                    let pt = Point(x, y);
                    if self.points_set.contains(&pt) {
                        write!(f, "{}", "#".red())?
                    }
                    else if self.green_points_set.contains(&pt) {
                        write!(f, "{}", "O".green())?
                    }
                    else {
                        write!(f, "{}", ".".bg::<Black>())?
                    }
                }
                writeln!(f, "")?
            }
            Ok(())
        }
    }
}

pub fn star_two (input: String) -> String {
    let floor = Floor::from(input);
    let christmas_floor = ChristmasFloor::from(floor);
    println!("{christmas_floor}");
    todo!()

    // let all_vertices = christmas_floor.green_points_set
    //     .union(&christmas_floor.points_set)
    //     .map(| pt | *pt)
    //     .collect::<HashSet<Point>>();

    // 
    // let mut max_area = 0;
    // for (Point(p1_x, p1_y), Point(p2_x, p2_y)) in christmas_floor.get_pairs() {
    //     let max_x = p1_x.max(p2_x);
    //     let max_y = p1_y.max(p2_y);

    //     let min_x = p1_x.min(p2_x);
    //     let min_y = p1_y.min(p2_y);

    //     let top_right    = Point(*min_x, *min_y);
    //     let top_left     = Point(*max_x, *min_y);
    //     let bottom_right = Point(*min_x, *max_y);
    //     let bottom_left  = Point(*max_x, *max_y);

    //     if all_vertices.contains(&top_right)
    //     && all_vertices.contains(&top_left)
    //     && all_vertices.contains(&bottom_right)
    //     && all_vertices.contains(&bottom_left) {
    //         let rect_x = (p1_x.max(p2_x) - p1_x.min(p2_x)) + 1;
    //         let rect_y = (p1_y.max(p2_y) - p1_y.min(p2_y)) + 1;
            
    //         let area = rect_x * rect_y;
    //         if area > max_area {
    //             max_area = area;
    //         }
    //     }
    // }
    // max_area.to_string()

    


}

