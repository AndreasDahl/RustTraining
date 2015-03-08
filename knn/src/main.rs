#![feature(core)]
#![feature(io)]
#![feature(fs)]
#![feature(collections)]

use std::num::Float;
use std::cmp::Ordering::Greater;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io;
use std::io::BufReader;
use std::fs::File;
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
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

fn highest_in_vec<T: PartialOrd>(vec: &[T]) -> Option<(&T, usize)> {
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

fn most_common<'a>(vec: &[&'a str]) -> Option<&'a str> {
    let mut counter : HashMap<&str, i32> = HashMap::new();
    // Build counter
    for e in vec {
        match counter.get(e) {
            Some(&n) => counter.insert(e, n + 1),
            None    => counter.insert(e, 1)
        };
    };
    // Find the key with the highest value
    let mut best : Option<(&str, i32)> = None;
    for (k, v) in counter {
        best = match best {
            Some((_, bv)) if v > bv => Some((k, v)),
            None => Some((k, v)),
            _ => best,
        }
    };
    // Format return value
    match best {
        Some((k, _))  => Some(k),
        None        => None
    }
}

fn mean_point(data: &[&Point]) -> Point {
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    for point in data {
        sum_x += point.x;
        sum_y += point.y;
    }
    Point { x: sum_x / data.len() as f32, y: sum_y / data.len() as f32 }
}

fn kmeans(data: &[Point], k: usize) -> Vec<Vec<&Point>> {
    // Pick initial centroids
    let mut cent : Vec<Point> = Vec::new();
    for i in 0..k {
        cent.push(data[i].clone());
    }
    // Initialize old_clusters
    let mut old_clusters : Vec<Vec<&Point>> = Vec::with_capacity(k);
    for _ in 0..k {
        old_clusters.push(Vec::new());
    }
    loop {
        // Initialize new_clusters
        let mut new_clusters : Vec<Vec<&Point>> = Vec::with_capacity(k);
        for _ in 0..k {
            new_clusters.push(Vec::new());
        }
        println!("Iteration");
        // Find best labels based on current centroids
        for p in data {
            let mut best_dist = std::f32::INFINITY;
            let mut best_label = 0;
            for i in 0..cent.len() {
                let tmp_dist = distance(p, &cent[i]);
                if tmp_dist < best_dist {
                    best_dist = tmp_dist;
                    best_label = i;
                }
            }
            new_clusters[best_label].push(p);
        }
        // Terminate of clusters are the same
        if old_clusters == new_clusters {
            return old_clusters;
        }

        // Otherwise prepare for next iteration
        old_clusters = new_clusters;
        
        // Calculate new centroids by finding the mean of each cluster.
        for i in 0..old_clusters.len() {
            cent[i] = mean_point(&old_clusters[i]);
        }
    }
}


fn knn<'a>(train: &'a[LabeledPoint], data: &[Point], k: usize) -> Vec<&'a str> {
    let mut ret = Vec::new();
    for dp in data {
        let mut distances = Vec::new();
        let mut tmp_labels : Vec<&str> = Vec::new();
        // Build vector of closest points
        for tp in train {
            let dist = distance( dp, &tp.point );
            if distances.len() < k {
                distances.push(dist);
                tmp_labels.push(tp.label.as_slice())
            } else {
                let (&v, i) = highest_in_vec(&distances).expect("This should not happen");
                if v > dist { 
                    distances[i] = dist;
                    tmp_labels[i] = tp.label.as_slice();
                };
            }
        }
        // Add best label to return vector
        let best_label = most_common(&tmp_labels).expect("Label were not found");
        ret.push(best_label);
    }
    ret
}

