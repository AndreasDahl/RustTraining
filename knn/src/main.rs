use std::num::Float; // TODO: probably deprecated
use std::cmp::Ordering;
use std::cmp::Ordering::{ Less, Greater, Equal };

struct Point {
    x: i32,
    y: i32,
}

struct LabeledPoint {
    point: Point,
    label: i32, // TODO: Generic
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

// Current only one-nearest-neighbour
fn knn(train: &[LabeledPoint], data: &[Point], k: u32) -> Vec<i32> {
    let mut labels = vec![];
    for dp in data {
        let mut distances = vec![]; 
        for tp in train {
            distances.push( (distance( dp, &tp.point ), tp.label ) )
        }
        let mut min_dist : Option<f32> = None;
        let mut best_label : Option<i32> = None;
        for (dist, l) in distances {
            match min_dist {
                Some(min_d) => {
                    match cmp(min_d, dist) {
                        Greater => {
                            min_dist = Some(dist);
                            best_label = Some(l);
                        }
                        _ => continue
                    }
                }
                None        => {
                    min_dist = Some(dist);
                    best_label = Some(l);
                }
            }
        }
        match best_label {
            Some(l) => labels.push(l),
            None    => panic!(),
        }
    }

    labels
    // Box::new([LabeledPoint { point: Point { x: test[0].x, y: 0 }, label: 0 }])
}

fn main() {
    print_point( Point { x: 0, y: 0 } );
    print_lpoint( LabeledPoint { point: Point { x: 0, y: 0 }, label: 0 } );

    let train = [ LabeledPoint { point: Point { x: 0, y: 0 }, label: 0 },
                  LabeledPoint { point: Point { x: 1, y: 1 }, label: 0 },
                  LabeledPoint { point: Point { x: 3, y: -3 }, label: 1 } ];
    let test = [ Point { x: 0, y: 0 },
                 Point { x: 3, y:-3 }];
    let res = knn(&train, &test, 1);

    for p in res {
        println!("res: {}", p);
    }

    println!("Hello, world!");
}

// TESTS --------------------------------

#[test]
fn test_distace() {
    assert_eq!(5.0, distance(Point { x: 0, y: 0 }, Point { x: 3, y: 4 } ));
}

#[test]
#[should_fail]
fn test_distance_fail() {
    assert_eq!(5.0, distance(Point { x: 0, y: 0 }, Point { x: 4, y: 4 } ));
}