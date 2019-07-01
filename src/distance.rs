use nalgebra::DMatrix;

use crate::density;

// rca input is matrix of rca, where
// - col indexes are product
// - row indexes are countries
// proximity input is calculated from rca, is product x product
pub fn distance(rca: &DMatrix<f64>, proximity: &DMatrix<f64>) -> DMatrix<f64> {
    let mut m = density(rca, proximity);
    m.apply(|x| 1.0 - x);
    m
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rca;
    use crate::proximity;

    #[test]
    fn test_density() {
        println!("columns: product, rows: country");

        let m = DMatrix::from_vec(2,3,vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        println!("matrix:\n{}", m);

        let rca = rca(&m);
        println!("rca:\n{}", rca);

        let proximity = proximity(&rca);
        println!("proximity:\n{}", proximity);

        let distance = distance(&rca, &proximity);
        println!("distance:\n{}", distance);

        let expected = DMatrix::from_vec(2,3,
            vec![
                0.05554893032802588,
                -0.041661697746019355,
                0.05293972381952472,
                -0.03970479286464368,
                0.05224469284165134,
                -0.03918351963123823,
            ]
        );
        println!("expected:\n{}", expected);

        assert_eq!(distance, expected);
    }
}

// expected from simoes
//           0         1         2
//0  0.055549  0.052940  0.052245
//1 -0.041662 -0.039705 -0.039184

