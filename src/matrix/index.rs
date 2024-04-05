use super::Matrix;
use std::ops;

impl<T, const M: usize, const N: usize> ops::Index<(usize, usize)> for Matrix<T, M, N>
where
    [T; M * N]: Sized,
{
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        if !Self::ij_in_bounds(i, j) {
            panic!("index ({i}, {j}) out of reach for shape ({M}, {N})")
        } else {
            unsafe { self.0.get_unchecked(Self::ij_to_i(i, j)) }
        }
    }
}

impl<T, const M: usize, const N: usize> ops::IndexMut<(usize, usize)> for Matrix<T, M, N>
where
    [T; M * N]: Sized,
{
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        if !Self::ij_in_bounds(i, j) {
            panic!("index ({i}, {j}) out of reach for shape ({M}, {N})")
        } else {
            unsafe { self.0.get_unchecked_mut(Self::ij_to_i(i, j)) }
        }
    }
}
