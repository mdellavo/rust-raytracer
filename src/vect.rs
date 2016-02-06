use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Vec3 {
     x: f64,
     y: f64,
     z: f64,
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other : Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other : Vec3) -> Vec3 {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Vec3 {}

#[test]
fn test_vec3_ops() {
    let a = Vec3 { x: 1.0, y: 2.0, z: 3.0};
    let b = Vec3 { x: 4.0, y: 5.0, z: 6.0};

    assert!(a != b);
    assert!(a == a);
    assert_eq!(a + b, Vec3 { x: 5.0, y: 7.0, z: 9.0 });
    assert_eq!(b - a, Vec3 { x: 3.0, y: 3.0, z: 3.0 });

}
