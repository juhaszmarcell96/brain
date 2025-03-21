use std::ops::{Add, Sub, Mul, Div, Index, IndexMut, MulAssign, DivAssign}; // +, -, *, /, [], *=, /=
use std::fmt;

/*
nxm = 2x3
    a_00 a_01 a_02
    a_10 a_11 a_12
*/

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T> {
    data: Vec<Vec<T>>,
}

impl<T> Matrix<T>
where
    T: Default + Clone
{
    // constructor
    pub fn new(rows: usize, cols: usize) -> Self {
        if rows == 0 || cols == 0 {
            panic!("matrix dimensions cannot be zero");
        }
        Self {
            data: vec![vec![T::default(); cols]; rows],
        }
    }
}

impl<T> Matrix<T>
where
    T: Copy
{
    // getter for the number of rows
    pub fn rows(&self) -> usize {
        self.data.len()
    }

    // getter for the number of columns
    pub fn cols(&self) -> usize {
        if self.data.is_empty() {
            0
        }
        else {
            self.data[0].len()
        }
    }

    // get element [row][col]
    pub fn get(&self, row: usize, col: usize) -> T {
        self.data[row][col]
    }

    // set element [row][col]
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.data[row][col] = value;
    }

    pub fn assert_valid_element(&self, row: usize, col: usize) {
        assert!(row < self.rows() && col < self.cols(), "Invalid matrix indices");
    }
}

// Index trait []
impl<T> Index<usize> for Matrix<T> {
    type Output = [T];
    fn index(&self, row: usize) -> &Self::Output {
        &self.data[row]
    }
}

// IndexMut trait []
impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, row: usize) -> &mut[T] {
        &mut self.data[row]
    }
}

// Add trait Matrix + Matrix
impl<T> Add for Matrix<T>
where T: Add<Output=T> + Default + Copy
{
    type Output = Matrix<T>;
    fn add(self, other: Self) -> Self::Output {
        assert_eq!(self.rows(), other.rows(), "matrix dimensions must match for addition");
        assert_eq!(self.cols(), other.cols(), "matrix dimensions must match for addition");
        let mut result = Matrix::new(self.rows(), self.cols());
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        result
    }
}
impl<T> Add<&Matrix<T>> for &Matrix<T>
where T: Add<Output=T> + Default + Copy
{
    type Output = Matrix<T>;
    fn add(self, other: &Matrix<T>) -> Self::Output {
        assert_eq!(self.rows(), other.rows(), "matrix dimensions must match for addition");
        assert_eq!(self.cols(), other.cols(), "matrix dimensions must match for addition");
        let mut result = Matrix::new(self.rows(), self.cols());
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        result
    }
}


// Sub trait Matrix - Matrix
impl<T> Sub for Matrix<T>
where T : Sub<Output=T> + Default + Copy
{
    type Output = Matrix<T>;
    fn sub(self, other: Self) -> Self::Output {
        assert_eq!(self.rows(), other.rows(), "matrix dimensions must match for subtraction");
        assert_eq!(self.cols(), other.cols(), "matrix dimensions must match for subtraction");
        let mut result = Matrix::new(self.rows(), self.cols());
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                result.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        result
    }
}
impl<T> Sub<&Matrix<T>> for &Matrix<T>
where T : Sub<Output=T> + Default + Copy
{
    type Output = Matrix<T>;
    fn sub(self, other: &Matrix<T>) -> Self::Output {
        assert_eq!(self.rows(), other.rows(), "matrix dimensions must match for subtraction");
        assert_eq!(self.cols(), other.cols(), "matrix dimensions must match for subtraction");
        let mut result = Matrix::new(self.rows(), self.cols());
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                result.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        result
        //Matrix { data: self.data.iter().zip(&other.data).map(|(a, b)| *a - *b).collect() }
    }
}

// Mul trait Matrix * scalar
impl<T> Mul<T> for Matrix<T>
where T: Mul<Output=T> + Default + Copy
{
    type Output = Matrix<T>;
    fn mul(self, scalar: T) -> Self::Output {
        let mut result = Matrix::new(self.rows(), self.cols());
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                result.data[i][j] = self.data[i][j] * scalar;
            }
        }
        result
    }
    //fn mul(self, scalar: T) -> Self::Output {
    //    let data = self.data.into_iter()
    //        .map(|row| row.into_iter().map(|x| x * scalar).collect())
    //        .collect();
    //    Matrix { data }
    //}
}
// Mul trait Matrix * Matrix
impl<T> Mul for Matrix<T>
where
    T: Mul<Output=T> + Add<Output=T>,
    T: Default + Copy
{
    type Output = Matrix<T>;
    fn mul(self, other: Self) -> Self::Output {
        assert_eq!(self.cols(), other.rows(), "matrix dimensions must match for multiplication");
        let mut result = Matrix::new(self.rows(), other.cols());
        for i in 0..self.rows() {
            for j in 0..other.cols() {
                let mut sum = T::default();
                for k in 0..self.cols() {
                    sum = sum + (self.data[i][k] * other.data[k][j]);
                }
                result.data[i][j] = sum;
            }
        }
        result
    }
}
impl<T> Mul<&Matrix<T>> for &Matrix<T>
where
    T: Mul<Output=T> + Add<Output=T>,
    T: Default + Copy
{
    type Output = Matrix<T>;
    fn mul(self, other: &Matrix<T>) -> Self::Output {
        assert_eq!(self.cols(), other.rows(), "matrix dimensions must match for multiplication");
        let mut result = Matrix::new(self.rows(), other.cols());
        for i in 0..self.rows() {
            for j in 0..other.cols() {
                let mut sum = T::default();
                for k in 0..self.cols() {
                    sum = sum + (self.data[i][k] * other.data[k][j]);
                }
                result.data[i][j] = sum;
            }
        }
        result
    }
}

