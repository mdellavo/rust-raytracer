use vect::Vec3;

#[derive(RustcDecodable, RustcEncodable)]
struct Light {
     position: Vec3,
}

struct Surface;

#[derive(RustcDecodable, RustcEncodable)]
struct Object {
     position: Vec3
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Scene {
     objects: Vec<Object>,
     lights: Vec<Light>
}

pub struct Ray;


#[test]
fn test() {
}