// TODO: Handle Parsing Error.
fn load_lpoints(path: &str) -> io::Result<Vec<LabeledPoint>> {
    let mut s = String::new();
    let mut points = Vec::new();
    let mut br = BufReader::new(try!(File::open(path)));

    br.read_to_string(&mut s);
    let lines = s.trim().split("\n");
    for line in lines {
        let tokens = line.trim().split(" ");
        let values : Vec<&str> = tokens.collect();
        let x = values[0].parse().ok().expect("Badly formatted file");
        let y = values[1].parse().ok().expect("Badly formatted file");
        let label = String::from_str(values[2]);
        let p = LabeledPoint { point : Point {x: x, y: y}, label: label };
        points.push(p);
    }
    Ok(points)
}

// TODO: Handle Parsing Error.
fn load_points(path: &str) -> io::Result<Vec<Point>> {
    let mut s = String::new();
    let mut points = Vec::new();
    let mut br = BufReader::new(try!(File::open(path)));

    br.read_to_string(&mut s);
    let lines = s.trim().split("\n");
    for line in lines {
        let tokens = line.trim().split(" ");
        let values : Vec<&str> = tokens.collect();
        let x = values[0].parse().ok().expect("Badly formatted file");
        let y = values[1].parse().ok().expect("Badly formatted file");
        let p = Point { x: x, y: y };
        points.push(p);
    }
    Ok(points)
}

fn main() {
    let train = load_lpoints("res/IrisTrain2014.dt")
        .ok().expect("Error Loading training data");
    let test = load_points("res/IrisTest2014.dt")
        .ok().expect("Error Loading test data");
    let res = knn(&train, &test, 3);

    /*
    println!("length: {}", res.len());
    for label in res {
        println!("Best label: {}", label);
    }
    */

    let res2 = kmeans(&test, 3);
    println!("length: {}", res2.len());
    for c in 0..res2.len() {
        println!("Cluster: {}", c);
        for point in &res2[c] {
            println!("Point: {}", point); 
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;
    use super::{Point, distance, highest_in_vec, most_common, knn, load_points,
    load_lpoints};

    #[test]
    fn test_distace() {
        assert_eq!(5.0, distance(&Point { x: 0.0, y: 0.0 }, &Point { x: 3.0, y: 4.0 } ));
    }

    #[test]
    #[should_fail(expected = "assertion failed")]
    fn test_distance_fail() {
        assert_eq!(5.0, distance(&Point { x: 0.0, y: 0.0 }, &Point { x: 4.0, y: 4.0 } ));
    }

    #[test]
    fn test_highest_in_vec() {
        let v = vec![0.5, 1.0, 3.0, 2.0];
        let (res_v, res_i) = highest_in_vec(&v).expect("Error");
        assert_eq!(3.0, *res_v);
        assert_eq!(2, res_i);                         
        assert!(1.0 != *res_v);
    }

    #[test]
    fn test_most_common() {
        let v = vec!["a", "b", "c", "a", "b", "a"];
        assert_eq!("a", most_common(&v).expect("Error"));
    }

    // Benchmarks

    #[bench]
    fn bench_highest_in_vec(b: &mut Bencher) {
        let v = vec![0.5, 1.0, 3.0, 2.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        b.iter(|| {highest_in_vec(&v)})
    }

    #[bench]
    fn bench_most_common(b: &mut Bencher) {
        let v = vec!["a", "b", "c", "a", "b", "a"];
        b.iter(|| {most_common(&v)});
    }

    #[bench]
    fn bench_load_points(b: &mut Bencher) {
         b.iter(|| {load_points("res/IrisTrain2014.dt")});
    }

    #[bench]
    fn bench_load_lpoints(b: &mut Bencher) {
         b.iter(|| {load_lpoints("res/IrisTrain2014.dt")});
    }
    #[bench]
    fn bench_knn(b: &mut Bencher) {
        let train = load_lpoints("res/IrisTrain2014.dt")
            .ok().expect("Error Loading training data");
        let test = load_points("res/IrisTest2014.dt")
            .ok().expect("Error Loading test data");
        b.iter(|| {knn(&train, &test, 3)});
    }
}

