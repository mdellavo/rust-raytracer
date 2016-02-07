#![feature(op_assign_traits)]
#![feature(augmented_assignments)]

use std::env;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

extern crate rustc_serialize;
use rustc_serialize::json;

mod scene;
mod vect;

use scene::Scene;

struct Color {
    r: f64,
    g: f64,
    b: f64,
}

fn load_scene(s: &String) -> Scene {
    let path = Path::new(s);
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), Error::description(&why)),
        Ok(_) => {},
    }

    let scene: Scene = json::decode(&s).unwrap();
    return scene;
}

fn render_scene(scene: &Scene) {
}

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in &args[1..] {
        println!("loading {}...", arg);
        let scene = load_scene(&arg);
        render_scene(&scene);
    }
}
