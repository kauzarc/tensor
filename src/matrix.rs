use std::ops;

pub struct Matrix<T, const M: usize, const N: usize>([T; M * N])
where
    [T; M * N]: Sized,
    T: num::Num + Copy;

impl<T, const M: usize, const N: usize> From<[T; M * N]> for Matrix<T, M, N>
where
    [T; M * N]: Sized,
    T: num::Num + Copy,
{
    fn from(value: [T; M * N]) -> Self {
        Self(value)
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    [T; M * N]: Sized,
    T: num::Num + Copy,
{
    fn apply_ops<F>(&self, rhs: &Self, ops: F) -> Self
    where
        F: Fn(T, T) -> T,
    {
        unsafe {
            Matrix(
                self.0
                    .iter()
                    .cloned()
                    .zip(rhs.0.iter().cloned())
                    .map(|(a, b)| ops(a, b))
                    .collect::<Vec<T>>()
                    .try_into()
                    .unwrap_unchecked(),
            )
        }
    }

    fn ij_to_i(i: usize, j: usize) -> Option<usize> {
        if i >= M || j >= N {
            None
        } else {
            Some(unsafe { Self::ij_to_i_unchecked(i, j) })
        }
    }

    unsafe fn ij_to_i_unchecked(i: usize, j: usize) -> usize {
        i * N + j
    }

    pub unsafe fn get_unchecked(&self, i: usize, j: usize) -> &T {
        self.0.get_unchecked(Self::ij_to_i_unchecked(i, j))
    }

    pub unsafe fn get_unchecked_mut(&mut self, i: usize, j: usize) -> &mut T {
        self.0.get_unchecked_mut(Self::ij_to_i_unchecked(i, j))
    }
}

impl<T, const M: usize, const N: usize> ops::Add for &Matrix<T, M, N>
where
    [T; M * N]: Sized,
    T: num::Num + Copy,
{
    type Output = Matrix<T, M, N>;

    fn add(self, rhs: Self) -> Self::Output {
        self.apply_ops(rhs, T::add)
    }
}

impl<T, const M: usize, const N: usize> ops::Sub for &Matrix<T, M, N>
where
    [T; M * N]: Sized,
    T: num::Num + Copy,
{
    type Output = Matrix<T, M, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.apply_ops(rhs, T::sub)
    }
}

impl<T, const M: usize, const N: usize> ops::Mul for &Matrix<T, M, N>
where
    [T; M * N]: Sized,
    T: num::Num + Copy,
{
    type Output = Matrix<T, M, N>;

    fn mul(self, rhs: Self) -> Self::Output {
        self.apply_ops(rhs, T::mul)
    }
}

impl<T, const M: usize, const N: usize> ops::Div for &Matrix<T, M, N>
where
    [T; M * N]: Sized,
    T: num::Num + Copy,
{
    type Output = Matrix<T, M, N>;

    fn div(self, rhs: Self) -> Self::Output {
        self.apply_ops(rhs, T::div)
    }
}

impl<T, const M: usize, const N: usize> ops::Index<(usize, usize)> for Matrix<T, M, N>
where
    [T; M * N]: Sized,
    T: num::Num + Copy,
{
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        match Self::ij_to_i(i, j) {
            None => panic!("Index ({i}, {j}) out of bounds for Matrix<{M}, {N}>."),
            Some(i) => unsafe { self.0.get_unchecked(i) },
        }
    }
}

impl<T, const M: usize, const N: usize> ops::IndexMut<(usize, usize)> for Matrix<T, M, N>
where
    [T; M * N]: Sized,
    T: num::Num + Copy,
{
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        match Self::ij_to_i(i, j) {
            None => panic!("Index ({i}, {j}) out of bounds for Matrix<{M}, {N}>."),
            Some(i) => unsafe { self.0.get_unchecked_mut(i) },
        }
    }
}

pub trait MatrixMul<Rhs> {
    type Output;
    fn matrix_mul(self, rhs: Rhs) -> Self::Output;
}

impl<T, const M: usize, const N: usize, const P: usize> MatrixMul<&Matrix<T, N, P>>
    for &Matrix<T, M, N>
where
    [T; M * N]: Sized,
    [T; M * P]: Sized,
    [T; N * P]: Sized,
    T: num::Num + Copy,
{
    type Output = Matrix<T, M, P>;

    fn matrix_mul(self, rhs: &Matrix<T, N, P>) -> Self::Output {
        let mut res = Matrix([T::zero(); M * P]);

        for i in 0..M {
            for j in 0..P {
                for k in 0..N {
                    unsafe {
                        *res.get_unchecked_mut(i, j) = *res.get_unchecked(i, j)
                            + *self.get_unchecked(i, k) * *rhs.get_unchecked(k, j);
                    }
                }
            }
        }

        res
    }
}
