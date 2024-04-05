use crate::matrix::Matrix;
use std::{cell, ops, rc};

pub mod parameter;

struct NoBackpropagation;

struct OneBackPtr<T1, F>
where
    F: Fn(&mut T1),
{
    back_ptr: rc::Rc<cell::RefCell<T1>>,
    g: F,
}

pub struct Tensor<MatrixType, BackpropagationType> {
    data: MatrixType,
    gradient: Option<MatrixType>,
    backpropagation: BackpropagationType,
}

impl<T, const M: usize, const N: usize> From<Matrix<T, M, N>>
    for Tensor<Matrix<T, M, N>, NoBackpropagation>
where
    [T; M * N]: Sized,
{
    fn from(value: Matrix<T, M, N>) -> Self {
        Self {
            data: value,
            gradient: None,
            backpropagation: NoBackpropagation,
        }
    }
}

pub trait Gradient {
    type GradientType;

    fn get_gradient(&self) -> &Option<Self::GradientType>;
}

impl<MatrixType, BackpropagationType> Gradient for Tensor<MatrixType, BackpropagationType> {
    type GradientType = MatrixType;

    fn get_gradient(&self) -> &Option<Self::GradientType> {
        &self.gradient
    }
}

pub trait AddGradient: Gradient {
    fn add_gradient(&mut self, gradient: Self::GradientType);
}

impl<MatrixType, BackpropagationType> AddGradient for Tensor<MatrixType, BackpropagationType>
where
    MatrixType: ops::AddAssign,
{
    fn add_gradient(&mut self, gradient: Self::GradientType) {
        match &mut self.gradient {
            Some(value) => *value += gradient,
            None => self.gradient = Some(gradient),
        }
    }
}

pub trait Backward {
    fn backward(&self);
}

impl<MatrixType> Backward for Tensor<MatrixType, NoBackpropagation> {
    fn backward(&self) {}
}

impl<MatrixType, T1, F> Backward for Tensor<MatrixType, OneBackPtr<T1, F>>
where
    T1: AddGradient + Backward,
    F: Fn(&mut T1),
{
    fn backward(&self) {
        (self.backpropagation.g)(&mut self.backpropagation.back_ptr.borrow_mut());
        self.backpropagation.back_ptr.borrow().backward();
    }
}
