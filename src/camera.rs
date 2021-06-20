use crate::canvas::Canvas;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::*;
use crate::world::World;

pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f64,
    transform: Matrix,
    transform_inverse: Matrix, // cache inverse
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        let pixel_size = (half_width * 2.0) / hsize as f64;
        let transform = Matrix::identity();
        let transform_inverse = Matrix::inverse(&transform);
        Camera {
            hsize,
            vsize,
            field_of_view,
            transform,
            transform_inverse,
            pixel_size,
            half_width,
            half_height,
        }
    }

    pub fn set_transform(self, transform: Matrix) -> Camera {
        Camera {
            transform_inverse: Matrix::inverse(&transform),
            transform,
            ..self
        }
    }

    fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        // offset from the edge of the canvas of the pixel's center
        let x_offset = (px as f64 + 0.5) * self.pixel_size;
        let y_offset = (py as f64 + 0.5) * self.pixel_size;
        // untransformed coordinates of the pixel in world space
        // (remember that the camera looks forward -z, so +x is to the left.)
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;
        // using the camera matrix, transform the canvas point and the origin
        // and then compute the ray's direction vector
        // (remember that the canvas is at z=-1)
        let pixel = self
            .transform_inverse
            .multiply_tuple(&point(world_x, world_y, -1.0));
        let origin = self.transform_inverse.multiply_tuple(&point_zero());
        let direction = vector_normalize(&subtract_tuple(&pixel, &origin));
        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut canvas = Canvas::make(self.hsize, self.vsize);
        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray);
                canvas.write(x, y, color);
            }
        }
        canvas
    }
}

#[cfg(test)]
mod camera_tests {
    use crate::camera::Camera;
    use crate::color::Color;
    use crate::matrix::Matrix;
    use crate::transformation::*;
    use crate::tuple::*;
    use crate::world::World;
    use std::f32::consts::FRAC_PI_2;
    use std::f64::consts::FRAC_PI_4;

    #[test]
    fn constructing_a_camera() {
        let c = Camera::new(160, 120, FRAC_PI_2 as f64);
        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.field_of_view, FRAC_PI_2 as f64);
        assert_eq!(c.transform, Matrix::identity());
    }

    #[test]
    fn pixel_size_horizontal_canvas() {
        let c = Camera::new(200, 125, FRAC_PI_2 as f64);
        assert_eq!(c.pixel_size, 0.01000000043711391);
    }

    #[test]
    fn pixel_size_vertical_canvas() {
        let c = Camera::new(125, 200, FRAC_PI_2 as f64);
        assert_eq!(c.pixel_size, 0.01000000043711391);
    }

    #[test]
    fn ray_through_center_canvas_untransformed_one() {
        let c = Camera::new(201, 101, FRAC_PI_2 as f64);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_eq!(
            r.direction,
            vector(0.0, 0.00000000000000011102230246251565, -1.0)
        );
    }

    #[test]
    fn ray_through_center_canvas_untransformed_two() {
        let c = Camera::new(201, 101, FRAC_PI_2 as f64);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_eq!(
            r.direction,
            vector(0.6651864391138698, 0.3325932195569349, -0.6685123420878348)
        );
    }

    #[test]
    fn ray_through_center_canvas_transformed() {
        let transformation =
            Matrix::rotate_y(FRAC_PI_4).multiply(&Matrix::translation(0.0, -2.0, 5.0));
        let c = Camera::new(201, 101, FRAC_PI_2 as f64).set_transform(transformation);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, point(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            vector(0.7071067811865474, 0.0, -0.7071067811865478)
        );
    }

    #[test]
    fn rendering_world_with_camera() {
        let w = World::default();
        let from = point(0.0, 0.0, -5.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        let c =
            Camera::new(11, 11, FRAC_PI_2 as f64).set_transform(view_transform(&from, &to, &up));
        let canvas = c.render(&w);
        let color_at = canvas.color_at(5, 5);
        assert_eq!(
            color_at.unwrap(),
            Color::make(0.380661169303951945, 0.4758264616299399, 0.2854958769779639)
        );
    }
}