// Div trait Matrix / scalar
impl<T> Div<T> for Matrix<T>
where T: Div<Output=T> + Default + Copy + PartialEq,
{
    type Output = Matrix<T>;
    fn div(self, scalar: T) -> Self::Output {
        assert!(scalar != T::default(), "cannot divide by zero!");
        let mut result = Matrix::new(self.rows(), self.cols());
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                result.data[i][j] = self.data[i][j] / scalar;
            }
        }
        result
    }
}

// MulAssign Matrix *= scalar
impl<T> MulAssign<T> for Matrix<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, scalar: T) {
        for row in &mut self.data {
            for elem in row {
                *elem *= scalar;
            }
        }
    }
}

// DivAssign Matrix /= scalar
impl<T> DivAssign<T> for Matrix<T>
where
    T: DivAssign + Copy + PartialEq + Default,
{
    fn div_assign(&mut self, scalar: T) {
        assert!(scalar != T::default(), "cannot divide by zero!");
        for row in &mut self.data {
            for elem in row {
                *elem /= scalar;
            }
        }
    }
}

impl<T: fmt::Display + std::fmt::Debug> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.data {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}

impl<T> Matrix<T>
where
    T: Copy,
{
    pub fn apply<F>(&mut self, func: F)
    where
        F: Fn(T) -> T,
    {
        for row in self.data.iter_mut() {
            for elem in row.iter_mut() {
                *elem = func(*elem);
            }
        }
    }
}

