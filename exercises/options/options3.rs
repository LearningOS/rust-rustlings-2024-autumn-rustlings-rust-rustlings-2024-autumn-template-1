// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


#[derive(Debug)] // 这里添加了 Debug trait 的派生
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    
    // 现在可以打印 y，因为 Point 实现了 Debug trait
    println!("y is still available: {:?}", y);
}
