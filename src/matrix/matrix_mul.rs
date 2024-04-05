use super::Matrix;

pub trait MatrixMul<T> {
    type Output;

    fn matrix_mul(self, rhs: T) -> Self::Output;
}

impl<T, const M: usize, const N: usize, const P: usize> MatrixMul<Matrix<T, N, P>>
    for Matrix<T, M, N>
where
    [T; M * N]: Sized,
    [T; N * P]: Sized,
    [T; M * P]: Sized,
    T: num::Num + Clone,
{
    type Output = Matrix<T, M, P>;

    fn matrix_mul(self, rhs: Matrix<T, N, P>) -> Self::Output {
        let mut result: Self::Output = Matrix::zeros();

        for i in 0..M {
            for j in 0..P {
                for k in 0..N {
                    result[(i, j)] =
                        result[(i, j)].clone() + self[(i, k)].clone() * rhs[(k, j)].clone();
                }
            }
        }

        result
    }
}

impl<T, const M: usize, const N: usize, const P: usize> MatrixMul<&Matrix<T, N, P>>
    for &Matrix<T, M, N>
where
    [T; M * N]: Sized,
    [T; N * P]: Sized,
    [T; M * P]: Sized,
    T: num::Zero + Clone,
    for<'a> &'a T: num::traits::NumOps<&'a T, T>,
{
    type Output = Matrix<T, M, P>;

    fn matrix_mul(self, rhs: &Matrix<T, N, P>) -> Self::Output {
        let mut result: Self::Output = Matrix::zeros();

        for i in 0..M {
            for j in 0..P {
                for k in 0..N {
                    let temp = &self[(i, k)] * &rhs[(k, j)];
                    result[(i, j)] = &result[(i, j)] + &temp;
                }
            }
        }

        result
    }
}
