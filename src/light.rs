use crate::color::*;
use crate::material::Material;
use crate::matrix::Transformation;
use crate::tuple::*;

#[derive(Debug, PartialEq)]
pub struct Light {
    pub position: Tuple,
    pub intensity: Color,
}

impl Light {
    pub fn point_light(position: Tuple, intensity: Color) -> Light {
        Light {
            position,
            intensity,
        }
    }

    pub fn lighting(
        &self,
        material: &Material,
        object_transformation: &Transformation,
        point: &Tuple,
        eyev: &Tuple,
        normalv: &Tuple,
        in_shadow: bool,
    ) -> Color {
        let color = match &material.pattern {
            None => material.color,
            Some(p) => p.pattern_at_object(object_transformation, point),
        };
        // combine the surface color with the light's color/intensity
        let effective_color = color.multiply(&self.intensity);
        // find the direction to the light source
        let lightv = vector_normalize(&subtract_tuple(&self.position, point));
        // compute the ambient contribution
        let ambient = effective_color.multiply_value(material.ambient);

        let mut diffuse = Color::default();
        let mut specular = Color::default();

        // light can't contribute to diffuse & specular
        if !in_shadow {
            // light_dot_normal represents the cosine of the angle between the light vector and the normal vector.
            // A negative number means the light is on the other side of the surface.
            let light_dot_normal = vector_dot_product(&lightv, normalv);

            if light_dot_normal >= 0.0 {
                diffuse = effective_color.multiply_value(material.diffuse * light_dot_normal);
                let reflectv = vector_reflect(&negate_tuple(&lightv), normalv);
                let reflect_dot_eye = vector_dot_product(&reflectv, eyev);
                if reflect_dot_eye >= 0.0 {
                    let factor = reflect_dot_eye.powf(material.shininess);
                    specular = self.intensity.multiply_value(material.specular * factor)
                }
            };
        }
        ambient.add(&diffuse).add(&specular)
    }
}

#[cfg(test)]
mod light_tests {
    use super::Light;
    use crate::color::*;
    use crate::material::Material;
    use crate::matrix::{Matrix, Transformation};
    use crate::pattern::Pattern::StripePattern;
    use crate::pattern::{Pattern, Stripe};
    use crate::tuple::*;

    #[test]
    fn creating_point_light() {
        let intensity = Color::make(1.0, 1.0, 1.0);
        let position = point(0.0, 0.0, 0.0);
        let light = Light::point_light(position, intensity);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }

    #[test]
    fn lighting_eye_between_light_and_surface() {
        let m = Material::default();
        let p = point(0.0, 0.0, 0.0);
        let eye = vector(0.0, 0.0, -1.0);
        let normal = vector(0.0, 0.0, -1.0);
        let light = Light::point_light(point(0.0, 0.0, -10.0), Color::make(1.0, 1.0, 1.0));
        let t = Transformation::default();
        let result = light.lighting(&m, &t, &p, &eye, &normal, false);
        assert_eq!(result, Color::make(1.9, 1.9, 1.9))
    }

    #[test]
    fn lighting_eye_between_light_and_surface_eye_offset_45_deg() {
        let m = Material::default();
        let p = point(0.0, 0.0, 0.0);
        let value = 2.0_f64.sqrt() / 2.0;
        let eye = vector(0.0, value, value);
        let normal = vector(0.0, 0.0, -1.0);
        let light = Light::point_light(point(0.0, 0.0, -10.0), Color::make(1.0, 1.0, 1.0));
        let t = Transformation::default();
        let result = light.lighting(&m, &t, &p, &eye, &normal, false);
        assert_eq!(result, Color::make(1.0, 1.0, 1.0))
    }

    #[test]
    fn lighting_eye_between_light_and_surface_light_offset_45_deg() {
        let m = Material::default();
        let p = point(0.0, 0.0, 0.0);
        let eye = vector(0.0, 0.0, -1.0);
        let normal = vector(0.0, 0.0, -1.0);
        let light = Light::point_light(point(0.0, 10.0, -10.0), Color::make(1.0, 1.0, 1.0));
        let t = Transformation::default();
        let result = light.lighting(&m, &t, &p, &eye, &normal, false);
        let value = 0.7363961030678927;
        assert_eq!(result, Color::make(value, value, value))
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let m = Material::default();
        let p = point(0.0, 0.0, 0.0);
        let eye = vector(0.0, 0.0, -1.0);
        let normal = vector(0.0, 0.0, -1.0);
        let light = Light::point_light(point(0.0, 0.0, 10.0), Color::make(1.0, 1.0, 1.0));
        let t = Transformation::default();
        let result = light.lighting(&m, &t, &p, &eye, &normal, false);
        assert_eq!(result, Color::make(0.1, 0.1, 0.1))
    }

    #[test]
    fn lighting_with_light_in_shadow() {
        let m = Material::default();
        let p = point(0.0, 0.0, 0.0);
        let eye = vector(0.0, 0.0, -1.0);
        let normal = vector(0.0, 0.0, -1.0);
        let light = Light::point_light(point(0.0, 0.0, -10.0), Color::make(1.0, 1.0, 1.0));
        let t = Transformation::default();
        let result = light.lighting(&m, &t, &p, &eye, &normal, true);
        assert_eq!(result, Color::make(0.1, 0.1, 0.1))
    }

    #[test]
    fn lighting_with_pattern_applied() {
        let p = Pattern::new_stripe(WHITE, BLACK, Matrix::identity());
        let m = Material {
            color: WHITE,
            ambient: 1.,
            diffuse: 0.,
            specular: 0.,
            shininess: 200.0,
            pattern: Some(p),
        };
        let eye = vector(0.0, 0.0, -1.0);
        let normal = vector(0.0, 0.0, -1.0);
        let light = Light::point_light(point(0.0, 0.0, -10.0), Color::make(1.0, 1.0, 1.0));
        let t = Transformation::default();

        let p1 = point(0.9, 0.0, 0.0);
        let r1 = light.lighting(&m, &t, &p1, &eye, &normal, true);
        assert_eq!(r1, Color::make(1., 1., 1.));

        let p2 = point(1.1, 0.0, 0.0);
        let r2 = light.lighting(&m, &t, &p2, &eye, &normal, true);
        assert_eq!(r2, Color::make(0., 0., 0.))
    }
}
