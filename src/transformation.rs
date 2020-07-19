use crate::matrix::Matrix;
use crate::tuple::{subtract_tuple, vector_cross_product, vector_normalize, Tuple};

// from: position of the eye
// to: point of the scene to look at
// up: indicating which direction is up
// returns the corresponding transformation matrix
pub fn view_transform(from: &Tuple, to: &Tuple, up: &Tuple) -> Matrix {
    let forward = vector_normalize(&subtract_tuple(to, from));
    let upn = vector_normalize(up);
    let left = vector_cross_product(&forward, &upn);
    let true_up = vector_cross_product(&left, &forward);
    let orientation = Matrix::make_matrix_4(
        left.0, left.1, left.2, 0.0, true_up.0, true_up.1, true_up.2, 0.0, -forward.0, -forward.1,
        -forward.2, 0.0, 0.0, 0.0, 0.0, 1.0,
    );
    let translation = Matrix::translation(-from.0, -from.1, -from.2);
    orientation.multiply(&translation)
}

#[cfg(test)]
mod transformation_tests {
    use crate::matrix::Matrix;
    use crate::transformation::view_transform;
    use crate::tuple::*;

    #[test]
    fn transformation_for_default_orientation() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, -1.0);
        let up = vector(0.0, 1.0, 0.0);
        let t = view_transform(&from, &to, &up);
        assert_eq!(t, Matrix::identity());
    }

    #[test]
    fn transformation_looking_in_positive_z() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, 1.0);
        let up = vector(0.0, 1.0, 0.0);
        let t = view_transform(&from, &to, &up);
        assert_eq!(t, Matrix::scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn transformation_moves_the_world() {
        let from = point(0.0, 0.0, 8.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        let t = view_transform(&from, &to, &up);
        assert_eq!(t, Matrix::translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn arbitrary_transformation() {
        let from = point(1.0, 3.0, 2.0);
        let to = point(4.0, -2.0, 8.0);
        let up = vector(1.0, 1.0, 0.0);
        let t = view_transform(&from, &to, &up);
        let expected = Matrix::make_matrix_4(
            -0.5070925528371099,
            0.5070925528371099,
            0.6761234037828132,
            -2.366431913239846,
            0.7677159338596801,
            0.6060915267313263,
            0.12121830534626524,
            -2.8284271247461894,
            -0.35856858280031806,
            0.5976143046671968,
            -0.7171371656006361,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );
        assert_eq!(t, expected);
    }
}
