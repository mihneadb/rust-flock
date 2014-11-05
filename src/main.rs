mod vector3;

fn main() {
    let v = vector3::Vector3 { x: 1f32, y: 2f32, z: 3f32 };
    let w = vector3::Vector3 { x: 3f32, y: 2f32, z: 1f32 };
    println!("{}", v.magnitude());
    println!("{}", v + w);
}
