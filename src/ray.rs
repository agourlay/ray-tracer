use crate::matrix::Matrix;
use crate::tuple::{add_tuple, scale_tuple, Tuple};

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray { origin, direction }
    }

    pub fn position_at(&self, t: f64) -> Tuple {
        let scaled = scale_tuple(&self.direction, t);
        add_tuple(&self.origin, &scaled)
    }

    pub fn transform(&self, matrix: &Matrix) -> Ray {
        Ray {
            origin: matrix.multiply_tuple(&self.origin),
            direction: matrix.multiply_tuple(&self.direction),
        }
    }
}

#[cfg(test)]
mod ray_tests {
    use super::Ray;
    use crate::matrix::Matrix;
    use crate::tuple::*;

    #[test]
    fn creating_querying_ray() {
        let origin = point(1.0, 2.0, 3.0);
        let direction = vector(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn computing_point_from_distance() {
        let origin = point(2.0, 3.0, 4.0);
        let direction = vector(1.0, 0.0, 0.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(ray.position_at(0.0), origin);
        assert_eq!(ray.position_at(1.0), point(3.0, 3.0, 4.0));
        assert_eq!(ray.position_at(-1.0), point(1.0, 3.0, 4.0));
        assert_eq!(ray.position_at(2.5), point(4.5, 3.0, 4.0));
    }

    #[test]
    fn translating_ray() {
        let r = Ray::new(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = Matrix::translation(3.0, 4.0, 5.0);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin, point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn scaling_ray() {
        let r = Ray::new(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = Matrix::scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin, point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, vector(0.0, 3.0, 0.0));
    }
}
