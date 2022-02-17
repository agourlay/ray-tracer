use crate::color::Color;
use crate::matrix::{Matrix, Transformation};
use crate::pattern::Pattern::*;
use crate::tuple::Tuple;
use std::fmt::Debug;

// decided against the trait based solution like in Shape and went for an enum.
#[derive(Debug, PartialEq)]
pub enum Pattern {
    StripePattern {
        inner: Stripe,
        transform: Transformation,
    },
    GradientPattern {
        inner: Gradient,
        transform: Transformation,
    },
    RingPattern {
        inner: Ring,
        transform: Transformation,
    },
    CheckerPattern {
        inner: Checker,
        transform: Transformation,
    },
}

impl Pattern {
    pub fn convert_to_pattern_point(
        pattern_transformation: &Transformation,
        object_transformation: &Transformation,
        point: &Tuple,
    ) -> Tuple {
        // world-space point into object point
        let object_point = object_transformation.inverse.multiply_tuple(point);
        // object point into pattern point
        pattern_transformation.inverse.multiply_tuple(&object_point)
    }

    pub fn pattern_at_object(
        &self,
        object_transformation: &Transformation,
        point: &Tuple,
    ) -> Color {
        match self {
            Pattern::StripePattern { inner, transform } => {
                let pattern_point =
                    Pattern::convert_to_pattern_point(transform, object_transformation, point);
                inner.stripe_at(&pattern_point)
            }
            Pattern::GradientPattern { inner, transform } => {
                let pattern_point =
                    Pattern::convert_to_pattern_point(transform, object_transformation, point);
                inner.gradient_at(&pattern_point)
            }
            Pattern::RingPattern { inner, transform } => {
                let pattern_point =
                    Pattern::convert_to_pattern_point(transform, object_transformation, point);
                inner.ring_at(&pattern_point)
            }
            Pattern::CheckerPattern { inner, transform } => {
                let pattern_point =
                    Pattern::convert_to_pattern_point(transform, object_transformation, point);
                inner.checker_at(&pattern_point)
            }
        }
    }

    pub fn new_stripe(a: Color, b: Color, transform: Matrix) -> Pattern {
        StripePattern {
            inner: Stripe::new(a, b),
            transform: Transformation::make(transform),
        }
    }

    pub fn new_gradient(a: Color, b: Color, transform: Matrix) -> Pattern {
        GradientPattern {
            inner: Gradient::new(a, b),
            transform: Transformation::make(transform),
        }
    }

    pub fn new_ring(a: Color, b: Color, transform: Matrix) -> Pattern {
        RingPattern {
            inner: Ring::new(a, b),
            transform: Transformation::make(transform),
        }
    }

