mod vector3;

fn main() {
    let v = vector3::Vector3 { x: 1f32, y: 2f32, z: 3f32 };
    println!("Hello, world! {}", v);
    println!("Hello, world! {}", v.magnitude());
}
