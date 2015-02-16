use std::num::Float; // TODO: probably deprecated
use std::cmp::Ordering;
use std::cmp::Ordering::{ Less, Greater, Equal };
use std::collections::HashMap;

struct Point {
    x: i32,
    y: i32,
}

struct LabeledPoint<'a> {
    point: Point,
    label: &'a str, // TODO: Generic
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

fn cmp( a: f32, b: f32 ) -> Ordering {
    if a < b { Less }
    else if a > b { Greater }
    else { Equal }
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

fn most_common<'a>(vec: &Vec<&'a str>) -> Option<&'a str> {
    let mut hist: HashMap<&str, u32> = HashMap::new();

    for e in vec.iter() {
        match hist.get(e) {
            Some(&n) => hist.insert(e, n + 1),
            None    => hist.insert(e, 1)
        };
    };

    let mut best = None;
    for (k, v) in hist.iter() {
        match best {
            Some((_, bv)) => {
                if v > bv {
                    best = Some((k, v))
                }},
            None => best = Some((k, v))
        };
    };
    
    match best {
        Some((k, _))  => Some(&k),
        None        => None
    }
}

// Currently only one-nearest-neighbour
fn knn<'a>(train: &'a[LabeledPoint], data: &[Point], k: usize) -> Vec<&'a str> {
    let mut ret = Vec::new();
    for dp in data {
        let mut distances = Vec::new();
        let mut tmp_labels = Vec::new();
        // Build vector of closest points
        for tp in train {
            let dist = distance( dp, &tp.point );
            if distances.len() < k {
                distances.push(dist);
                tmp_labels.push(tp.label)
            } else {
                let (&v, i) = highest_in_vec(&distances).expect("This should not happen");
                if v > dist { 
                    distances[i] = dist;
                    tmp_labels[i] = tp.label;
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

fn main() {
    print_point( Point { x: 0, y: 0 } );
    print_lpoint( LabeledPoint { point: Point { x: 0, y: 0 }, label: "0" } );

    let train = [ LabeledPoint { point: Point { x: 0, y: 0 }, label: "0" },
                  LabeledPoint { point: Point { x: 1, y: 1 }, label: "0" },
                  LabeledPoint { point: Point { x: 3, y: -3 }, label: "1" } ];
    let test = [ Point { x: 0, y: 0 },
                 Point { x: 3, y:-3 }];
    let res = knn(&train, &test, 3);

    println!("length: {}", res.len());
    for p in res {
        println!("res: {}", p);
    }
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

