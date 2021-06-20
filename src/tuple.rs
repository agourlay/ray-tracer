pub type Tuple = (f64, f64, f64, f64);

pub fn tuples_are_equal(t1: &Tuple, t2: &Tuple) -> bool {
    ((t1.0 - t2.0).abs() < f64::EPSILON)
        && ((t1.1 - t2.1).abs() < f64::EPSILON)
        && ((t1.2 - t2.2).abs() < f64::EPSILON)
        && ((t1.3 - t2.3).abs() < f64::EPSILON)
}

pub fn tuple_is_vector(t: &Tuple) -> bool {
    t.3 == 0.0
}

pub fn tuple_is_point(t: &Tuple) -> bool {
    (t.3 - 1.0_f64).abs() < f64::EPSILON
}

pub fn add_tuple(t1: &Tuple, t2: &Tuple) -> Tuple {
    (t1.0 + t2.0, t1.1 + t2.1, t1.2 + t2.2, t1.3 + t2.3)
}

pub fn subtract_tuple(t1: &Tuple, t2: &Tuple) -> Tuple {
    (t1.0 - t2.0, t1.1 - t2.1, t1.2 - t2.2, t1.3 - t2.3)
}

pub fn negate_tuple(t1: &Tuple) -> Tuple {
    (-t1.0, -t1.1, -t1.2, -t1.3)
}

pub fn scale_tuple(v: &Tuple, scale: f64) -> Tuple {
    (v.0 * scale, v.1 * scale, v.2 * scale, v.3 * scale)
}

pub fn scale_tuple_division(v: &Tuple, scale: f64) -> Tuple {
    (v.0 / scale, v.1 / scale, v.2 / scale, v.3 / scale)
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    (x, y, z, 1.0)
}

pub const fn point_zero() -> Tuple {
    // point(0.0, 0.0, 0.0)
    (0.0, 0.0, 0.0, 1.0)
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    (x, y, z, 0.0)
}

pub fn vector_magnitude(v: &Tuple) -> f64 {
    (v.0.powi(2) + v.1.powi(2) + v.2.powi(2)).sqrt()
}

pub fn vector_normalize(v: &Tuple) -> Tuple {
    let mag = vector_magnitude(&v);
    (v.0 / mag, v.1 / mag, v.2 / mag, 0.0)
}

pub fn vector_dot_product(t1: &Tuple, t2: &Tuple) -> f64 {
    // t1.0.m * t2.0 + t1.1 * t2.1 + t1.2 * t2.2
    // using mul_add https://rust-lang.github.io/rust-clippy/master/index.html#manual_mul_add
    t1.0.mul_add(t2.0, t1.1.mul_add(t2.1, t1.2 * t2.2))
}

pub fn vector_cross_product(a: &Tuple, b: &Tuple) -> Tuple {
    vector(
        a.1 * b.2 - a.2 * b.1,
        a.2 * b.0 - a.0 * b.2,
        a.0 * b.1 - a.1 * b.0,
    )
}

pub fn vector_reflect(v: &Tuple, normal: &Tuple) -> Tuple {
    let dot = vector_dot_product(v, normal);
    let other = scale_tuple(normal, 2.0 * dot);
    subtract_tuple(v, &other)
}

#[cfg(test)]
mod tuple_tests {
    use crate::tuple::*;

    #[test]
    fn is_vector() {
        assert_eq!(tuple_is_vector(&(1.0, 2.0, -3.0, 0.0)), true);
    }

    #[test]
    fn is_not_vector() {
        assert_eq!(tuple_is_vector(&(1.0, 2.0, -3.0, 1.0)), false);
    }

    #[test]
    fn vector_is_point() {
        assert_eq!(tuple_is_vector(&vector(1.0, 2.0, -3.0)), true);
    }

    #[test]
    fn is_point() {
        assert_eq!(tuple_is_point(&(1.0, 2.0, -3.0, 1.0)), true);
    }

    #[test]
    fn is_not_point() {
        assert_eq!(tuple_is_point(&(1.0, 2.0, -3.0, 0.0)), false);
    }

