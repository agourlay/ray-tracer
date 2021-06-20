use crate::tuple::Tuple;

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix {
    pub size: usize,
    pub content: Vec<f64>,
}

impl Matrix {
    pub fn make_matrix_2(aa: f64, ab: f64, ba: f64, bb: f64) -> Matrix {
        Matrix {
            size: 2,
            content: vec![aa, ab, ba, bb],
        }
    }

    pub fn make_matrix_3(
        aa: f64,
        ab: f64,
        ac: f64,
        ba: f64,
        bb: f64,
        bc: f64,
        ca: f64,
        cb: f64,
        cc: f64,
    ) -> Matrix {
        Matrix {
            size: 3,
            content: vec![aa, ab, ac, ba, bb, bc, ca, cb, cc],
        }
    }

    pub fn make_matrix_4(
        aa: f64,
        ab: f64,
        ac: f64,
        ad: f64,
        ba: f64,
        bb: f64,
        bc: f64,
        bd: f64,
        ca: f64,
        cb: f64,
        cc: f64,
        cd: f64,
        da: f64,
        db: f64,
        dc: f64,
        dd: f64,
    ) -> Matrix {
        Matrix {
            size: 4,
            content: vec![
                aa, ab, ac, ad, ba, bb, bc, bd, ca, cb, cc, cd, da, db, dc, dd,
            ],
        }
    }