    pub fn new_checker(a: Color, b: Color, transform: Matrix) -> Pattern {
        CheckerPattern {
            inner: Checker::new(a, b),
            transform: Transformation::make(transform),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Checker {
    a: Color,
    b: Color,
}

impl Checker {
    pub fn new(a: Color, b: Color) -> Checker {
        Checker { a, b }
    }

    // The function for this pattern is very much like that for stripes,
    // but instead of relying on a single dimension, it relies on the sum of all three dimensions, x, y, and z.
    pub fn checker_at(&self, point: &Tuple) -> Color {
        let x = point.0.floor();
        let y = point.1.floor();
        let z = point.2.floor();
        let threshold = x + y + z;
        if threshold % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Ring {
    a: Color,
    b: Color,
}

impl Ring {
    pub fn new(a: Color, b: Color) -> Ring {
        Ring { a, b }
    }

    // It works similarly to stripes, but instead of testing the distance of the point in just x,
    // it tests the distance of the point in both x and z, which results in this pattern of concentric circles.
    pub fn ring_at(&self, point: &Tuple) -> Color {
        let x = point.0;
        let z = point.2;
        let threshold = (x.powi(2) + z.powi(2)).sqrt();
        if threshold.floor() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Gradient {
    a: Color,
    distance: Color,
}

impl Gradient {
    pub fn new(a: Color, b: Color) -> Gradient {
        // save only the distance between the two colors as it is constant
        let distance = b.subtract(&a);
        Gradient { a, distance }
    }

    // This takes the distance between the two colors, multiplies it by the fractional portion of the x coordinate, and adds the product to the first color.
    // The result is a smooth, linear transition from the first color to the second.
    pub fn gradient_at(&self, point: &Tuple) -> Color {
        let fraction = point.0.fract();
        let portion = self.distance.multiply_value(fraction);
        self.a.add(&portion)
    }
}

#[derive(Debug, PartialEq)]
pub struct Stripe {
    a: Color,
    b: Color,
}

impl Stripe {
    pub fn new(a: Color, b: Color) -> Stripe {
        Stripe { a, b }
    }

    // As the x coordinate changes, the pattern alternates between the two colors.
    // The other two dimensions, y and z, have no effect on it
    pub fn stripe_at(&self, point: &Tuple) -> Color {
        let x = point.0;
        if x < 0. {
            if x.abs() % 2. <= 1. {
                self.b
            } else {
                self.a
            }
        } else if x % 2. < 1. {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod pattern_tests {
    use crate::color::{Color, BLACK, WHITE};
    use crate::matrix::Matrix;
    use crate::pattern::*;
    use crate::shape::Shape;
    use crate::sphere::Sphere;
    use crate::tuple::point;

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = Stripe::new(WHITE, BLACK);
        assert_eq!(pattern.stripe_at(&point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.stripe_at(&point(0., 1., 0.)), WHITE);
        assert_eq!(pattern.stripe_at(&point(0., 2., 0.)), WHITE);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = Stripe::new(WHITE, BLACK);
        assert_eq!(pattern.stripe_at(&point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.stripe_at(&point(0., 0., 1.)), WHITE);
        assert_eq!(pattern.stripe_at(&point(0., 0., 2.)), WHITE);
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = Stripe::new(WHITE, BLACK);
        assert_eq!(pattern.stripe_at(&point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.stripe_at(&point(0.9, 0., 0.)), WHITE);
        assert_eq!(pattern.stripe_at(&point(1., 0., 0.)), BLACK);
        assert_eq!(pattern.stripe_at(&point(-1., 0., 0.)), BLACK);
        assert_eq!(pattern.stripe_at(&point(-1.1, 0., 0.)), WHITE);
    }

    #[test]
    fn a_stripe_pattern_on_transformed_object() {
        let s = Sphere::new(1).set_transform(Matrix::scaling(2., 2., 2.));
        let pattern = Pattern::new_stripe(WHITE, BLACK, Matrix::identity());
        let c = pattern.pattern_at_object(s.transform(), &point(1.5, 0., 0.));
        assert_eq!(c, WHITE);
    }

    #[test]
    fn a_stripe_pattern_with_transformation() {
        let s = Sphere::new(1);
        let pattern = Pattern::new_stripe(WHITE, BLACK, Matrix::scaling(2., 2., 2.));
        let c = pattern.pattern_at_object(s.transform(), &point(1.5, 0., 0.));
        assert_eq!(c, WHITE);
    }

    #[test]
    fn a_stripe_pattern_with_transformation_on_transformed_object() {
        let s = Sphere::new(1).set_transform(Matrix::scaling(2., 2., 2.));
        let pattern = Pattern::new_stripe(WHITE, BLACK, Matrix::translation(0.5, 0., 0.));
        let c = pattern.pattern_at_object(s.transform(), &point(2.5, 0., 0.));
        assert_eq!(c, WHITE);
    }

    #[test]
    fn a_gradient_pattern_linearly_interpolates_between_two_colors() {
        let g = Gradient::new(WHITE, BLACK);
        let r1 = g.gradient_at(&point(0., 0., 0.));
        assert_eq!(r1, WHITE);
        let r2 = g.gradient_at(&point(0.25, 0., 0.));
        assert_eq!(r2, Color::make(0.75, 0.75, 0.75));
        let r3 = g.gradient_at(&point(0.5, 0., 0.));
        assert_eq!(r3, Color::make(0.5, 0.5, 0.5));
        let r4 = g.gradient_at(&point(0.75, 0., 0.));
        assert_eq!(r4, Color::make(0.25, 0.25, 0.25));
    }

    #[test]
    fn a_ring_pattern_should_extend_in_both_x_and_z() {
        let g = Ring::new(WHITE, BLACK);
        let r1 = g.ring_at(&point(0., 0., 0.));
        assert_eq!(r1, WHITE);
        let r2 = g.ring_at(&point(1., 0., 0.));
        assert_eq!(r2, BLACK);
        let r3 = g.ring_at(&point(0., 0., 1.));
        assert_eq!(r3, BLACK);
        // 0.708 is just slightly more than √2/2
        let r4 = g.ring_at(&point(0.708, 0., 0.708));
        assert_eq!(r4, BLACK);
    }

    #[test]
    fn a_checker_pattern_should_repeat_in_x() {
        let g = Checker::new(WHITE, BLACK);
        let r1 = g.checker_at(&point(0., 0., 0.));
        assert_eq!(r1, WHITE);
        let r2 = g.checker_at(&point(0.99, 0., 0.));
        assert_eq!(r2, WHITE);
        let r3 = g.checker_at(&point(1.01, 0., 0.));
        assert_eq!(r3, BLACK);
    }

    #[test]
    fn a_checker_pattern_should_repeat_in_y() {
        let g = Checker::new(WHITE, BLACK);
        let r1 = g.checker_at(&point(0., 0., 0.));
        assert_eq!(r1, WHITE);
        let r2 = g.checker_at(&point(0., 0.99, 0.));
        assert_eq!(r2, WHITE);
        let r3 = g.checker_at(&point(0., 1.01, 0.));
        assert_eq!(r3, BLACK);
    }

    #[test]
    fn a_checker_pattern_should_repeat_in_z() {
        let g = Checker::new(WHITE, BLACK);
        let r1 = g.checker_at(&point(0., 0., 0.));
        assert_eq!(r1, WHITE);
        let r2 = g.checker_at(&point(0., 0., 0.99));
        assert_eq!(r2, WHITE);
        let r3 = g.checker_at(&point(0., 0., 1.01));
        assert_eq!(r3, BLACK);
    }
}
