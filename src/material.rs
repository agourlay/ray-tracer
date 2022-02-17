use crate::color::*;
use crate::pattern::Pattern;

#[derive(Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub pattern: Option<Pattern>,
}

impl Material {
    pub fn default() -> Material {
        Material {
            color: WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: None,
        }
    }

    pub fn new(color: Color, diffuse: f64, specular: f64) -> Material {
        Material {
            color,
            ambient: 0.1,
            diffuse,
            specular,
            shininess: 200.0,
            pattern: None,
        }
    }

    pub fn new_with_pattern(
        color: Color,
        diffuse: f64,
        specular: f64,
        pattern: Pattern,
    ) -> Material {
        Material {
            color,
            ambient: 0.1,
            diffuse,
            specular,
            shininess: 200.0,
            pattern: Some(pattern),
        }
    }

    pub fn set_pattern(self, pattern: Pattern) -> Material {
        Material {
            pattern: Some(pattern),
            ..self
        }
    }
}

#[cfg(test)]
mod material_tests {
    use super::Material;
    use crate::color::*;

    #[test]
    fn default_material() {
        let material = Material::default();
        assert_eq!(material.color, Color::make(1.0, 1.0, 1.0));
        assert_eq!(material.ambient, 0.1);
        assert_eq!(material.diffuse, 0.9);
        assert_eq!(material.specular, 0.9);
        assert_eq!(material.shininess, 200.0);
        assert!(material.pattern.is_none());
    }
}
