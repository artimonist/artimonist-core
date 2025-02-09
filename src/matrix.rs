/// type alias for any diagram
pub type Matrix<const H: usize, const W: usize, T> = [[Option<T>; W]; H];

/// transform to generic diagram
pub trait ToMatrix<T> {
    /// transform to matrix
    fn to_matrix<const H: usize, const W: usize>(self) -> Matrix<H, W, T>;
}

impl<T> ToMatrix<T> for Vec<Option<T>> {
    fn to_matrix<const H: usize, const W: usize>(mut self) -> Matrix<H, W, T> {
        self.resize_with(H * W, || None);
        self.reverse();
        core::array::from_fn(|_| core::array::from_fn(|_| self.pop().unwrap()))
    }
}

impl<T: std::fmt::Debug> ToMatrix<T> for Vec<Vec<Option<T>>> {
    fn to_matrix<const H: usize, const W: usize>(mut self) -> Matrix<H, W, T> {
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

#[cfg(test)]
mod matrix_test {
    use crate::ToMatrix;

    #[test]
    fn test_matrix() {
        const MX: [[Option<u8>; 3]; 3] = [
            [Some(1), Some(2), Some(3)],
            [Some(4), Some(5), Some(6)],
            [None, None, None],
        ];
        let vs: Vec<Option<_>> = (1u8..=6).map(|i| Some(i)).collect();
        assert_eq!(vs.to_matrix(), MX);
    }
}
