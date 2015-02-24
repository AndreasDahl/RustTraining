use std::num::Float; // TODO: probably deprecated
use std::cmp::Ordering::Greater;
use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;

struct Point {
    x: f32,
    y: f32,
}

struct LabeledPoint {
    point: Point,
    label: String, // TODO: Generic
}

fn print_point(point: Point) {
    println!("({}, {})", point.x, point.y);
}

fn print_lpoint(lpoint: LabeledPoint) {
    println!("({}, {}): {}", lpoint.point.x, lpoint.point.y, lpoint.label);
}

fn distance(p1: &Point, p2: &Point) -> f32 {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    let fo = (dx * dx + dy * dy) as f32;
    fo.sqrt()
}

fn highest_in_vec<T: PartialOrd>(vec: &Vec<T>) -> Option<(&T, usize)> {
    let mut highest: Option<(&T, usize)> = None;
    for i in 0..vec.len() {
        match highest {
            None => highest = Some((&vec[i], i)),
            Some(h_tuple)    => {
                let (h, _) = h_tuple;
                match (&vec[i]).partial_cmp(h) {
                    Some(o) => match o {
                        Greater => highest = Some((&vec[i], i)),
                        _ => continue
                    },
                    None => continue
                }
            }
        }
    }
    highest
}

fn most_common(vec: &Vec<String>) -> Option<Box<String>> {
    let mut counter : HashMap<String, i32> = HashMap::new();
    // Build counter
    for e in vec {
        match counter.get(e) {
            Some(&n) => counter.insert(e.clone(), n + 1),
            None    => counter.insert(e.clone(), 1)
        };
    };
    // Find the key with the highest value
    let mut best : Option<(String, i32)> = None;
    for (k, v) in counter {
        match best {
            Some((_, bv)) => {
                if v > bv {
                    best = Some((k, v))
                }},
            None => best = Some((k, v))
        };
    };
    // Format return value
    match best {
        Some((k, _))  => Some(Box::new(k)),
        None        => None
    }
}

// Currently only one-nearest-neighbour
fn knn(train: &[LabeledPoint], data: &[Point], k: usize) -> Vec<Box<String>> {
    let mut ret = Vec::new();
    for dp in data {
        let mut distances = Vec::new();
        let mut tmp_labels = Vec::new();
        // Build vector of closest points
        for tp in train {
            let dist = distance( dp, &tp.point );
            if distances.len() < k {
                distances.push(dist);
                tmp_labels.push(tp.label.clone())
            } else {
                let (&v, i) = highest_in_vec(&distances).expect("This should not happen");
                if v > dist { 
                    distances[i] = dist;
                    tmp_labels[i] = tp.label.clone();
                };
            }
        }
        // Add best label to return vector
        let best_label = most_common(&tmp_labels).expect("Label were not found");
        println!("Best label: {}", best_label);
        ret.push(best_label);
    }
    ret
}

// TODO: better error handling.
fn load_lpoints(path: &str) -> Vec<LabeledPoint> {
    let mut f = File::open(path).ok().expect("Failed to open file");
    let mut s = String::new();
    let mut points = Vec::new();
    
    f.read_to_string(&mut s).ok().expect("Failed to read file to string");

    let lines = s.trim().split_str("\n");
    for line in lines {
        let tokens = line.trim().split_str(" ");
        let mut values = Vec::new();
        for t in tokens {
            values.push(t); // Collect?
        }
        let x = values[0].parse().ok().expect("Badly formatted file");
        let y = values[1].parse().ok().expect("Badly formatted file");
        let label = values[2];
        let p = LabeledPoint { point: Point { x: x, y: y }, label:
            String::from_str(label) };
        points.push(p);
    }
    points
}

// TODO: better error handling.
fn load_points(path: &str) -> Vec<Point> {
    let mut f = File::open(path).ok().expect("Failed to open file");
    let mut s = String::new();
    let mut points = Vec::new();
    
    f.read_to_string(&mut s).ok().expect("Failed to read file to string");

    let lines = s.trim().split_str("\n");
    for line in lines {
        let tokens = line.trim().split_str(" ");
        let mut values = Vec::new();
        for t in tokens { // collect?
            values.push(t);
        }
        let x = values[0].parse().ok().expect("Badly formatted file");
        let y = values[1].parse().ok().expect("Badly formatted file");
        let p = Point { x: x, y: y };
        points.push(p);
    }
    points
}

fn main() {
    let train = load_lpoints("res/IrisTrain2014.dt");
    let test = load_points("res/IrisTest2014.dt");
    let res = knn(&train, &test, 3);

    println!("length: {}", res.len());
}

// TESTS --------------------------------

#[test]
fn test_distace() {
    assert_eq!(5.0, distance(&Point { x: 0, y: 0 }, &Point { x: 3, y: 4 } ));
}

#[test]
#[should_fail]
fn test_distance_fail() {
    assert_eq!(5.0, distance(&Point { x: 0, y: 0 }, &Point { x: 4, y: 4 } ));
}

#[test]
fn test_highest_in_vec() {
    let v = vec![0.5, 1.0, 3.0, 2.0];
    let (res_v, res_i) = highest_in_vec(&v).expect("Error"); 
    assert_eq!(3.0, *res_v);
    assert_eq!(2, res_i);
    assert!(1.0 != *res_v);
}

