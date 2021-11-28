use matrix::Matrix;

fn main() {
    let a = Matrix::from(vec![vec![1, 2], vec![3, 4]]);
    let b = Matrix::from(vec![vec![5, 6], vec![7, 8]]);
    println!("{:?}", a * b);
}
