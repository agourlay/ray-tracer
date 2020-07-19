use crate::color::*;
use crate::tuple::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn default() -> Material {
        Material {
            color: Color::make(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn new(color: Color, diffuse: f64, specular: f64) -> Material {
        Material {
            color,
            ambient: 0.1,
            diffuse,
            specular,
            shininess: 200.0,
        }
    }
}

#[cfg(test)]
mod material_tests {
    use super::Material;
    use crate::color::*;
    use crate::tuple::*;

    #[test]
    fn default_material() {
        let material = Material::default();
        assert_eq!(material.color, Color::make(1.0, 1.0, 1.0));
        assert_eq!(material.ambient, 0.1);
        assert_eq!(material.diffuse, 0.9);
        assert_eq!(material.specular, 0.9);
        assert_eq!(material.shininess, 200.0);
    }
}
