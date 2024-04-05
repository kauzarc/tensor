use super::*;

struct Parameter<MatrixType>(Tensor<MatrixType, NoBackpropagation>);

impl<MatrixType> From<Tensor<MatrixType, NoBackpropagation>> for Parameter<MatrixType> {
    fn from(value: Tensor<MatrixType, NoBackpropagation>) -> Self {
        Self(value)
    }
}

impl<MatrixType> ops::Deref for Parameter<MatrixType> {
    type Target = Tensor<MatrixType, NoBackpropagation>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<MatrixType> ops::DerefMut for Parameter<MatrixType> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