    // TODO make const?
    pub fn identity() -> Matrix {
        Matrix::make_matrix_4(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
        Matrix::make_matrix_4(
            1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
        Matrix::make_matrix_4(
            x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn at(&self, x: usize, y: usize) -> f64 {
        self.content.get(y + x * self.size).copied().unwrap()
    }

    pub fn inverse(&self) -> Matrix {
        let det = self.determinant();
        if det == 0.0 {
            panic!("matrix cannot be inverted because its determinant is 0")
        } else {
            let s = self.size;
            let s_square = s * s;
            let mut inverse: Vec<f64> = Vec::with_capacity(s_square);
            // init vector
            for index in 0..s_square {
                inverse.insert(index, 0.0);
            }
            for row in 0..s {
                for col in 0..s {
                    let col_index = col * s;
                    let cofactor = self.cofactor(row, col);
                    // we perform the transpose operation at insertion time
                    // by switching row/col in the target matrix
                    let target_index = row + col_index;
                    let precise_value = cofactor / det;
                    inverse.remove(target_index);
                    inverse.insert(target_index, precise_value);
                }
            }
            Matrix {
                size: s,
                content: inverse,
            }
        }
    }

    pub fn determinant(&self) -> f64 {
        if self.size == 2 {
            self.at(0, 0) * self.at(1, 1) - self.at(0, 1) * self.at(1, 0)
        } else {
            let mut determinant = 0.0;
            for col in 0..self.size {
                determinant += self.at(0, col) * self.cofactor(0, col);
            }
            determinant
        }
    }

    pub fn sub_matrix(&self, row_delete: usize, col_delete: usize) -> Matrix {
        let s = self.size;
        let sub_size = s - 1;
        let mut res: Vec<f64> = Vec::with_capacity(sub_size * sub_size);
        for row in 0..s {
            if row != row_delete {
                for col in 0..s {
                    if col != col_delete {
                        let val = self.at(row, col);
                        res.push(val);
                    }
                }
            }
        }
        Matrix {
            size: sub_size,
            content: res,
        }
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        let sub = self.sub_matrix(row, col);
        sub.determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    pub fn transpose(&self) -> Matrix {
        let s = self.size;
        let mut res: Vec<f64> = Vec::with_capacity(s * s);
        for row in 0..s {
            let row_index = row * s;
            for col in 0..s {
                let val = self.at(col, row); // flip coordinates
                res.insert(col + row_index, val);
            }
        }
        Matrix {
            size: s,
            content: res,
        }
    }

    pub fn multiply(&self, m: &Matrix) -> Matrix {
        let s = self.size;
        let mut res: Vec<f64> = Vec::with_capacity(s * s);
        for row in 0..s {
            let row_index = row * s;
            for col in 0..s {
                let mut val = 0.0;
                for inner in 0..s {
                    val += self.at(row, inner) * m.at(inner, col);
                }
                res.insert(col + row_index, val);
            }
        }
        Matrix {
            size: s,
            content: res,
        }
    }

    pub fn multiply_tuple(&self, t: &Tuple) -> Tuple {
        (
            Matrix::compute_line(
                self.at(0, 0),
                self.at(0, 1),
                self.at(0, 2),
                self.at(0, 3),
                t,
            ),
            Matrix::compute_line(
                self.at(1, 0),
                self.at(1, 1),
                self.at(1, 2),
                self.at(1, 3),
                t,
            ),
            Matrix::compute_line(
                self.at(2, 0),
                self.at(2, 1),
                self.at(2, 2),
                self.at(2, 3),
                t,
            ),
            Matrix::compute_line(
                self.at(3, 0),
                self.at(3, 1),
                self.at(3, 2),
                self.at(3, 3),
                t,
            ),
        )
    }

    // using mul_add https://rust-lang.github.io/rust-clippy/master/index.html#manual_mul_add
    fn compute_line(at1: f64, at2: f64, at3: f64, at4: f64, t: &Tuple) -> f64 {
        at1.mul_add(t.0, at2.mul_add(t.1, at3.mul_add(t.2, at4 * t.3)))
    }

    pub fn rotate_x(angle: f64) -> Matrix {
        let cos_r = angle.cos();
        let sin_r = angle.sin();
        Matrix::make_matrix_4(
            1.0, 0.0, 0.0, 0.0, 0.0, cos_r, -sin_r, 0.0, 0.0, sin_r, cos_r, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn rotate_y(angle: f64) -> Matrix {
        let cos_r = angle.cos();
        let sin_r = angle.sin();
        Matrix::make_matrix_4(
            cos_r, 0.0, sin_r, 0.0, 0.0, 1.0, 0.0, 0.0, -sin_r, 0.0, cos_r, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn rotate_z(angle: f64) -> Matrix {
        let cos_r = angle.cos();
        let sin_r = angle.sin();
        Matrix::make_matrix_4(
            cos_r, -sin_r, 0.0, 0.0, sin_r, cos_r, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
        Matrix::make_matrix_4(
            1.0, xy, xz, 0.0, yx, 1.0, yz, 0.0, zx, zy, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }
}

// structure to cache redundant operations on the transform field
#[derive(Debug, PartialEq, Clone)]
pub struct Transformation {
    pub matrix: Matrix,
    pub inverse: Matrix,
    pub inverse_transpose: Matrix,
}

impl Transformation {
    pub fn default() -> Self {
        Transformation {
            matrix: Matrix::identity(),
            inverse: Matrix::identity(),
            inverse_transpose: Matrix::identity(),
        }
    }

    pub fn make(transform: Matrix) -> Self {
        let inverse = Matrix::inverse(&transform);
        let inverse_transpose = inverse.transpose();
        Transformation {
            matrix: transform,
            inverse,
            inverse_transpose,
        }
    }
}

#[cfg(test)]
mod matrix_tests {
    use crate::matrix::*;
    use crate::tuple::*;

    extern crate quickcheck;

    use self::quickcheck::{Arbitrary, Gen};

    impl Arbitrary for Matrix {
        fn arbitrary(g: &mut Gen) -> Matrix {
            let aa = f64::arbitrary(g);
            let ab = f64::arbitrary(g);
            let ac = f64::arbitrary(g);
            let ad = f64::arbitrary(g);
            let ba = f64::arbitrary(g);
            let bb = f64::arbitrary(g);
            let bc = f64::arbitrary(g);
            let bd = f64::arbitrary(g);
            let ca = f64::arbitrary(g);
            let cb = f64::arbitrary(g);
            let cc = f64::arbitrary(g);
            let cd = f64::arbitrary(g);
            let da = f64::arbitrary(g);
            let db = f64::arbitrary(g);
            let dc = f64::arbitrary(g);
            let dd = f64::arbitrary(g);

            Matrix {
                size: 4,
                content: vec![
                    aa, ab, ac, ad, ba, bb, bc, bd, ca, cb, cc, cd, da, db, dc, dd,
                ],
            }
        }
    }

    #[test]
    fn make_matrix_4_valid() {
        let m = Matrix::make_matrix_4(
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
        );
        assert_eq!(m.at(0, 0), 1.0);
        assert_eq!(m.at(0, 3), 4.0);
        assert_eq!(m.at(1, 0), 5.5);
        assert_eq!(m.at(1, 2), 7.5);
        assert_eq!(m.at(2, 2), 11.0);
        assert_eq!(m.at(3, 0), 13.5);
        assert_eq!(m.at(3, 2), 15.5);
    }

    #[test]
    fn make_matrix_3_valid() {
        let m = Matrix::make_matrix_3(-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0);
        assert_eq!(m.at(0, 0), -3.0);
        assert_eq!(m.at(1, 1), -2.0);
        assert_eq!(m.at(2, 2), 1.0);
    }

    #[test]
    fn make_matrix_2_valid() {
        let m = Matrix::make_matrix_2(-3.0, 5.0, 1.0, -2.0);
        assert_eq!(m.at(0, 0), -3.0);
        assert_eq!(m.at(0, 1), 5.0);
        assert_eq!(m.at(1, 0), 1.0);
        assert_eq!(m.at(1, 1), -2.0);
    }

    #[test]
    fn matrix_equality_valid() {
        let m1 = Matrix::make_matrix_4(
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
        );
        let m2 = Matrix::make_matrix_4(
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
        );
        assert_eq!(m1, m2);
    }

    #[test]
    fn matrix_equality_invalid() {
        let m1 = Matrix::make_matrix_4(
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
        );
        let m2 = Matrix::make_matrix_4(
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.4, 15.5, 16.5,
        ); //changed (3,1)
        assert_ne!(m1, m2);
    }

    #[test]
    fn matrix_multiply() {
        let m1 = Matrix::make_matrix_4(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        );
        let m2 = Matrix::make_matrix_4(
            -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
        );
        let res = m1.multiply(&m2);
        assert_eq!(
            res,
            Matrix::make_matrix_4(
                20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0,
                26.0, 46.0, 42.0
            )
        );
    }

    #[test]
    fn matrix_multiply_by_tuple() {
        let m = Matrix::make_matrix_4(
            1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        );
        let t = (1.0, 2.0, 3.0, 1.0);
        assert_eq!(m.multiply_tuple(&t), (18.0, 24.0, 33.0, 1.0));
    }

    #[test]
    fn matrix_multiply_identity() {
        let m1 = Matrix::make_matrix_4(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        );
        let identity = Matrix::make_matrix_4(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        let res = m1.multiply(&identity);
        assert_eq!(res, m1);
    }

    #[test]
    fn matrix_multiply_identity_bis() {
        let m1 = Matrix::make_matrix_4(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        );
        let identity = Matrix::make_matrix_4(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        let res = identity.multiply(&m1);
        assert_eq!(res, m1);
    }

    //TODO fix float epsilon eq
    //#[quickcheck]
    fn matrix_multiply_identity_prop(m: Matrix) -> bool {
        let identity = Matrix::identity();
        m.multiply(&identity) == m && identity.multiply(&m) == m
    }

    #[test]
    fn matrix_transpose() {
        let m = Matrix::make_matrix_4(
            0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
        );

        let transposed = Matrix::make_matrix_4(
            0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0,
        );
        assert_eq!(m.transpose(), transposed);
    }

    #[test]
    fn matrix_transpose_identity() {
        let identity = Matrix::make_matrix_4(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );

        assert_eq!(identity.transpose(), identity);
    }

    #[test]
    fn matrix_determinant_2() {
        let m = Matrix::make_matrix_2(1.0, 5.0, -3.0, 2.0);

        assert_eq!(m.determinant(), 17.0)
    }

    #[test]
    fn matrix_determinant_3() {
        let m = Matrix::make_matrix_3(1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0);

        assert_eq!(m.determinant(), -196.0)
    }

    #[test]
    fn matrix_determinant_4() {
        let m = Matrix::make_matrix_4(
            -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0,
        );

        assert_eq!(m.determinant(), -4071.0)
    }

    #[test]
    fn matrix_sub_matrix_4_to_3() {
        let m1 = Matrix::make_matrix_4(
            -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
        );
        let expected_sub = Matrix::make_matrix_3(-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0);
        assert_eq!(m1.sub_matrix(2, 1), expected_sub);
    }

    #[test]
    fn matrix_minor() {
        let m = Matrix::make_matrix_3(3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0);

        assert_eq!(m.minor(1, 0), 25.0)
    }

    #[test]
    fn matrix_cofactor() {
        let m = Matrix::make_matrix_3(3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0);

        assert_eq!(m.minor(0, 0), -12.0);
        assert_eq!(m.cofactor(0, 0), -12.0);
        assert_eq!(m.minor(1, 0), 25.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn matrix_inversion() {
        let m1 = Matrix::make_matrix_4(
            8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0,
        );
        let expected_inverse = Matrix::make_matrix_4(
            -0.15384615384615385,
            -0.15384615384615385,
            -0.28205128205128205,
            -0.5384615384615384,
            -0.07692307692307693,
            0.12307692307692308,
            0.02564102564102564,
            0.03076923076923077,
            0.358974358974359,
            0.358974358974359,
            0.4358974358974359,
            0.9230769230769231,
            -0.6923076923076923,
            -0.6923076923076923,
            -0.7692307692307693,
            -1.9230769230769231,
        );
        assert_eq!(m1.inverse(), expected_inverse);
    }

    #[test]
    fn matrix_inversion_bis() {
        let m1 = Matrix::make_matrix_4(
            9.0, 3.0, 0.0, 9.0, -5.0, -2.0, -6.0, -3.0, -4.0, 9.0, 6.0, 4.0, -7.0, 6.0, 6.0, 2.0,
        );
        let expected_inverse = Matrix::make_matrix_4(
            -0.040740740740740744,
            -0.07777777777777778,
            0.14444444444444443,
            -0.2222222222222222,
            -0.07777777777777778,
            0.03333333333333333,
            0.36666666666666664,
            -0.3333333333333333,
            -0.029012345679012345,
            -0.14629629629629629,
            -0.10925925925925926,
            0.12962962962962962,
            0.17777777777777778,
            0.06666666666666667,
            -0.26666666666666666,
            0.3333333333333333,
        );
        assert_eq!(m1.inverse(), expected_inverse);
    }

    #[test]
    fn matrix_invert_identity() {
        let identity = Matrix::make_matrix_4(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );

        assert_eq!(identity.inverse(), identity);
    }

    #[test]
    fn matrix_translation_point() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);
        let translated = transform.multiply_tuple(&p);
        assert_eq!(translated, point(2.0, 1.0, 7.0));
    }

    #[test]
    fn matrix_inverse_translation_point() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let inverse_transform = transform.inverse();
        let p = point(-3.0, 4.0, 5.0);
        let translated = inverse_transform.multiply_tuple(&p);
        assert_eq!(translated, point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn matrix_translation_vector_no_effect() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let v = vector(-3.0, 4.0, 5.0);
        let translated = transform.multiply_tuple(&v);
        assert_eq!(translated, v);
    }

    #[test]
    fn matrix_scaling_point() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let p = point(-4.0, 6.0, 8.0);
        let scaled = transform.multiply_tuple(&p);
        assert_eq!(scaled, point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn matrix_scaling_vector() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let p = vector(-4.0, 6.0, 8.0);
        let scaled = transform.multiply_tuple(&p);
        assert_eq!(scaled, vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn matrix_scaling_inverse() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let inverse = transform.inverse();
        let v = vector(-4.0, 6.0, 8.0);
        let shrinked = inverse.multiply_tuple(&v);
        assert_eq!(shrinked, vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn matrix_reflection_via_negative_scaling() {
        let transform = Matrix::scaling(-1.0, 1.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        let scaled = transform.multiply_tuple(&p);
        assert_eq!(scaled, point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotate_x(std::f64::consts::FRAC_PI_4);
        let full_quarter = Matrix::rotate_x(std::f64::consts::FRAC_PI_2);
        assert_eq!(
            half_quarter.multiply_tuple(&p),
            point(0.0, 0.7071067811865476, 0.7071067811865475)
        );
        assert_eq!(
            full_quarter.multiply_tuple(&p),
            point(0.0, 0.00000000000000006123233995736766, 1.0)
        );
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let p = point(0.0, 0.0, 1.0);
        let half_quarter = Matrix::rotate_y(std::f64::consts::FRAC_PI_4);
        let full_quarter = Matrix::rotate_y(std::f64::consts::FRAC_PI_2);
        assert_eq!(
            half_quarter.multiply_tuple(&p),
            point(0.7071067811865475, 0.0, 0.7071067811865476)
        );
        assert_eq!(
            full_quarter.multiply_tuple(&p),
            point(1.0, 0.0, 0.00000000000000006123233995736766)
        );
    }

    #[test]
    fn rotating_point_around_z_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotate_z(std::f64::consts::FRAC_PI_4);
        let full_quarter = Matrix::rotate_z(std::f64::consts::FRAC_PI_2);
        assert_eq!(
            half_quarter.multiply_tuple(&p),
            point(-0.7071067811865475, 0.7071067811865476, 0.0)
        );
        assert_eq!(
            full_quarter.multiply_tuple(&p),
            point(-1.0, 0.00000000000000006123233995736766, 0.0)
        );
    }

    #[test]
    fn shearing_moves_z_proportion_to_y() {
        let shear = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(shear.multiply_tuple(&p), point(2.0, 3.0, 7.0))
    }

    #[test]
    fn transformations_in_sequence() {
        let p1 = point(1.0, 0.0, 1.0);
        let rot = Matrix::rotate_x(std::f64::consts::FRAC_PI_2);
        let scaling = Matrix::scaling(5.0, 5.0, 5.0);
        let trans = Matrix::translation(10.0, 5.0, 7.0);

        let p2 = rot.multiply_tuple(&p1);
        assert_eq!(p2, point(1.0, -1.0, 0.000000000000000061232339957367660));

        let p3 = scaling.multiply_tuple(&p2);
        assert_eq!(p3, point(5.0, -5.0, 0.0000000000000003061616997868383));

        let p4 = trans.multiply_tuple(&p3);
        assert_eq!(p4, point(15.0, 0.0, 7.0));
    }

    #[test]
    fn transformations_chained_in_reverse() {
        let p = point(1.0, 0.0, 1.0);

        let rot = Matrix::rotate_x(std::f64::consts::FRAC_PI_2);
        let scaling = Matrix::scaling(5.0, 5.0, 5.0);
        let trans = Matrix::translation(10.0, 5.0, 7.0);

        let chain = trans.multiply(&scaling).multiply(&rot);

        let p1 = chain.multiply_tuple(&p);
        assert_eq!(p1, point(15.0, 0.0, 7.0));
    }
}
