use crate::epsilon::EPSILON;
use crate::ray::Ray;
use crate::tuple::*;
use crate::world::World;

#[derive(Debug, PartialEq)]
pub struct Intersection {
    pub object_id: usize,
    pub distance: f64,
}

pub struct PreparedComputations {
    pub object_id: usize,
    pub intersection_distance: f64,
    pub point: Tuple,
    pub over_point: Tuple,
    pub normalv: Tuple,
    pub eyev: Tuple,
    pub inside: bool,
}

impl Intersection {
    pub fn new(object_id: usize, distance: f64) -> Intersection {
        Intersection {
            object_id,
            distance,
        }
    }

    pub fn tupled(&self) -> (usize, f64) {
        (self.object_id, self.distance)
    }

    pub fn hit(intersections: Vec<Intersection>) -> Option<(usize, f64)> {
        if intersections.is_empty() {
            None
        } else {
            intersections
                .iter()
                .filter(|i| i.distance > 0.0)
                .map(|i| i.tupled())
                .max_by(|a, b| b.1.partial_cmp(&a.1).unwrap())
        }
    }

    pub fn prepare_computations(
        intersection: &Intersection,
        ray: &Ray,
        world: &World,
    ) -> PreparedComputations {
        let (object_id, intersection_distance) = intersection.tupled();
        let point = ray.position_at(intersection_distance);
        let shape = world.objects.iter().find(|&o| o.id() == object_id).unwrap();
        let eyev = negate_tuple(&ray.direction);
        let (inside, normalv) = {
            let normalv = shape.normal_at(&point);
            // negative dot_product means the vectors are pointing in opposite direction
            if vector_dot_product(&normalv, &eyev) < 0.0 {
                // the normal is inverted for a correct illumination
                (true, negate_tuple(&normalv))
            } else {
                (false, normalv)
            }
        };
        // to prevent self shadowing we bump slightly the point in the direction of the normal
        // handpicked epsilon for this context
        let over_point = add_tuple(&point, &scale_tuple(&normalv, EPSILON));
        PreparedComputations {
            object_id,
            intersection_distance,
            point,
            over_point,
            normalv,
            eyev,
            inside,
        }
    }
}

#[cfg(test)]
mod intersection_tests {
    use crate::intersection::*;
    use crate::matrix::Matrix;
    use crate::sphere::Sphere;
    use crate::tuple::{point, vector};
    use crate::world::World;

    #[test]
    fn hit_when_all_positive() {
        let hits = vec![
            Intersection::new(1, 1.0),
            Intersection::new(1, 2.0),
            Intersection::new(2, 3.0),
        ];
        let tuple = Intersection::hit(hits).unwrap();
        assert_eq!(tuple, (1, 1.0))
    }

    #[test]
    fn hit_when_all_some_negative_positive() {
        let hits = vec![
            Intersection::new(1, -1.0),
            Intersection::new(1, 2.0),
            Intersection::new(2, 3.0),
        ];
        let tuple = Intersection::hit(hits).unwrap();
        assert_eq!(tuple, (1, 2.0))
    }

    #[test]
    fn hit_when_all_negative() {
        let hits = vec![
            Intersection::new(1, -1.0),
            Intersection::new(1, -2.0),
            Intersection::new(2, -3.0),
        ];
        let tuple = Intersection::hit(hits);
        assert_eq!(tuple.is_none(), true)
    }

    #[test]
    fn hit_always_lowest_non_negative() {
        let hits = vec![
            Intersection::new(1, 5.0),
            Intersection::new(1, 7.0),
            Intersection::new(1, -3.0),
            Intersection::new(1, 2.0),
        ];
        let tuple = Intersection::hit(hits).unwrap();
        assert_eq!(tuple, (1, 2.0))
    }

    #[test]
    fn prepare_computation_for_intersection_outside() {
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = Sphere::new(1);
        let intersection = Intersection::new(1, 4.0);
        let w = World::empty().add_object(Box::new(shape));
        let comps = Intersection::prepare_computations(&intersection, &ray, &w);
        assert_eq!(comps.object_id, intersection.object_id);
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn prepare_computation_for_intersection_inside() {
        let ray = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = Sphere::new(1);
        let intersection = Intersection::new(1, 1.0);
        let w = World::empty().add_object(Box::new(shape));
        let comps = Intersection::prepare_computations(&intersection, &ray, &w);
        assert_eq!(comps.object_id, intersection.object_id);
        assert_eq!(comps.point, point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
    }

    #[test]
    fn the_hit_offset_the_point_to_avoid_self_shadowing() {
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = Sphere::new(1).set_transform(Matrix::translation(0.0, 0.0, 1.0));
        let intersection = Intersection::new(1, 5.0);
        let w = World::empty().add_object(Box::new(shape));
        let comps = Intersection::prepare_computations(&intersection, &ray, &w);
        assert_eq!(comps.over_point.2 < -(f64::EPSILON / 2.0), true);
        assert_eq!(comps.point.2 > comps.over_point.2, true);
    }
}
