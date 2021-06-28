use crate::epsilon::EPSILON;
use crate::intersection::*;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::matrix::Transformation;
use crate::ray::*;
use crate::shape::Shape;
use crate::tuple::*;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub id: usize,
    center: Tuple,
    radius: f64,
    transform: Transformation,
    pub material: Material,
}

impl Sphere {
    pub fn new(id: usize) -> Sphere {
        Sphere {
            id,
            center: point_zero(),
            radius: 1.0,
            transform: Transformation::default(),
            material: Material::default(),
        }
    }

    pub fn set_transform(self, transform: Matrix) -> Sphere {
        Sphere {
            transform: Transformation::make(transform),
            ..self
        }
    }

    pub fn set_material(self, material: Material) -> Sphere {
        Sphere { material, ..self }
    }

    pub fn set_radius(self, radius: f64) -> Sphere {
        Sphere { radius, ..self }
    }
}

impl Shape for Sphere {
    fn id(&self) -> usize {
        self.id
    }

    fn transform(&self) -> &Transformation {
        &self.transform
    }

    fn material(&self) -> &Material {
        &self.material
    }

    // https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection
    fn local_intersect(&self, local_ray: &Ray) -> Vec<Intersection> {
        // ray from the sphere center to the ray origin
        let sphere_to_ray = subtract_tuple(&local_ray.origin, &self.center);
        let a = vector_dot_product(&local_ray.direction, &local_ray.direction);
        let b = 2.0 * vector_dot_product(&local_ray.direction, &sphere_to_ray);
        let c = vector_dot_product(&sphere_to_ray, &sphere_to_ray) - self.radius;
        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            vec![]
        } else {
            let sqrt_discriminant = discriminant.sqrt();
            let two_a = 2.0 * a;
            let t1 = (-b - sqrt_discriminant) / two_a;
            let t2 = (-b + sqrt_discriminant) / two_a;
            if (t1 - t2).abs() < EPSILON {
                vec![Intersection::new(self.id, t1)]
            } else if t1 < t2 {
                vec![
                    Intersection::new(self.id, t1),
                    Intersection::new(self.id, t2),
                ]
            } else {
                vec![
                    Intersection::new(self.id, t2),
                    Intersection::new(self.id, t1),
                ]
            }
        }
    }

    fn local_normal_at(&self, local_point: &(f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
        subtract_tuple(local_point, &point_zero())
    }
}

#[cfg(test)]
mod sphere_tests {
    use crate::material::Material;
    use crate::matrix::Matrix;
    use crate::ray::*;
    use crate::shape::Shape;
    use crate::sphere::*;
    use std::f64::consts::PI;

