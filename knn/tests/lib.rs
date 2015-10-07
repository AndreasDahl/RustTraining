extern crate knn;

use knn::*;

#[test]
fn test_distace() {
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 3.0, y: 4.0 };
    assert_eq!(5.0, p1.distance(&p2));
    assert_eq!(5.0, p2.distance(&p1));
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_distance_fail() {
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 4.0, y: 4.0 };
    assert_eq!(5.0, p1.distance(&p2));
    assert_eq!(5.0, p2.distance(&p1));
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

#[test]
fn test_zero_one_error() {
    let expected = vec!["a", "a", "b", "b", "c"];
    let actual   = vec!["a", "b", "b", "b", "c"];

    assert_eq!(0.2, zero_one_error(&expected, &actual));
}

// Benchmarks
//
// #[bench]
// fn bench_highest_in_vec(b: &mut Bencher) {
//     let v = vec![0.5, 1.0, 3.0, 2.0, 4.0, 5.0, 6.0, 7.0, 8.0];
//     b.iter(|| {highest_in_vec(&v)})
// }
//
// #[bench]
// fn bench_most_common(b: &mut Bencher) {
//     let v = vec!["a", "b", "c", "a", "b", "a"];
//     b.iter(|| {most_common(&v)});
// }
//
// #[bench]
// fn bench_load_points(b: &mut Bencher) {
//      b.iter(|| {load_points("res/IrisTrain2014.dt")});
// }
//
// #[bench]
// fn bench_load_lpoints(b: &mut Bencher) {
//      b.iter(|| {load_lpoints("res/IrisTrain2014.dt")});
// }
//
// #[bench]
// fn bench_knn(b: &mut Bencher) {
//     let train = load_lpoints("res/IrisTrain2014.dt")
//         .ok().expect("Error Loading training data");
//     let test = load_points("res/IrisTest2014.dt")
//         .ok().expect("Error Loading test data");
//     b.iter(|| {knn(&train, &test, 3)});
// }
//
// #[bench]
// fn bench_kmeans(b: &mut Bencher) {
//     let test = load_points("res/IrisTest2014.dt")
//         .ok().expect("Error Loading test data");
//     b.iter(|| {kmeans(&test, 3)});
// }
