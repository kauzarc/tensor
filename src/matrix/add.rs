use super::Matrix;
use std::ops;

impl<T, const M: usize, const N: usize> ops::Add for Matrix<T, M, N>
where
    [T; M * N]: Sized,
    T: ops::Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        unsafe {
            Self(
                self.0
                    .into_iter()
                    .zip(rhs.0.into_iter())
                    .map(|(a, b)| a + b)
                    .collect::<Vec<T>>()
                    .try_into()
                    .unwrap_unchecked(),
            )
        }
    }
}

impl<T, const M: usize, const N: usize> ops::Add for &Matrix<T, M, N>
where
    [T; M * N]: Sized,
    for<'a> &'a T: ops::Add<Output = T>,
{
    type Output = Matrix<T, M, N>;

    fn add(self, rhs: Self) -> Self::Output {
        unsafe {
            Matrix(
                self.0
                    .iter()
                    .zip(rhs.0.iter())
                    .map(|(a, b)| a + b)
                    .collect::<Vec<T>>()
                    .try_into()
                    .unwrap_unchecked(),
            )
        }
    }
}

impl<T, const M: usize, const N: usize> ops::AddAssign for Matrix<T, M, N>
where
    [T; M * N]: Sized,
    T: ops::AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.0
            .iter_mut()
            .zip(rhs.0.into_iter())
            .for_each(|(a, b)| *a += b)
    }
}

impl<T, const M: usize, const N: usize> ops::AddAssign<&Matrix<T, M, N>> for Matrix<T, M, N>
where
    [T; M * N]: Sized,
    T: for<'a> ops::AddAssign<&'a T>,
{
    fn add_assign(&mut self, rhs: &Matrix<T, M, N>) {
        self.0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(a, b)| *a += b)
    }
}
