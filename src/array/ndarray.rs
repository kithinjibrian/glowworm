#[derive(Debug, Clone, PartialEq)]
pub struct Array<T> {
    pub data: Vec<T>,
    pub shape: Vec<usize>,
}

impl<T> Array<T> {
    pub fn new(data: Vec<T>, shape: Vec<usize>) -> Self {
        let expected_len: usize = shape.iter().product();
        assert_eq!(
            data.len(),
            expected_len,
            "Data length {} doesn't match shape {:?} (expected {})",
            data.len(),
            shape,
            expected_len
        );
        Self { data, shape }
    }

    pub fn ndim(&self) -> usize {
        self.shape.len()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn reshape(&mut self, new_shape: Vec<usize>) {
        let new_size: usize = new_shape.iter().product();
        assert_eq!(
            self.data.len(),
            new_size,
            "Cannot reshape array of size {} into shape {:?}",
            self.data.len(),
            new_shape
        );
        self.shape = new_shape;
    }
}

impl<T: Clone> Array<T> {
    pub fn zeros(shape: Vec<usize>) -> Self
    where
        T: Default,
    {
        let size: usize = shape.iter().product();
        let data = vec![T::default(); size];
        Self { data, shape }
    }

    pub fn full(shape: Vec<usize>, fill_value: T) -> Self {
        let size: usize = shape.iter().product();
        let data = vec![fill_value; size];
        Self { data, shape }
    }
}
