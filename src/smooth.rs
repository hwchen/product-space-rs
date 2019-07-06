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
            DMatrix::from_element(2,2,1.0),
            DMatrix::from_element(2,2,2.0),
            DMatrix::from_element(2,2,3.0),
            DMatrix::from_element(2,2,4.0),
            DMatrix::from_element(2,2,5.0),
        ];

        let my_avg = avg(matrixes.iter(), 2, 5);

        let expected = DMatrix::from_element(2,2,3.0);

        assert_eq!(my_avg, expected);
    }
}
