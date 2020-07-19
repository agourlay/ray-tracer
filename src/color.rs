use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn make(r: f64, g: f64, b: f64) -> Self {
        Color {
            red: r,
            green: g,
            blue: b,
        }
    }

    pub fn raw(self) -> String {
        format!("{} {} {}", self.red, self.green, self.blue)
    }

    pub fn raw_scale(self, scale: usize) -> String {
        let scaled = self.scale(scale);
        scaled.raw()
    }

    pub fn scale(self, scale: usize) -> Color {
        let scale_f64 = scale as f64;
        Color {
            red: Color::scale_value(self.red, scale_f64),
            green: Color::scale_value(self.green, scale_f64),
            blue: Color::scale_value(self.blue, scale_f64),
        }
    }

    fn scale_value(value: f64, scale: f64) -> f64 {
        if value <= 0.0 {
            0.0
        } else if value > 1.0 {
            scale
        } else {
            (value * scale).round()
        }
    }

    pub fn add(self, c: &Color) -> Color {
        Color {
            red: self.red + c.red,
            green: self.green + c.green,
            blue: self.blue + c.blue,
        }
    }

    pub fn subtract(self, c: &Color) -> Color {
        Color {
            red: self.red - c.red,
            green: self.green - c.green,
            blue: self.blue - c.blue,
        }
    }

    pub fn multiply(self, c: &Color) -> Color {
        Color {
            red: self.red * c.red,
            green: self.green * c.green,
            blue: self.blue * c.blue,
        }
    }

    pub fn multiply_value(self, value: f64) -> Color {
        Color {
            red: self.red * value,
            green: self.green * value,
            blue: self.blue * value,
        }
    }
}

impl From<Tuple> for Color {
    fn from(t: Tuple) -> Self {
        Color {
            red: t.0,
            green: t.1,
            blue: t.2,
        }
    }
}

impl Default for Color {
    // TODO make const
    fn default() -> Self {
        Color {
            red: 0.0,
            blue: 0.0,
            green: 0.0,
        }
    }
}

#[cfg(test)]
mod color_tests {
    use crate::color::*;

    #[test]
    fn created_from_tuple() {
        let t = (0.5, 0.4, 1.7, 0.0);
        let c = Color::from(t);
        assert_eq!(c.red, 0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }
}
