use nalgebra::DMatrix;

pub fn into_proximity(rca: DMatrix<f64>) -> DMatrix<f64> {
    DMatrix::zeros(1,1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::into_rca;

    #[test]
    fn test_proximity() {
        println!("columns: product, rows: country");

        let m = DMatrix::from_vec(2,3,vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        println!("matrix:\n{}", m);

        let rca = into_rca(m);
        println!("rca:\n{}", rca);

        let proximity = into_proximity(rca);
        println!("proximity:\n{}", proximity);

        let expected = DMatrix::from_vec(3,3,
            vec![
                1.011111,
                0.986013,
                0.979323,
                0.986013,
                1.000000,
                1.003781,
                0.979323,
                1.003781,
                1.010367,
            ]
        );
        println!("expected:\n{}", expected);

        assert_eq!(proximity, expected);
    }
}

// output from python fn:
//"""
//          0         1         2
//0  1.011111  0.986013  0.979323
//1  0.986013  1.000000  1.003781
//2  0.979323  1.003781  1.010367
//"""
