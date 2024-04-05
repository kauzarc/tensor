use std::mem;

#[derive(Debug, Clone)]
pub struct Matrix<T, const M: usize, const N: usize>([T; M * N])
where
    [T; M * N]: Sized;

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    [T; M * N]: Sized,
{
    pub fn get(&self, i: usize, j: usize) -> Option<&T> {
        if !Self::ij_in_bounds(i, j) {
            None
        } else {
            unsafe { Some(self.0.get_unchecked(Self::ij_to_i(i, j))) }
        }
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
        if !Self::ij_in_bounds(i, j) {
            None
        } else {
            unsafe { Some(self.0.get_unchecked_mut(Self::ij_to_i(i, j))) }
        }
    }

    fn ij_in_bounds(i: usize, j: usize) -> bool {
        i < N && j < M
    }

    fn ij_to_i(i: usize, j: usize) -> usize {
        i * N + j
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    [T; M * N]: Sized,
    T: num::Zero,
{
    pub fn zeros() -> Self {
        let mut res: [T; M * N] = unsafe { mem::MaybeUninit::uninit().assume_init() };
        res.iter_mut().for_each(|x| *x = T::zero());

        Self(res)
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    [T; M * N]: Sized,
    T: num::One,
{
    pub fn ones() -> Self {
        let mut res: [T; M * N] = unsafe { mem::MaybeUninit::uninit().assume_init() };
        res.iter_mut().for_each(|x| *x = T::one());

        Self(res)
    }
}

impl<T, const M: usize, const N: usize> From<[T; M * N]> for Matrix<T, M, N> {
    fn from(value: [T; M * N]) -> Self {
        Self(value)
    }
}

pub mod add;
pub mod index;
pub mod matrix_mul;
pub mod mul;
pub mod sub;
