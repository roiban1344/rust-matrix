use rug::Rational;
use std::cmp::{Eq, PartialEq};
use std::ops::{Add, Mul};

pub trait Zero {
    fn zero() -> Self;
}

impl Zero for i32 {
    fn zero() -> i32 {
        0
    }
}

impl Zero for Rational {
    fn zero() -> Rational {
        Rational::new()
    }
}

pub trait One {
    fn one() -> Self;
}

impl One for i32 {
    fn one() -> i32 {
        1
    }
}

impl One for Rational {
    fn one() -> Rational {
        Rational::from(1)
    }
}

#[derive(Debug)]
pub struct Matrix<T>
where
    T: Add + Mul + Clone + Zero + One,
{
    row: usize,
    col: usize,
    elements: Vec<Vec<T>>,
}

impl<T> Matrix<T>
where
    T: Add + Mul + Clone + Zero + One,
{
    //TODO: 行列入力用マクロ
    pub fn from(elements: Vec<Vec<T>>) -> Self {
        let row = elements.len();
        let col = elements[0].len();
        Matrix { row, col, elements }
    }

    pub fn at(&self, i: usize, j: usize) -> T {
        self.elements[i][j].clone()
    }

    pub fn size(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}

impl<T> Add for Matrix<T>
where
    T: Add<Output = T> + Mul + Clone + Zero + One,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let (row, col) = self.size();
        let mut elements = Vec::<Vec<T>>::new();
        for _ in 0..row {
            elements.push(Vec::new());
        }
        for i in 0..row {
            for j in 0..col {
                //let a =  self.elements[i][j];// + rhs.elements[i][j];
                elements[i].push(self.at(i, j) + rhs.at(i, j));
            }
        }
        Matrix { row, col, elements }
    }
}

impl<T> Mul for Matrix<T>
where
    T: Add<Output = T> + Mul<Output = T> + Clone + Zero + One,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let (row, mid) = self.size();
        let (_, col) = rhs.size();
        let mut elements = Vec::<Vec<T>>::new();
        for i in 0..row {
            elements.push(vec![]);
            for j in 0..col {
                let mut e = T::zero();
                for k in 0..mid {
                    e = e + self.at(i, k) * rhs.at(k, j);
                }
                elements[i].push(e);
            }
        }

        Matrix { row, col, elements }
    }
}

impl<T> PartialEq for Matrix<T>
where
    T: Add + Mul + Clone + Eq + Zero + One,
{
    fn eq(&self, other: &Self) -> bool {
        let (row, col) = self.size();
        for i in 0..row {
            for j in 0..col {
                if self.at(i, j) != other.at(i, j) {
                    return false;
                }
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        let a = Matrix::from(vec![vec![1, 2], vec![3, 4]]);
        let b = Matrix::from(vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(a, b);
    }

    #[test]
    fn sum() {
        let a = Matrix::from(vec![vec![1, 2], vec![3, 4]]);
        let b = Matrix::from(vec![vec![-1, 5], vec![12, 9]]);
        let c = Matrix::from(vec![vec![0, 7], vec![15, 13]]);

        assert_eq!(a + b, c);
    }

    #[test]
    fn prod() {
        let a = Matrix::from(vec![vec![1, 2], vec![3, 4]]);
        let b = Matrix::from(vec![vec![5, 6], vec![7, 8]]);
        let c = Matrix::from(vec![vec![19, 22], vec![43, 50]]);

        assert_eq!(a * b, c);
    }
}
