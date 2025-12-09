use std::{cmp::Ordering, collections::{HashSet}, fmt::Debug};

fn eat_number_skip_charset <'a, 'b> (line: &'a str, skip_charset: &'b str) -> (Option<usize>, &'a str) {
    let mut num: Option<usize> = None;
    let nonnum: Option<usize> = line.chars().enumerate().find_map(| (idx, chr) | {
        // Only if we haven't found a digit yet, we can skip the whitespace
        if skip_charset.contains(chr) && num.is_none() {
            return None;
        }
        if chr.is_numeric() {
            let digit = chr.to_digit(10).unwrap();
            num = Some(num.unwrap_or(0) * 10 + digit as usize);
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

#[derive(Debug, Copy, Clone)]
struct Point (usize, usize, usize);

impl Point {
    fn extract_point (line: &str) -> (Self, &str) {
        let skipme = ",\n\r\t ";
        let (first, line) = eat_number_skip_charset(line, skipme);
        let (second, line) = eat_number_skip_charset(line, skipme);
        let (third, line) = eat_number_skip_charset(line, skipme);

        if first.is_none() || second.is_none() || third.is_none() {
            panic!("Could not parse point from input line '{line}'");
        }
        
        let pt = Point (
            first.unwrap(),
            second.unwrap(),
            third.unwrap()
        );

        return (pt, line);
    }

    fn dist (&self, other: &Point) -> f64 {
        let Point(sx, sy, sz) = *self;
        let Point(ox, oy, oz) = *other;

        let sx = sx as f64;
        let sy = sy as f64;
        let sz = sz as f64;
        let ox = ox as f64;
        let oy = oy as f64;
        let oz = oz as f64;

        ((sx - ox).powi(2) + (sy - oy).powi(2) + (sz - oz).powi(2)).sqrt()
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
struct DistanceEntry (usize, usize, f64);

impl Eq for DistanceEntry {

}

impl Ord for DistanceEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.2.total_cmp(&other.2)
    }
}

// Simple priority queue in a vector min heap
#[derive(Debug, Clone)]
struct DistanceQueue <T: Ord + Copy + Clone + Debug> {
    queue: Vec<T>,
}

#[allow(unused)]
impl <T: Ord + Copy + Clone + Debug> DistanceQueue <T> {
    fn with_capacity (size: usize) -> Self {
        DistanceQueue { queue: Vec::with_capacity(size) }
    }

    fn parent_idx (&self, idx: usize) -> usize {
        if idx == 0 { 0 }
        else {
            (idx - 1) / 2
        }
    }

    fn children_idxs (&self, idx: usize) -> (usize, usize) {
        (idx * 2 + 1, idx * 2 + 2)
    }

    fn swap_idx (&mut self, idx_1: usize, idx_2: usize) {
        let tmp = self.queue[idx_1];
        self.queue[idx_1] = self.queue[idx_2];
        self.queue[idx_2] = tmp;
    }

    fn enqueue (&mut self, entry: T) -> usize {
        // First, just push to the end
        let mut entry_idx = self.queue.len();
        self.queue.push(entry);
        
        // Compare the inserted entry against its immediate parent and if it is 
        //      smaller than the parent, then swap those elements
        // Repeat until the immediate parent is smaller, or the inserted element is the root
        let mut parent_idx = self.parent_idx(entry_idx);
        while self.queue[parent_idx].cmp(&entry).is_gt() {
            self.swap_idx(parent_idx, entry_idx);
            entry_idx = parent_idx;
            parent_idx = self.parent_idx(parent_idx);
        }
        entry_idx
    }

    fn peek (&self) -> Option<&T> {
        if self.queue.len() == 0 {
            None
        }
        else {
            Some(&self.queue[0])
        }
    }

    fn confirm_tree (&self, idx: usize, depth: usize) -> bool {
        if idx >= self.queue.len() {
            return true;
        }
        
        let (left, right) = self.children_idxs(idx);
        let cmp = | idx: usize, other: usize | -> Ordering {
            match (idx < self.queue.len(), other < self.queue.len()) {
                (true, true) => self.queue[idx].cmp(&self.queue[other]),
                (true, false) => Ordering::Less,
                (false, true) => todo!(),
                (false, false) => todo!(),
            }
        };

        let get = | idx: usize | -> Option<T> {
            if idx < self.queue.len() {
                Some(self.queue[idx])
            }
            else {
                None
            }
        };
        
        let c1 = cmp(idx, left).is_le();
        let c2 = cmp(idx, right).is_le();
        let c3 = self.confirm_tree(left, depth+1);
        let c4 = self.confirm_tree(right, depth+1);
        
        let confirm = c1 && c2 && c3 && c4;

        if !confirm {
            println!("self.queue[{idx}] ({:?}) <= self.queue[{left}] ({:?}): {}", get(idx), get(left), c1);
            println!("self.queue[{idx}] ({:?}) <= self.queue[{right}] ({:?}): {}", get(idx), get(right), c2);
            println!("self.confirm_tree({left}):  {}", self.confirm_tree(left, depth+1));
            println!("self.confirm_tree({right}): {}", self.confirm_tree(right, depth+1));
        }

        assert!(
            confirm, "^^^"
        );
        confirm
    }

    fn dequeue (&mut self) -> Option<T> {
        let queue_size = self.queue.len();
        if queue_size == 0 {
            return None;
        }

        // Get the number we are about to dequeue
        // Because of the way the min heap is implemented, we cannot just unshift this element from the front
        //      instead we have to do a sifting process, moving this element down the tree until we cannot move
        //      any more lower
        // Swapping this element with the smallest child element at each point in the min heap until it 
        //      reaches the bottom will effectively re-justify the min heap
        let deq = Some(self.queue[0]);
        self.swap_idx(0, self.queue.len() - 1);
        let popped = self.queue.pop();

        assert!(popped == deq, "Popped element must be the element we are dequeue-ing");
        
        let queue_size = self.queue.len();
        if queue_size == 0 {
            return deq;
        }
        
        let mut idx = 0;
        loop {
            let current = self.queue[idx];

            let (left_idx, right_idx) = self.children_idxs(idx);

            if left_idx < queue_size && right_idx < queue_size {
                let left_elt = self.queue[left_idx];
                let right_elt = self.queue[right_idx];

                // CASE: BOTH CHILDREN EXIST
                //      Find the smallest child and replace `current` with that child
                if left_elt.cmp(&current).is_lt() && right_elt.cmp(&current).is_lt() {
                    // CASE: BOTH CHILDREN EXIST AND ARE SMALLER
                    //      Replace the smallest child with `current`
                    if left_elt.cmp(&right_elt).is_lt() {
                        self.swap_idx(idx, left_idx);
                        idx = left_idx;
                    }
                    else {
                        self.swap_idx(idx, right_idx);
                        idx = right_idx;
                    }
                }
                // CASE: ONLY LEFT IS SMALLER THAN CURRENT
                //      Replace left with `current`
                else if left_elt.cmp(&current).is_lt() {
                    self.swap_idx(idx, left_idx);
                    idx = left_idx;
                }
                // CASE: ONLY RIGHT IS SMALLER THAN CURRENT
                //      Replace right with `current`
                else if right_elt.cmp(&current).is_lt() {
                    self.swap_idx(idx, right_idx);
                    idx = right_idx;
                }
                // CASE: NEITHER CHILD IS SMALLER
                //      Break out of the loop, we've found the correct spot for `current`
                else {
                    break;
                }
            }
            else if left_idx < queue_size {
                // CASE: ONLY LEFT EXISTS
                //      If left is smaller than `current`, swap them
                let left_elt = self.queue[left_idx];
                if left_elt.cmp(&current).is_lt() {
                    self.swap_idx(idx, left_idx);
                    idx = left_idx;
                }
                // CASE: ONLY LEFT EXISTS, BUT IS NOT SMALLER
                //      We've found the correct spot for `current`, return
                else {
                    break;
                }
            }
            else if right_idx < queue_size {
                // CASE: ONLY RIGHT EXISTS
                //      If right is smaller than `current`, swap them
                let right_elt = self.queue[right_idx];
                if right_elt.cmp(&current).is_lt() {
                    self.swap_idx(idx, right_idx);
                    idx = right_idx;
                }
                // CASE: ONLY RIGHT EXISTS, BUT IS NOT SMALLER
                //      We've found the correct spot for `current`, return
                else {
                    break;
                }
            }
            else {
                // CASE: NEITHER CHILD EXISTS
                //      `current` cannot be sifted down anymore, return
                break;
            }

        }
        deq
    }
}


#[derive(Debug)]
enum CircuitState {
    Alive,
    Dead
}

#[derive(Debug)]
#[allow(unused)]
struct Circuit {
    state: CircuitState, 
    id: usize,
    point_indexes: HashSet<usize>,
    connections: usize,
}

type CircuitIndex = usize;

#[allow(unused)]
struct Graph {
    points: Vec<Point>,

    // Indexed by index of p1, then index of p2, result is distance between p1 and p2
    // distance_matrix[idx_p1][pdx_p2] = d(p1, p2)
    distance_matrix: Vec<Vec<f64>>,

    ordered_distances: DistanceQueue<DistanceEntry>,

    circuits: Vec<Circuit>,
    circuits_map: Vec<CircuitIndex>
}

impl Graph {
    fn from (input: String) -> Self {
        let mut points: Vec<Point> = Vec::new();
        
        let mut input = &input[..];
        while input.len() > 0 {
            let (point, next_input) = Point::extract_point(input);
            points.push(point);
            input = next_input;
        }

        let point_count = points.len();
        let mut dqueue = DistanceQueue::<DistanceEntry>::with_capacity(
            point_count * point_count
        );

        let mut distance_matrix: Vec<Vec<f64>> = Vec::with_capacity(point_count);
        for p1_idx in 0..point_count {
            let p1 = &points[p1_idx];
            let mut p1_distances: Vec<f64> = Vec::with_capacity(point_count);
            for p2_idx in 0..point_count {
                if p1_idx == p2_idx {
                    continue;
                }

                let p2 = &points[p2_idx];
                let dist = p1.dist(p2);
                p1_distances.push(dist);

                // By checking that the second point does not already have a vector in the distance
                //      matrix, we prevent duplicate inserts into the priority queue 
                // (If p2_idx > distance_matrix, then we haven't gotten to that point in the outer loop yet,
                //      so there's no way we could have inserted (p2, p1, dist) (which would be a duplicate of 
                //      (p1, p2, dist) that we'd insert below))
                if p2_idx > distance_matrix.len() {
                    dqueue.enqueue(DistanceEntry(p1_idx, p2_idx, dist));
                }
            }
            distance_matrix.push(p1_distances);
        }

        let mut circuits: Vec<Circuit> = Vec::new();
        let mut circuits_map: Vec<CircuitIndex> = Vec::new();
        for pidx in 0..points.len() {
            circuits.push(Circuit { 
                state: CircuitState::Alive, 
                id: pidx, 
                point_indexes: HashSet::from([ pidx ]), 
                connections: 0 
            });
            circuits_map.push(pidx);
        }

        Graph { points, distance_matrix, circuits, circuits_map, ordered_distances: dqueue }
    }

    fn _print_circuits (&self, threshold: usize) {
        
        let mut c = self.circuits.iter().filter(| circ | {
            if let CircuitState::Alive = circ.state {
                true
            }
            else {
                false
            }
        }).collect::<Vec<&Circuit>>();
        c.sort_by(| a, b | b.connections.cmp(&a.connections));

        for circ in &c {
            if circ.connections < threshold {
                break;
            }
            print!("Circuit (cxn={}) [{}] [", circ.connections, if let CircuitState::Alive = circ.state {
                "ALIVE"
            } else {
                "DEAD"
            });
            for point in &circ.point_indexes {
                print!("{}, ", point);
            }
            println!("]");
        }
    }

    fn add_connection (&mut self) -> Option<(usize, usize)> {
        let connection = self.ordered_distances.dequeue();
        let DistanceEntry( p1_idx, p2_idx, _distance ) = if let Some(connection) = connection {
            connection
        }
        else { 
            return None;
        };

        let c1_idx = self.circuits_map[p1_idx];
        let c2_idx = self.circuits_map[p2_idx];

        if c1_idx == c2_idx {
            self.circuits.get_mut(c1_idx).unwrap().connections += 1;
            return Some((p1_idx, p2_idx));
        }

        let (c2pt_idxs, c2_cxns) = {
            let c2 = self.circuits.get_mut(c2_idx).unwrap();
            c2.state = CircuitState::Dead;
            let connections = c2.connections.clone();
            c2.connections = 0;
            (c2.point_indexes.drain().collect::<Vec<usize>>(), connections)
        };

        {
            let c1 = self.circuits.get_mut(c1_idx).unwrap();
            for c2pt_idx in &c2pt_idxs {
                c1.point_indexes.insert(*c2pt_idx);
            }
            c1.connections += c2_cxns;
            c1.connections += 1;
        }

        // 115197
        // 049464

        for c2pt_idx in c2pt_idxs {
            self.circuits_map[c2pt_idx] = c1_idx;
        }

        return Some((p1_idx, p2_idx));

    }
}


pub fn star_one (input: String) -> String {

    let mut graph = Graph::from(input);
    loop {
        graph.add_connection();
        
        // Add all the connections of the living circuits
        let cxns = graph.circuits.iter().filter(| circ | {
            if let CircuitState::Alive = circ.state {
                true
            }
            else {
                false
            }
        })
        .map(| circ | {
            circ.connections
        })
        .sum::<usize>();

        // Example only has 20 nodes, and also only wants you to stop at the 
        //      10th connection
        if graph.points.len() == 20 {
            if cxns == 10 {
                break;
            }
        }
        // Real input has 1000 notes and stops at 1000 connections
        else {
            if cxns == 1000 {
                break;
            }
        }

    }

    // Sort the circuits by the length of the points in the circuit (descending)
    let mut circs = graph.circuits.iter().filter_map(| circ | {
        if let CircuitState::Alive = circ.state {
            Some(circ.point_indexes.len())
        }
        else {
            None
        }
    }).collect::<Vec<usize>>();
    circs.sort_by(| a, b | b.cmp(&a));

    // Take the top 3 circuits and multiply them together
    let product = circs.iter()
        .take(3)
        .product::<usize>();

    product.to_string()
}

pub fn star_two (input: String) -> String {
    
    let mut graph = Graph::from(input);
    loop {
        let latest_connection = graph.add_connection();
        if latest_connection.is_none() {
            panic!("Couldn't find another connection!");
        }

        // Get the count of living circuits
        let living_circuits = graph.circuits.iter().filter(| circ | {
            if let CircuitState::Alive = circ.state {
                true
            }
            else {
                false
            }
        }).count();

        // If there is only one living circuit, then take the recently joined pair of points
        //      and multiply their x-coords for the result
        if living_circuits == 1 {
            let (p1_idx, p2_idx) = latest_connection.unwrap();
            let Point(p1_x, _p1_y, _p1_z) = graph.points[p1_idx];
            let Point(p2_x, _p2_y, _p2_z) = graph.points[p2_idx];
            return (p1_x * p2_x).to_string();
        }
    }
}

