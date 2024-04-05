use super::Matrix;
use std::ops;

impl<T, const M: usize, const N: usize> ops::Mul for Matrix<T, M, N>
where
    [T; M * N]: Sized,
    T: ops::Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        unsafe {
            Self(
                self.0
                    .into_iter()
                    .zip(rhs.0.into_iter())
                    .map(|(a, b)| a * b)
                    .collect::<Vec<T>>()
                    .try_into()
                    .unwrap_unchecked(),
            )
        }
    }
}

impl<T, const M: usize, const N: usize> ops::Mul for &Matrix<T, M, N>
where
    [T; M * N]: Sized,
    for<'a> &'a T: ops::Mul<Output = T>,
{
    type Output = Matrix<T, M, N>;

    fn mul(self, rhs: Self) -> Self::Output {
        unsafe {
            Matrix(
                self.0
                    .iter()
                    .zip(rhs.0.iter())
                    .map(|(a, b)| a * b)
                    .collect::<Vec<T>>()
                    .try_into()
                    .unwrap_unchecked(),
            )
        }
    }
}

impl<T, const M: usize, const N: usize> ops::MulAssign for Matrix<T, M, N>
where
    [T; M * N]: Sized,
    T: ops::MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.0
            .iter_mut()
            .zip(rhs.0.into_iter())
            .for_each(|(a, b)| *a *= b)
    }
}

impl<T, const M: usize, const N: usize> ops::MulAssign<&Matrix<T, M, N>> for Matrix<T, M, N>
where
    [T; M * N]: Sized,
    T: for<'a> ops::MulAssign<&'a T>,
{
    fn mul_assign(&mut self, rhs: &Matrix<T, M, N>) {
        self.0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(a, b)| *a *= b)
    }
}
