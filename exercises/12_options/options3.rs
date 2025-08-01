#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x:200, y:300 });
    // let optional_point :Option<Point> = None;

    // TODO: Fix the compiler error by adding something to this match statement.
    match optional_point {
        Some(ref p) => println!("Co-ordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
    }

    println!("{optional_point:?}"); // Don't change this line.
}
