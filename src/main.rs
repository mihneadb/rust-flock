mod point;

fn main() {
    let v = point::Point { x: 1i, y: 2i, z: 3i };
    let w = point::Point { x: 3i, y: 2i, z: 1i };
    println!("{}", v + w);
}