    #[test]
    fn ray_intersects_sphere_with_two_points() {
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new(1);
        let intersections = sphere.intersect(&ray);
        assert_eq!(2, intersections.len());
        assert_eq!(intersections[0].object_id, sphere.id);
        assert_eq!(intersections[0].distance, 4.0);
        assert_eq!(intersections[1].object_id, sphere.id);
        assert_eq!(intersections[1].distance, 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let ray = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new(1);
        let intersections = sphere.intersect(&ray);
        assert_eq!(1, intersections.len());
        assert_eq!(intersections[0].object_id, sphere.id);
        assert_eq!(intersections[0].distance, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let ray = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new(1);
        let intersections = sphere.intersect(&ray);
        assert_eq!(0, intersections.len());
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let ray = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new(1);
        let intersections = sphere.intersect(&ray);
        assert_eq!(2, intersections.len());
        assert_eq!(intersections[0].object_id, sphere.id);
        assert_eq!(intersections[0].distance, -1.0);
        assert_eq!(intersections[1].object_id, sphere.id);
        assert_eq!(intersections[1].distance, 1.0);
    }

    #[test]
    fn sphere_behind_ray() {
        let ray = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new(1);
        let intersections = sphere.intersect(&ray);
        assert_eq!(2, intersections.len());
        assert_eq!(intersections[0].object_id, sphere.id);
        assert_eq!(intersections[0].distance, -6.0);
        assert_eq!(intersections[1].object_id, sphere.id);
        assert_eq!(intersections[1].distance, -4.0);
    }

    #[test]
    fn sphere_default_transform() {
        let s = Sphere::new(1);
        assert_eq!(s.transform.matrix, Matrix::identity())
    }

    #[test]
    fn changing_sphere_transform() {
        let s = Sphere::new(1);
        let t = Matrix::translation(2.0, 3.0, 4.0);
        let s2 = s.set_transform(t.clone());
        assert_eq!(s2.transform.matrix, t)
    }

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new(1).set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let intersections = sphere.intersect(&ray);
        assert_eq!(2, intersections.len());
        assert_eq!(intersections[0].object_id, sphere.id);
        assert_eq!(intersections[0].distance, 3.0);
        assert_eq!(intersections[1].object_id, sphere.id);
        assert_eq!(intersections[1].distance, 7.0);
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new(1).set_transform(Matrix::translation(5.0, 0.0, 0.0));
        let intersections = sphere.intersect(&ray);
        assert_eq!(0, intersections.len());
    }

    #[test]
    fn normal_on_x_axis() {
        let sphere = Sphere::new(1);
        let normal = sphere.normal_at(&point(1.0, 0.0, 0.0));
        assert_eq!(normal, vector(1.0, 0.0, 0.0))
    }

    #[test]
    fn normal_on_y_axis() {
        let sphere = Sphere::new(1);
        let normal = sphere.normal_at(&point(0.0, 1.0, 0.0));
        assert_eq!(normal, vector(0.0, 1.0, 0.0))
    }

    #[test]
    fn normal_on_z_axis() {
        let sphere = Sphere::new(1);
        let normal = sphere.normal_at(&point(0.0, 0.0, 1.0));
        assert_eq!(normal, vector(0.0, 0.0, 1.0))
    }

    #[test]
    fn normal_on_non_axial_point() {
        let sphere = Sphere::new(1);
        let value = 3.0_f64.sqrt() / 3.0;
        let normal = sphere.normal_at(&point(value, value, value));
        assert_eq!(normal, vector(value, value, value))
    }

    #[test]
    fn normal_is_a_normalized_vector() {
        let sphere = Sphere::new(1);
        let value = 3.0_f64.sqrt() / 3.0;
        let normal = sphere.normal_at(&point(value, value, value));
        assert_eq!(normal, vector_normalize(&normal))
    }

    #[test]
    fn normal_on_a_translated_sphere() {
        let sphere = Sphere::new(1).set_transform(Matrix::translation(0.0, 1.0, 0.0));
        let normal = sphere.normal_at(&point(0.0, 1.70711, -0.707011));
        assert_eq!(normal, vector(0.0, 0.7071562826936714, -0.7070572762137932))
    }

    #[test]
    fn normal_on_a_transformed_sphere() {
        let trans = Matrix::scaling(1.0, 0.5, 1.0).multiply(&Matrix::rotate_z(PI / 5.0));
        let sphere = Sphere::new(1).set_transform(trans);
        let value = 2.0_f64.sqrt() / 2.0;
        let normal = sphere.normal_at(&point(0.0, value, -value));
        assert_eq!(
            &normal,
            &vector(
                0.00000000000000000972703314792188,
                0.9701425001453319,
                -0.24253562503633297
            )
        )
    }

    #[test]
    fn sphere_has_default_material() {
        let s = Sphere::new(1);
        assert_eq!(s.material, Material::default())
    }

    #[test]
    fn sphere_may_be_assigned_material() {
        let s = Sphere::new(1);
        let m = Material {
            ambient: 1.0,
            ..Material::default()
        };
        let final_sphere = s.set_material(m);
        assert_eq!(
            final_sphere.material,
            Material {
                ambient: 1.0,
                ..Material::default()
            }
        )
    }
}