//impl<T> PartialEq<&Matrix<T>> for &Matrix<T>
//where
//    T: PartialEq,
//    T: Default + Copy
//{
//    fn eq(self, other: &Matrix<T>) -> bool {
//        if self.rows() != other.rows() { return false; }
//        if self.cols() != other.cols() { return false; }
//        for i in 0..self.rows() {
//            for j in 0..other.cols() {
//                if self.data[i][j] != other.data[i][j] { return false; }
//            }
//        }
//        true
//    }
//}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "matrix dimensions cannot be zero")]
    fn constructor_invalid_dimensions() {
        let _ = Matrix::<f64>::new(0, 3);
    }

    #[test]
    #[should_panic(expected = "matrix dimensions cannot be zero")]
    fn constructor_invalid_dimensions2() {
        let _ = Matrix::<f64>::new(3, 0);
    }

    #[test]
    fn constructor_valid_dimensions() {
        let matrix = Matrix::<i32>::new(2, 3);
        assert_eq!(matrix.rows(), 2);
        assert_eq!(matrix.cols(), 3);
    }

    #[test]
    fn element_access() {
        let mut matrix = Matrix::<i32>::new(2, 2);
        matrix[0][0] = 42;
        matrix[1][1] = 24;
        assert_eq!(matrix[0][0], 42);
        assert_eq!(matrix[1][1], 24);
    }

    #[test]
    fn copy_assignment() {
        let mut matrix_0 = Matrix::<i32>::new(2, 2);
        matrix_0[0][0] = 1;
        matrix_0[1][1] = 2;

        let matrix_1 = matrix_0.clone();
        assert_eq!(matrix_1[0][0], 1);
        assert_eq!(matrix_1[1][1], 2);
    }

    #[test]
    fn equality_and_inequality_operators() {
        let mut matrix_0 = Matrix::<i32>::new(2, 2);
        let matrix_1 = Matrix::<i32>::new(2, 2);
        assert_eq!(matrix_0, matrix_1);

        matrix_0[0][0] = 1;
        assert_ne!(matrix_0, matrix_1);
    }

    #[test]
    fn matrix_addition() {
        let mut matrix_0 = Matrix::<i32>::new(2, 2);
        let mut matrix_1 = Matrix::<i32>::new(2, 2);
        matrix_0[0][0] = 1;
        matrix_1[0][0] = 2;

        let result = matrix_0 + matrix_1;
        assert_eq!(result[0][0], 3);
    }

    #[test]
    #[should_panic(expected = "matrix dimensions must match for addition")] 
    fn matrix_addition_invalid_dimensions() {
        let matrix_0 = Matrix::<i32>::new(2, 2);
        let matrix_1 = Matrix::<i32>::new(3, 3);
        let _ = matrix_0 + matrix_1;
    }

    #[test]
    fn scalar_multiplication() {
        let mut matrix = Matrix::<i32>::new(2, 2);
        matrix[0][0] = 2;
        
        let result = matrix.clone() * 3;
        assert_eq!(result[0][0], 6);
    }

    #[test]
    fn matrix_multiplication() {
        let mut matrix_0 = Matrix::<i32>::new(2, 3);
        matrix_0[0][0] = 1;
        matrix_0[0][1] = 2;
        matrix_0[0][2] = 3;
        matrix_0[1][0] = 4;
        matrix_0[1][1] = 5;
        matrix_0[1][2] = 6;

        let mut matrix_1 = Matrix::<i32>::new(3, 2);
        matrix_1[0][0] = 7;
        matrix_1[0][1] = 8;
        matrix_1[1][0] = 9;
        matrix_1[1][1] = 10;
        matrix_1[2][0] = 11;
        matrix_1[2][1] = 12;

        let result = matrix_0 * matrix_1;

        assert_eq!(result[0][0], 58);
        assert_eq!(result[0][1], 64);
        assert_eq!(result[1][0], 139);
        assert_eq!(result[1][1], 154);
    }

    #[test]
    fn test_matrix_creation() {
        let m = Matrix::<i32>::new(3, 3);
        assert_eq!(m.rows(), 3);
        assert_eq!(m.cols(), 3);
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(m.get(i, j), 0);
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_zero_dimension_matrix() {
        let _ = Matrix::<i32>::new(0, 3);
    }

    #[test]
    fn test_get_set() {
        let mut m = Matrix::<i32>::new(2, 2);
        m.set(1, 1, 42);
        assert_eq!(m.get(1, 1), 42);
    }

    #[test]
    fn test_matrix_addition() {
        let mut m1 = Matrix::<i32>::new(2, 2);
        let mut m2 = Matrix::<i32>::new(2, 2);
        m1.set(0, 0, 1);
        m2.set(0, 0, 2);
        let result = &m1 + &m2;
        assert_eq!(result.get(0, 0), 3);
    }

    #[test]
    #[should_panic]
    fn test_matrix_addition_mismatched_size() {
        let m1 = Matrix::<i32>::new(2, 2);
        let m2 = Matrix::<i32>::new(3, 3);
        let _ = &m1 + &m2;
    }

    #[test]
    fn test_matrix_subtraction() {
        let mut m1 = Matrix::<i32>::new(2, 2);
        let mut m2 = Matrix::<i32>::new(2, 2);
        m1.set(0, 0, 5);
        m2.set(0, 0, 3);
        let result = &m1 - &m2;
        assert_eq!(result.get(0, 0), 2);
    }

    #[test]
    fn test_scalar_multiplication() {
        let mut m = Matrix::<i32>::new(2, 2);
        m.set(0, 0, 2);
        let result = m * 3;
        assert_eq!(result.get(0, 0), 6);
    }

    #[test]
    fn test_matrix_multiplication() {
        let mut m1 = Matrix::<i32>::new(2, 3);
        let mut m2 = Matrix::<i32>::new(3, 2);
        
        m1.set(0, 0, 1);
        m1.set(0, 1, 2);
        m1.set(0, 2, 3);
        m2.set(0, 0, 4);
        m2.set(1, 0, 5);
        m2.set(2, 0, 6);
        
        let result = &m1 * &m2;
        assert_eq!(result.get(0, 0), 32);
    }

    #[test]
    #[should_panic]
    fn test_matrix_multiplication_mismatched_size() {
        let m1 = Matrix::<i32>::new(2, 3);
        let m2 = Matrix::<i32>::new(4, 2);
        let _ = &m1 * &m2;
    }

    #[test]
    fn test_scalar_division() {
        let mut m = Matrix::<i32>::new(2, 2);
        m.set(0, 0, 6);
        let result = m / 2;
        assert_eq!(result.get(0, 0), 3);
    }

    #[test]
    #[should_panic]
    fn test_scalar_division_by_zero() {
        let m = Matrix::<i32>::new(2, 2);
        let _ = m / 0;
    }

    #[test]
    fn test_apply() {
        let mut m = Matrix::<i32>::new(2, 2);
        m.set(0, 0, 6);
        m.apply(|x| x * x);
        assert_eq!(m.get(0, 0), 36);
    }

    #[test]
    fn test_matrix_equality() {
        let mut m1 = Matrix::<i32>::new(2, 3);
        let mut m2 = Matrix::<i32>::new(2, 3);
        
        m1.set(0, 0, 1);
        m1.set(0, 1, 2);
        m1.set(0, 2, 3);
        m2.set(0, 0, 1);
        m2.set(0, 1, 2);
        m2.set(0, 2, 3);
        
        assert_eq!(m1, m2);
    }

    #[test]
    fn test_matrix_inequality() {
        let mut m1 = Matrix::<i32>::new(2, 3);
        let mut m2 = Matrix::<i32>::new(2, 3);
        
        m1.set(0, 0, 1);
        m1.set(0, 1, 2);
        m1.set(0, 2, 3);
        m2.set(0, 0, 1);
        m2.set(0, 1, 2);
        m2.set(0, 2, 4);
        
        assert_ne!(m1, m2);
    }
}
