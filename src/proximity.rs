use nalgebra::DMatrix;

// rca input is matrix of rca, where
// - col indexes are product
// - row indexes are countries
pub fn proximity(rca: &DMatrix<f64>) -> DMatrix<f64> {
    // first pass: following instructions are from
    // simoes ps_calcs proximity fn using np
    // np notes:
    // dot is just multiplication, not dot product
    // mul/div is componentwise, not sweeping or otherwise

    // transpose matrix so row indexes are products
    let rca_t = rca.transpose();

    // product of rca_tranpose and rca transpose transpose
    let numerator_intersection = rca_t * rca;

    // kp0 is vector of the sum of rca per product
    // (simoes says it's vector of the number of munics with RCA in given product,
    // not sure what 'number of munics' has to do with it)
    let kp0 = rca.row_sum();

    // (simoes: transpose kp0 to get unions)
    // I don't know what unions are.
    let kp0_t = kp0.transpose();

    // (simoes: denominator is product of kp0 and kp0_t, then take sqrt
    // sqrt is for geometric mean)
    let mut denominator_union_sqrt = kp0_t * kp0;
    denominator_union_sqrt.apply(|x| x.sqrt());

    // componentwise division of numerator/denominator
    let phi = numerator_intersection.component_div(&denominator_union_sqrt);

    phi
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rca;

    #[test]
    fn test_proximity() {
        println!("columns: product, rows: country");

        let m = DMatrix::from_vec(2,3,vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        println!("matrix:\n{}", m);

        let rca = rca(&m);
        println!("rca:\n{}", rca);

        let proximity = proximity(&rca);
        println!("proximity:\n{}", proximity);

        let expected = DMatrix::from_vec(3,3,
            vec![
                1.011111111111111,
                0.9860132971832695,
                0.9793228211476318,
                0.9860132971832695,
                1.0,
                1.0037807318213265,
                0.9793228211476318,
                1.0037807318213265,
                1.0103668261562997,
            ]
        );
        println!("expected:\n{}", expected);

        assert_eq!(proximity, expected);
    }
}

// from simoes calcs, expected output:
// expected:

//  ┌                            ┐
//  │ 1.011111 0.986013 0.979323 │
//  │ 0.986013        1 1.003781 │
//  │ 0.979323 1.003781 1.010367 │
//  └                            ┘