    #[test]
    fn point_is_point() {
        assert_eq!(tuple_is_point(&point(1.0, 2.0, -3.0)), true);
    }

    #[test]
    fn tuple_addition() {
        let t1 = (3.0, -2.0, 5.0, 1.0);
        let t2 = (-2.0, 3.0, 1.0, 0.0);
        assert_eq!(add_tuple(&t1, &t2), (1.0, 1.0, 6.0, 1.0))
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        assert_eq!(subtract_tuple(&p1, &p2), vector(-2.0, -4.0, -6.0))
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = vector(5.0, 6.0, 7.0);
        assert_eq!(subtract_tuple(&p1, &p2), point(-2.0, -4.0, -6.0))
    }

    #[test]
    fn subtracting_two_vectors() {
        let p1 = vector(3.0, 2.0, 1.0);
        let p2 = vector(5.0, 6.0, 7.0);
        assert_eq!(subtract_tuple(&p1, &p2), vector(-2.0, -4.0, -6.0))
    }

    #[test]
    fn subtracting_a_vector_from_vector_zero() {
        let p1 = vector(0.0, 0.0, 0.0);
        let p2 = vector(1.0, -2.0, 3.0);
        assert_eq!(subtract_tuple(&p1, &p2), vector(-1.0, 2.0, -3.0))
    }

    #[test]
    fn negating_a_tuple() {
        let p2 = vector(1.0, -2.0, 3.0);
        assert_eq!(negate_tuple(&p2), (-1.0, 2.0, -3.0, 0.0))
    }

    #[test]
    fn scaling_tuple() {
        let t = (1.0, -2.0, 3.0, -4.0);
        assert_eq!(scale_tuple(&t, 3.5), (3.5, -7.0, 10.5, -14.0))
    }

    #[test]
    fn scaling_tuple_by_fraction() {
        let t = (1.0, -2.0, 3.0, -4.0);
        assert_eq!(scale_tuple(&t, 0.5), (0.5, -1.0, 1.5, -2.0))
    }

    #[test]
    fn scaling_tuple_division() {
        let t = (1.0, -2.0, 3.0, -4.0);
        assert_eq!(scale_tuple_division(&t, 2.0), (0.5, -1.0, 1.5, -2.0))
    }

    #[test]
    fn magnitude_of_vector() {
        let t = vector(-1.0, -2.0, -3.0);
        let t1: f64 = 14.0; //cast issue?
        assert_eq!(vector_magnitude(&t), t1.sqrt())
    }

    #[test]
    fn normalize_vector() {
        let t = vector(1.0, 2.0, 3.0);
        let t1: f64 = 14.0; //cast issue?
        let tmp = t1.sqrt();
        assert_eq!(vector_normalize(&t), (1.0 / tmp, 2.0 / tmp, 3.0 / tmp, 0.0))
    }

    #[test]
    fn dot_product_of_vectors() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);
        assert_eq!(vector_dot_product(&v1, &v2), 20.0)
    }

    #[test]
    fn cross_product_of_vectors() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);
        assert_eq!(vector_cross_product(&v1, &v2), vector(-1.0, 2.0, -1.0));
        assert_eq!(vector_cross_product(&v2, &v1), vector(1.0, -2.0, 1.0))
    }

    #[test]
    fn reflecting_vector_45_deg() {
        let v = vector(1.0, -1.0, 0.0);
        let n = vector(0.0, 1.0, 0.0);
        let r = vector_reflect(&v, &n);
        assert_eq!(r, vector(1.0, 1.0, 0.0))
    }

    #[test]
    fn reflecting_vector_off_slanted_surface() {
        let v = vector(0.0, -1.0, 0.0);
        let value = 2.0_f64.sqrt() / 2.0;
        let n = vector(value, value, 0.0);
        let r = vector_reflect(&v, &n);
        assert_eq!(
            r,
            vector(1.0000000000000002, 0.0000000000000002220446049250313, 0.0)
        )
    }
}
