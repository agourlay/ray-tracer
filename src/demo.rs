use crate::camera::*;
use crate::color::*;
use crate::light::Light;
use crate::material::Material;
use crate::matrix::{Matrix, Transformation};
use crate::pattern::Pattern;
use crate::plane::Plane;
use crate::sphere::Sphere;
use crate::transformation::*;
use crate::tuple::*;
use crate::world::World;
use std::f64::consts::*;
use std::io::Result;

pub fn demo() -> Result<()> {
    let checker = Pattern::new_checker(WHITE, BLACK, Matrix::rotate_y(FRAC_PI_4));
    let floor = Plane::new(1).set_material(Material::default().set_pattern(checker));

    let stripe = Pattern::new_stripe(
        BLUE,
        RED,
        Matrix::scaling(0.1, 1.0, 1.0).multiply(&Matrix::rotate_z(FRAC_PI_4)),
    );
    let middle_sphere = Sphere::new(4)
        .set_transform(Matrix::translation(-0.5, 1.0, 0.5))
        .set_material(Material::new_with_pattern(
            Color::make(0.1, 1.0, 0.5),
            0.7,
            0.3,
            stripe,
        ));

    let gradient = Pattern::new_gradient(
        AQUA,
        GREEN,
        Matrix::scaling(0.9, 1., 1.).multiply(&Matrix::rotate_y(FRAC_PI_4)),
    );
    let right_sphere = Sphere::new(5)
        .set_transform(
            Matrix::translation(1.5, 0.5, -0.5).multiply(&Matrix::scaling(0.5, 0.5, 0.5)),
        )
        .set_material(Material::new_with_pattern(
            Color::make(0.5, 1.0, 0.1),
            0.7,
            0.3,
            gradient,
        ));

    let ring = Pattern::new_ring(
        YELLOW,
        FUCHSIA,
        Matrix::scaling(0.3, 0.3, 0.3).multiply(&Matrix::rotate_x(FRAC_PI_2)),
    );
    let left_sphere = Sphere::new(6)
        .set_transform(
            Matrix::translation(-1.5, 0.33, -0.45).multiply(&Matrix::scaling(0.33, 0.33, 0.33)),
        )
        .set_material(Material::new_with_pattern(Color::make(1.0, 0.8, 0.1), 0.7, 0.3, ring));

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
