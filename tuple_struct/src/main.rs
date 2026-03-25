struct Color(u8, u8, u8);//(Red, Green, Blue)
struct Point(u8, u8, u8);//(x, y, z)

fn get_y(p: Point) -> u8 {
    p.1
}

fn main() {
    let red = Color(255, 0, 0);
    println!("First value {}", red.0);

    let coord = Point(4, 5, 6);
    let y = get_y(coord );
    println!("Second point is {y}")
}
