use nalgebra::DMatrix;

pub fn avg<'a>(ms: impl Iterator<Item=&'a DMatrix<f64>>, size: usize, count: usize) -> DMatrix<f64> {
    let zeros = DMatrix::zeros(
        size,
        size,
    );

    let mut res = ms
        .fold(zeros, |mut z, proximity_matrix| {
            z += proximity_matrix;
            z
        });

    res.apply(|x| x / count as f64);

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_avg() {
        let matrixes = [
            DMatrix::from_vec(2,2,vec![1.0,5.0,3.0,8.0]),
            DMatrix::from_vec(2,2,vec![2.0,3.0,4.0,5.0]),
            DMatrix::from_vec(2,2,vec![3.0,4.0,5.0,6.0]),
            DMatrix::from_vec(2,2,vec![4.0,2.0,6.0,9.0]),
            DMatrix::from_vec(2,2,vec![5.0,6.0,7.0,2.0]),
        ];

        let my_avg = avg(matrixes.iter(), 2, 5);

        let expected = DMatrix::from_vec(2,2,vec![3.0,4.0,5.0,6.0]);

        assert_eq!(my_avg, expected);
    }
}
