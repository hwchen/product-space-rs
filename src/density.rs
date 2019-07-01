use nalgebra::DMatrix;

// rca input is matrix of rca, where
// - col indexes are product
// - row indexes are countries
// proximity input is calculated from rca, is product x product
pub fn into_density(rca: DMatrix<f64>, proximity: DMatrix<f64>) -> DMatrix<f64> {
    // first pass: following instructions are from
    // simoes ps_calcs proximity fn using np
    // np notes:
    // dot is just multiplication, not dot productA.
    // mul/div is componentwise, not sweeping or otherwise

    // numerator is rca multiplied with proximities
    let density_numerator = &rca * &proximity;

    // (simoes says denominator by multiplying proximities by all
    // ones vector, getting sum of all proximities
    let rca_ones = rca.map(|_| 1.0);
    let density_denominator = rca_ones * &proximity;

    density_numerator.component_div(&density_denominator)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::into_rca;
    use crate::into_proximity;

    #[test]
    fn test_density() {
        println!("columns: product, rows: country");

        let m = DMatrix::from_vec(2,3,vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        println!("matrix:\n{}", m);

        let rca = into_rca(m);
        println!("rca:\n{}", rca);

        let proximity = into_proximity(rca.clone());
        println!("proximity:\n{}", proximity);

        let density = into_density(rca, proximity);
        println!("density:\n{}", density);

        let expected = DMatrix::from_vec(2,3,
            vec![
                0.9444510696719741,
                1.0416616977460194,
                0.9470602761804753,
                1.0397047928646437,
                0.9477553071583487,
                1.0391835196312382,
            ]
        );
        println!("expected:\n{}", expected);

        assert_eq!(density, expected);
    }
}

// expected from simoes
//          0         1         2
//0  0.944451  0.947060  0.947755
//1  1.041662  1.039705  1.039184

