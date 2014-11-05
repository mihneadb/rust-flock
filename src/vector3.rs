use std::num;

#[deriving(Show)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3 {
    pub fn magnitude(&self) -> f32 {
        let s: f32 = num::pow(self.x, 2) +
                     num::pow(self.y, 2) +
                     num::pow(self.z, 2);
        s.sqrt()
    }
}