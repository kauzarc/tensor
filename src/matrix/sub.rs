use super::Matrix;
use std::ops;

impl<T, const M: usize, const N: usize> ops::Sub for Matrix<T, M, N>
where
    [T; M * N]: Sized,
    T: ops::Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        unsafe {
            Self(
                self.0
                    .into_iter()
                    .zip(rhs.0.into_iter())
                    .map(|(a, b)| a - b)
                    .collect::<Vec<T>>()
                    .try_into()
                    .unwrap_unchecked(),
            )
        }
    }
}

impl<T, const M: usize, const N: usize> ops::Sub for &Matrix<T, M, N>
where
    [T; M * N]: Sized,
    for<'a> &'a T: ops::Sub<Output = T>,
{
    type Output = Matrix<T, M, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        unsafe {
            Matrix(
                self.0
                    .iter()
                    .zip(rhs.0.iter())
                    .map(|(a, b)| a - b)
                    .collect::<Vec<T>>()
                    .try_into()
                    .unwrap_unchecked(),
            )
        }
    }
}

impl<T, const M: usize, const N: usize> ops::SubAssign for Matrix<T, M, N>
where
    [T; M * N]: Sized,
    T: ops::SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.0
            .iter_mut()
            .zip(rhs.0.into_iter())
            .for_each(|(a, b)| *a -= b)
    }
}

impl<T, const M: usize, const N: usize> ops::SubAssign<&Matrix<T, M, N>> for Matrix<T, M, N>
where
    [T; M * N]: Sized,
    T: for<'a> ops::SubAssign<&'a T>,
{
    fn sub_assign(&mut self, rhs: &Matrix<T, M, N>) {
        self.0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(a, b)| *a -= b)
    }
}
