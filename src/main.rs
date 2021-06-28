mod camera;
mod canvas;
mod color;
mod demo;
mod epsilon;
mod intersection;
mod light;
mod material;
mod matrix;
mod pattern;
mod plane;
mod projectile;
mod ray;
mod shape;
mod sphere;
mod transformation;
mod tuple;
mod world;

use std::io::Result;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

fn main() -> Result<()> {
    use crate::demo::*;
    demo()
}
