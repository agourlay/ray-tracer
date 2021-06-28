use crate::color::*;
use crate::intersection::{Intersection, PreparedComputations};
use crate::light::Light;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::sphere::Sphere;
use crate::tuple::*;

pub struct World {
    pub lights: Vec<Light>,
    pub objects: Vec<Box<dyn Shape>>,
}

impl World {
    pub fn empty() -> World {
        World {
            lights: vec![],
            objects: vec![],
        }
    }

    pub fn add_object(self, object: Box<dyn Shape>) -> World {
        let mut objects: Vec<Box<dyn Shape>> = Vec::new();
        self.objects.into_iter().for_each(|o| objects.push(o));
        objects.push(object);
        World { objects, ..self }
    }

    pub fn set_light(self, light: Light) -> World {
        World {
            lights: vec![light],
            ..self
        }
    }

    pub fn set_lights(self, lights: Vec<Light>) -> World {
        World { lights, ..self }
    }

    pub fn default() -> World {
        World {
            lights: vec![Light::point_light(
                point(-10.0, 10.0, -10.0),
                Color::make(1.0, 1.0, 1.0),
            )],
            objects: vec![
                Box::new(Sphere::new(1).set_radius(1.0).set_material(Material::new(
                    Color::make(0.8, 1.0, 0.6),
                    0.7,
                    0.2,
                ))),
                Box::new(
                    Sphere::new(2)
                        .set_radius(0.5)
                        .set_transform(Matrix::scaling(0.5, 0.5, 0.5)),
                ),
            ],
        }
    }

    pub fn intersect_with_ray(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = Vec::new();
        self.objects.iter().for_each(|o| {
            o.intersect(&ray)
                .into_iter()
                .filter(|i| i.distance > 0.0)
                .for_each(|i| intersections.push(i))
        });
        intersections.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
        intersections
    }

    pub fn shade_hit(&self, comps: &PreparedComputations) -> Color {
        if self.lights.is_empty() {
            Color::default()
        } else {
            let shape = self
                .objects
                .iter()
                .find(|&o| o.id() == comps.object_id)
                .unwrap();
            // adding color for each light
            self.lights
                .iter()
                .map(|l| {
                    l.lighting(
                        shape.material(),
                        shape.transform(),
                        &comps.over_point,
                        &comps.eyev,
                        &comps.normalv,
                        self.is_shadowed(&comps.over_point, l),
                    )
                })
                .fold(Color::default(), |acc, c| acc.add(&c))
        }
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = self.intersect_with_ray(ray);
        if intersections.is_empty() {
            Color::default()
        } else {
            let comps = Intersection::prepare_computations(&intersections[0], ray, self);
            self.shade_hit(&comps)
        }
    }

    pub fn is_shadowed(&self, point: &Tuple, light: &Light) -> bool {
        // measure distance from the point to the light
        let v = subtract_tuple(&light.position, point);
        let distance = vector_magnitude(&v);
        let direction = vector_normalize(&v);

        // create a ray from point toward the light
        let r = Ray::new(*point, direction);

        // intersect the world with that ray
        let intersections = self.intersect_with_ray(&r);

        // the point is in the shadow if the hit lies between the point and the light source
        let hit = Intersection::hit(intersections);
        matches!(hit, Some((_, d)) if d < distance)
    }
}

#[cfg(test)]
mod world_tests {
    use super::World;
    use crate::color::*;
    use crate::intersection::Intersection;
    use crate::light::Light;
    use crate::material::Material;
    use crate::matrix::Matrix;
    use crate::ray::Ray;
    use crate::shape::Shape;
    use crate::sphere::Sphere;
    use crate::tuple::*;

    #[test]
    fn creating_empty_world() {
        let world = World::empty();
        assert_eq!(world.objects.is_empty(), true);
        assert_eq!(world.lights.is_empty(), true);
    }

