use crate::color::Color;
use std::fs::File;
use std::io::{Result, Write};

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub content: Vec<Color>,
}

impl Canvas {
    pub fn make(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            content: [Color::default()].repeat(width * height),
        }
    }

    pub fn make_with_color(width: usize, height: usize, color: Color) -> Canvas {
        Canvas {
            width,
            height,
            content: [color].repeat(width * height),
        }
    }

    pub fn write(&mut self, x: usize, y: usize, color: Color) {
        self.content.push(color);
        self.content.swap_remove(x + y * self.width);
    }

    pub fn color_at(self, x: usize, y: usize) -> Option<Color> {
        self.content.get(x + y * self.width).copied()
    }

    pub fn to_ppm(&self) -> String {
        let first_magic_line = "P3";
        let second_dim = format!("{} {}", self.width, self.height);
        let color_scale = "255";
        let header = format!("{}\n{}\n{}", first_magic_line, second_dim, color_scale);
        // Initiate with very bold approximate size
        let mut content_lines: String = String::with_capacity(self.width * self.width);
        self.content
            .chunks(self.width) // chunk by pixel line
            .for_each(|l| {
                l.iter().fold(0, |current_line_size, c| {
                    let raw_scaled_color = c.raw_scale(255);
                    let raw_scaled_color_len = raw_scaled_color.chars().count();
                    if current_line_size == 0 {
                        // first line
                        content_lines.push_str(&raw_scaled_color);
                        raw_scaled_color_len
                    } else {
                        let next_line_size = current_line_size + raw_scaled_color_len + 1;
                        if next_line_size <= 69 {
                            // continue line
                            content_lines.push_str(" ");
                            content_lines.push_str(&raw_scaled_color);
                            next_line_size
                        } else {
                            // new line
                            content_lines.push_str("\n");
                            content_lines.push_str(&raw_scaled_color);
                            raw_scaled_color_len
                        }
                    }
                });
                // separate lines
                content_lines.push_str("\n");
            });

        format!("{}\n{}\n ", header, content_lines)
    }

    pub fn save_file(self, filename: &str) -> Result<()> {
        let mut output = File::create(filename)?;
        let ppm = self.to_ppm();
        output.write(ppm.as_bytes()).map(|_| ())
    }
}

#[cfg(test)]
mod tuple_tests {
    use crate::canvas::*;
    use crate::color::Color;

    #[test]
    fn correctly_init() {
        let c = Canvas::make(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        assert_eq!(c.content.len(), 10 * 20);
        assert!(c.content.iter().all(|&c| c == Color::default()));
    }

    #[test]
    fn insert_color_in_canvas() {
        let mut canvas = Canvas::make(10, 20);
        let color_red = Color::from((1.0, 0.0, 0.0, 0.0));
        canvas.write(2, 3, color_red);
        assert_eq!(canvas.color_at(2, 3), Some(color_red));
    }

    #[test]
    fn valid_ppm() {
        let mut canvas = Canvas::make(5, 3);
        let c1 = Color::make(1.5, 0.0, 0.0);
        let c2 = Color::make(0.0, 0.5, 0.0);
        let c3 = Color::make(-0.5, 0.0, 1.0);
        canvas.write(0, 0, c1);
        canvas.write(2, 1, c2);
        canvas.write(4, 2, c3);
        let ppm = canvas.to_ppm();
        let mut ppm_lines = ppm.lines();
        assert_eq!(ppm_lines.next(), Some("P3"));
        assert_eq!(ppm_lines.next(), Some("5 3"));
        assert_eq!(ppm_lines.next(), Some("255"));
        assert_eq!(ppm_lines.next(), Some("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0"));
        assert_eq!(ppm_lines.next(), Some("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0"));
        assert_eq!(ppm_lines.next(), Some("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"));
    }

    #[test]
    fn ppm_ends_with_new_line() {
        let mut canvas = Canvas::make(5, 3);
        let c1 = Color::make(1.5, 0.0, 0.0);
        let c2 = Color::make(0.0, 0.5, 0.0);
        let c3 = Color::make(-0.5, 0.0, 1.0);
        canvas.write(0, 0, c1);
        canvas.write(2, 1, c2);
        canvas.write(4, 2, c3);
        let ppm = canvas.to_ppm();
        let ppm_lines = ppm.lines();
        assert_eq!(ppm_lines.last(), Some(" "));
    }

    #[test]
    fn ppm_has_max_line_size() {
        let c1 = Color::make(1.0, 0.8, 0.6);
        let canvas = Canvas::make_with_color(10, 2, c1);
        let ppm = canvas.to_ppm();
        let mut ppm_lines = ppm.lines();
        assert_eq!(ppm_lines.next(), Some("P3"));
        assert_eq!(ppm_lines.next(), Some("10 2"));
        assert_eq!(ppm_lines.next(), Some("255"));
        assert_eq!(
            ppm_lines.next(),
            Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153")
        );
        assert_eq!(
            ppm_lines.next(),
            Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153")
        );
        assert_eq!(
            ppm_lines.next(),
            Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153")
        );
        assert_eq!(
            ppm_lines.next(),
            Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153")
        );
    }
}
