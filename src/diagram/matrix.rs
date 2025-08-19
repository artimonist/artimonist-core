#![allow(deprecated)]

/// type alias for any diagram
#[deprecated(since = "1.7.2", note = "use `Diagram` trait instead")]
pub type Matrix<T, const H: usize = 7, const W: usize = 7> = [[Option<T>; W]; H];

/// transform to generic diagram
#[deprecated(since = "1.7.2", note = "use `Diagram` trait instead")]
pub trait ToMatrix<T> {
    /// transform to matrix
    fn to_matrix<const H: usize, const W: usize>(self) -> Matrix<T, H, W>;
}

impl<T> ToMatrix<T> for Vec<Option<T>> {
    fn to_matrix<const H: usize, const W: usize>(mut self) -> Matrix<T, H, W> {
        self.resize_with(H * W, || None);
        self.reverse();
        core::array::from_fn(|_| core::array::from_fn(|_| self.pop().unwrap()))
    }
}

impl<T: std::fmt::Debug> ToMatrix<T> for Vec<Vec<Option<T>>> {
    fn to_matrix<const H: usize, const W: usize>(mut self) -> Matrix<T, H, W> {
        self.resize_with(H, || [const { None }; W].into_iter().collect());
        self.into_iter()
            .map(|mut r| {
                r.resize_with(W, || None);
                r.try_into().unwrap()
            })
            .take(H)
            .collect::<Vec<[Option<T>; W]>>()
            .try_into()
            .unwrap()
    }
}

impl<T: std::fmt::Debug + Default + PartialEq> ToMatrix<T> for Vec<T> {
    fn to_matrix<const H: usize, const W: usize>(mut self) -> Matrix<T, H, W> {
        self.resize_with(H * W, T::default);
        self.reverse();
        core::array::from_fn(|_| {
            core::array::from_fn(|_| self.pop().filter(|v| *v != T::default()))
        })
    }
}

impl<T: std::fmt::Debug + Default + PartialEq> ToMatrix<T> for Vec<Vec<T>> {
    fn to_matrix<const H: usize, const W: usize>(mut self) -> Matrix<T, H, W> {
        self.resize_with(H, || (0..W).map(|_| T::default()).collect());
        self.into_iter()
            .map(|mut r| {
                r.resize_with(W, T::default);
                r.into_iter()
                    .map(|v| if v == T::default() { None } else { Some(v) })
                    .collect::<Vec<Option<T>>>()
                    .try_into()
                    .unwrap()
            })
            .take(H)
            .collect::<Vec<[Option<T>; W]>>()
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod matrix_test {
    use crate::ToMatrix;

    #[test]
    fn test_vector() {
        const MX: [[Option<u8>; 3]; 3] = [
            [Some(1), Some(2), Some(3)],
            [Some(4), Some(5), Some(6)],
            [None, None, None],
        ];
        let vs: Vec<Option<_>> = (1u8..=6).map(|i| Some(i)).collect();
        assert_eq!(vs.to_matrix(), MX);
    }

    #[test]
    fn test_matrix() {
        const MX: [[Option<u8>; 3]; 3] = [
            [Some(1), Some(2), Some(3)],
            [Some(4), Some(5), Some(6)],
            [None, None, None],
        ];
        let mvs = vec![
            vec![Some(1), Some(2), Some(3)],
            vec![Some(4), Some(5), Some(6)],
            vec![None, None, None],
        ];
        assert_eq!(mvs.to_matrix(), MX);
    }

    #[test]
    fn test_vec_default() {
        const MX: [[Option<u8>; 3]; 3] = [
            [None, Some(1), Some(2)],
            [Some(3), Some(4), Some(5)],
            [Some(6), None, None],
        ];
        let vs: Vec<_> = (0..=6).map(|i| i).collect();
        assert_eq!(vs.to_matrix(), MX);
    }

    #[test]
    fn test_matrix_default() {
        const MX: [[Option<u8>; 3]; 3] = [
            [Some(1), Some(2), None],
            [Some(4), Some(5), Some(6)],
            [None, None, None],
        ];
        let vs: Vec<Vec<u8>> = vec![vec![1, 2, 0], vec![4, 5, 6]];
        assert_eq!(vs.to_matrix(), MX);
    }
}
