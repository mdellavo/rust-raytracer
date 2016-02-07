use std::ops::{Add, Sub, AddAssign, SubAssign};

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

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other : Vec3) -> Vec3 {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Vec3 {}

#[test]
fn test() {
    let mut a = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
    let mut b = Vec3 { x: 4.0, y: 5.0, z: 6.0 };

    assert!(a != b);
    assert!(a == a);

    assert_eq!(a + b, Vec3 { x: 5.0, y: 7.0, z: 9.0 });
    assert_eq!(b - a, Vec3 { x: 3.0, y: 3.0, z: 3.0 });

    let c = Vec3 {x: 1.0, y: 1.0, z: 1.0 };

    a += c;
    assert_eq!(a, Vec3 { x: 2.0, y: 3.0, z: 4.0 });

    b -= c;
    assert_eq!(b, Vec3 { x: 3.0, y: 4.0, z: 5.0 });
}
