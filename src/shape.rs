use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Transformation;
use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::tuple::*;

pub trait Shape {
    fn id(&self) -> usize;
    fn transform(&self) -> &Transformation;
    fn material(&self) -> &Material;
    fn local_intersect(&self, local_ray: &Ray) -> Vec<Intersection>;
    fn local_normal_at(&self, local_point: &Tuple) -> Tuple;

    fn normal_at(&self, p: &Tuple) -> Tuple {
        let local_point = self.transform().inverse.multiply_tuple(&p);
        let local_normal = self.local_normal_at(&local_point);
        let world_normal = self
            .transform()
            .inverse_transpose
            .multiply_tuple(&local_normal);
        let tmp = vector(world_normal.0, world_normal.1, world_normal.2);
        vector_normalize(&tmp)
    }

    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(&self.transform().inverse);
        self.local_intersect(&local_ray)
    }
}

#[cfg(test)]
mod shape_tests {
    use crate::intersection::Intersection;
    use crate::material::Material;
    use crate::matrix::Matrix;
    use crate::matrix::Transformation;
    use crate::ray::Ray;
    use crate::shape::Shape;

    struct TestShape {
        transform: Transformation,
        material: Material,
    }

    impl TestShape {
        fn new() -> Self {
            TestShape {
                transform: Transformation::default(),
                material: Material::default(),
            }
        }

        fn set_transform(self, transform: Matrix) -> TestShape {
            let inverse = Matrix::inverse(&transform);
            let inverse_transpose = inverse.transpose();
            let transformation = Transformation {
                matrix: transform,
                inverse,
                inverse_transpose,
            };
            TestShape {
                transform: transformation,
                ..self
            }
        }

        fn set_material(self, material: Material) -> TestShape {
            TestShape { material, ..self }
        }
    }

    impl Shape for TestShape {
        fn id(&self) -> usize {
            unimplemented!()
        }

        fn transform(&self) -> &Transformation {
            &self.transform
        }

        fn material(&self) -> &Material {
            &self.material
        }

        fn local_intersect(&self, local_ray: &Ray) -> Vec<Intersection> {
            unimplemented!()
        }

        fn local_normal_at(&self, local_point: &(f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
            unimplemented!()
        }
    }

    #[test]
    fn has_a_default_transformation() {
        let s = TestShape::new();
        assert_eq!(s.transform.matrix, Matrix::identity())
    }

    #[test]
    fn can_set_transformation() {
        let s = TestShape::new();
        let translation = Matrix::translation(2.0, 3.0, 4.0);
        let s2 = s.set_transform(translation.clone());
        assert_eq!(s2.transform().matrix, translation)
    }

    #[test]
    fn has_a_default_material() {
        let s = TestShape::new();
        assert_eq!(s.material, Material::default())
    }

    #[test]
    fn can_set_material() {
        let s = TestShape::new();
        let default_m = s.material();
        let new_m = Material::new(default_m.color, default_m.diffuse, 1.0);
        let s2 = s.set_material(new_m);
        assert_eq!(s2.material().specular, 1.0)
    }
}
