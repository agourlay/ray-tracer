use crate::camera::*;
use crate::color::Color;
use crate::light::Light;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::plane::Plane;
use crate::sphere::Sphere;
use crate::transformation::*;
use crate::tuple::*;
use crate::world::World;
use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4};
use std::io::Result;

pub fn demo() -> Result<()> {
    let floor = Plane::new(1);

    let middle_sphere = Sphere::new(4)
        .set_transform(Matrix::translation(-0.5, 1.0, 0.5))
        .set_material(Material::new(Color::make(0.1, 1.0, 0.5), 0.7, 0.3));

    let right_sphere = Sphere::new(5)
        .set_transform(
            Matrix::translation(1.5, 0.5, -0.5).multiply(&Matrix::scaling(0.5, 0.5, 0.5)),
        )
        .set_material(Material::new(Color::make(0.5, 1.0, 0.1), 0.7, 0.3));

    let left_sphere = Sphere::new(6)
        .set_transform(
            Matrix::translation(-1.5, 0.33, -0.45).multiply(&Matrix::scaling(0.33, 0.33, 0.33)),
        )
        .set_material(Material::new(Color::make(1.0, 0.8, 0.1), 0.7, 0.3));

    let light_position = point(-10.0, 10.0, -10.0);
    let light_color = Color::make(1.0, 1.0, 1.0);
    let light = Light::point_light(light_position, light_color);

    let world = World::empty()
        .set_light(light)
        .add_object(Box::new(floor))
        .add_object(Box::new(middle_sphere))
        .add_object(Box::new(right_sphere))
        .add_object(Box::new(left_sphere));

    let camera = Camera::new(10000, 5000, FRAC_PI_3).set_transform(view_transform(
        &point(0.0, 1.5, -5.0),
        &point(0.0, 1.0, 0.0),
        &vector(0.0, 1.0, 0.0),
    ));

    let canvas = camera.render(&world);
    canvas.save_file("demo-projection.ppm")
}
