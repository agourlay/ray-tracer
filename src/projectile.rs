use crate::canvas::Canvas;
use crate::color::Color;
use crate::tuple::*;
use std::io::Result;

#[derive(Debug)]
pub struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Env {
    gravity: Tuple,
    wind: Tuple,
}

impl Projectile {
    fn tick(self: Projectile, env: &Env) -> Projectile {
        let new_position = add_tuple(&self.position, &self.velocity);
        let env_induced_speed = add_tuple(&env.gravity, &env.wind);
        let new_speed = add_tuple(&self.velocity, &env_induced_speed);
        Projectile {
            position: new_position,
            velocity: new_speed,
        }
    }

    pub fn simulation() -> Result<()> {
        let init_position = Projectile {
            position: point(0.0, 1.0, 0.0),
            velocity: scale_tuple(&vector_normalize(&vector(1.0, 1.8, 0.0)), 11.25),
        };

        let env = Env {
            gravity: vector(0.0, -0.1, 0.0),
            wind: vector(-0.01, 0.0, 0.0),
        };

        let mut pos = init_position;
        let mut canvas = Canvas::make(900, 550);
        let red = Color::make(1.5, 0.0, 0.0);
        while pos.position.1 > 0.0 {
            canvas.write(
                pos.position.0.round() as usize,
                canvas.height - pos.position.1.round() as usize,
                red,
            );
            pos = pos.tick(&env);
            println!("{:?}", pos);
        }
        println!("Touchdown, creating result file!");
        canvas.save_file("projectile.ppm")
    }
}