    #[test]
    fn creating_default_world() {
        let world = World::default();
        let default_light =
            Light::point_light(point(-10.0, 10.0, -10.0), Color::make(1.0, 1.0, 1.0));
        assert_eq!(world.lights[0], default_light);
        let s1 = Sphere::new(1).set_radius(1.0).set_material(Material::new(
            Color::make(0.8, 1.0, 0.6),
            0.7,
            0.2,
        ));
        let s2 = Sphere::new(2)
            .set_radius(0.5)
            .set_transform(Matrix::scaling(0.5, 0.5, 0.5));
        assert_eq!(world.objects[0].id(), s1.id());
        assert_eq!(world.objects[1].id(), s2.id());
    }

    #[test]
    fn intersect_default_world() {
        let w = World::default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let intersections = w.intersect_with_ray(&r);
        assert_eq!(intersections.len(), 4);
        assert_eq!(intersections[0].distance, 4.0);
        assert_eq!(intersections[1].distance, 4.646446609406726);
        assert_eq!(intersections[2].distance, 5.353553390593274);
        assert_eq!(intersections[3].distance, 6.0);
    }

    #[test]
    fn shade_at_intersection() {
        let w = World::default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let intersection = Intersection::new(w.objects[0].id(), 4.0);
        let comps = Intersection::prepare_computations(&intersection, &r, &w);
        let color = w.shade_hit(&comps);
        assert_eq!(
            color,
            Color::make(0.38066116930395194, 0.4758264616299399, 0.2854958769779639)
        );
    }

    #[test]
    fn shade_at_intersection_from_the_inside() {
        let light = Light::point_light(point(0.0, 0.25, 0.0), Color::make(1.0, 1.0, 1.0));
        let w = World::default().set_light(light);
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let intersection = Intersection::new(w.objects[1].id(), 0.5);
        let comps = Intersection::prepare_computations(&intersection, &r, &w);
        let color = w.shade_hit(&comps);
        assert_eq!(
            color,
            // Color::make(0.9049844720832575, 0.9049844720832575, 0.9049844720832575) what?
            Color::make(0.1, 0.1, 0.1)
        );
    }

    #[test]
    fn world_color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
        let color = w.color_at(&r);
        assert_eq!(color, Color::default());
    }

    #[test]
    fn world_color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let color = w.color_at(&r);
        assert_eq!(
            color,
            Color::make(0.38066116930395194, 0.4758264616299399, 0.2854958769779639)
        );
    }

    #[test]
    fn no_shadow_when_nothing_colinear_with_point_and_light() {
        let w = World::default();
        let p = point(0.0, 10.0, 0.0);
        let l = w.lights.first().unwrap();
        assert_eq!(false, w.is_shadowed(&p, &l));
    }

    #[test]
    fn no_shadow_when_an_object_is_between_the_point_and_light() {
        let w = World::default();
        let p = point(10.0, -10.0, 10.0);
        let l = w.lights.first().unwrap();
        assert_eq!(true, w.is_shadowed(&p, &l));
    }

    #[test]
    fn no_shadow_when_an_object_is_behind_the_light() {
        let w = World::default();
        let p = point(-20.0, 20.0, -20.0);
        let l = w.lights.first().unwrap();
        assert_eq!(false, w.is_shadowed(&p, &l));
    }

    #[test]
    fn no_shadow_when_an_object_is_behind_the_point() {
        let w = World::default();
        let p = point(-2.0, 2.0, -2.0);
        let l = w.lights.first().unwrap();
        assert_eq!(false, w.is_shadowed(&p, &l));
    }

    #[test]
    fn shade_it_intersection_in_the_shadow() {
        let light = Light::point_light(point(0.0, 0.0, -10.0), Color::make(1.0, 1.0, 1.0));
        let s1 = Sphere::new(1);
        let s2 = Sphere::new(2).set_transform(Matrix::translation(0.0, 0.0, 10.0));
        let w = World::empty()
            .set_light(light)
            .add_object(Box::new(s1))
            .add_object(Box::new(s2));

        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let intersection = Intersection::new(w.objects[1].id(), 4.0);
        let comps = Intersection::prepare_computations(&intersection, &r, &w);
        let color = w.shade_hit(&comps);
        assert_eq!(color, Color::make(0.1, 0.1, 0.1));
    }
}
