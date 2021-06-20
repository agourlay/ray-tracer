use crate::epsilon::EPSILON;
use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Transformation;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::tuple::*;

// xz plane
#[derive(Debug, PartialEq, Clone)]
pub struct Plane {
    pub id: usize,
    transform: Transformation,
    pub material: Material,
}

impl Plane {
    pub fn new(id: usize) -> Plane {
        Plane {
            id,
            transform: Transformation::default(),
            material: Material::default(),
        }
    }
}

impl Shape for Plane {
    fn id(&self) -> usize {
        self.id
    }

    fn transform(&self) -> Transformation {
        self.transform.clone()
    }

    fn material(&self) -> Material {
        self.material
    }

    fn local_intersect(&self, local_ray: &Ray) -> Vec<Intersection> {
        // To know if a ray is parallel to the plane, you need to note that the plane is in xz, it has no slope in y at all.
        // Thus, if your ray’s direction vector also has no slope in y (its y component is 0), it is parallel to the plane.
        // In practice, you’ll want to treat any tiny number as 0 for this comparison”
        if local_ray.direction.1.abs() < EPSILON {
            Vec::new()
        } else {
            let distance = -local_ray.origin.1 / local_ray.direction.1;
            let intersection = Intersection::new(self.id(), distance);
            vec![intersection]
        }
    }

    fn local_normal_at(&self, local_point: &(f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
        vector(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod plane_tests {
    use crate::material::Material;
    use crate::matrix::Matrix;
    use crate::plane::Plane;
    use crate::ray::*;
    use crate::shape::Shape;
    use crate::sphere::*;
    use crate::tuple::*;

    #[test]
    fn normal_of_plan_is_constant_everywhere() {
        let p = Plane::new(1);
        let n1 = p.local_normal_at(&(0.0, 0.0, 0.0, 0.0));
        let expected = vector(0.0, 1.0, 0.0);
        assert_eq!(n1, expected);
        let n2 = p.local_normal_at(&(10.0, 0.0, -10.0, 0.0));
        assert_eq!(n2, expected);
        let n3 = p.local_normal_at(&(-5.0, 0.0, 150.0, 0.0));
        assert_eq!(n3, expected);
    }

    #[test]
    fn intersect_parallel_to_the_plane() {
        let p = Plane::new(1);
        let ray = Ray::new(point(0.0, 10.0, 0.0), vector(0.0, 0.0, 1.0));
        let intersections = p.local_intersect(&ray);
        assert_eq!(intersections.is_empty(), true)
    }

    #[test]
    fn intersect_coplanar_to_the_plane() {
        let p = Plane::new(1);
        let ray = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let intersections = p.local_intersect(&ray);
        assert_eq!(intersections.is_empty(), true)
    }

    #[test]
    fn intersect_plane_from_above() {
        let p = Plane::new(1);
        let ray = Ray::new(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));
        let intersections = p.local_intersect(&ray);
        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].object_id, p.id);
        assert_eq!(intersections[0].distance, 1.0);
    }

    #[test]
    fn intersect_plane_from_below() {
        let p = Plane::new(1);
        let ray = Ray::new(point(0.0, -1.0, 0.0), vector(0.0, 1.0, 0.0));
        let intersections = p.local_intersect(&ray);
        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].object_id, p.id);
        assert_eq!(intersections[0].distance, 1.0);
    }
}
