struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let data = [Point { x: 0, y: 0 },
                Point { x: 1, y: 1 }];

    for point in data.iter() {
        println!("The value of x is: ({}, {})", point.x, point.y);
    }
}
