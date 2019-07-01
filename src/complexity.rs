use nalgebra::{DMatrix, convert};

// rca input is matrix of rca, where
// - col indexes are product
// - row indexes are countries
pub fn complexity(rca: &DMatrix<f64>) -> (DMatrix<f64>, DMatrix<f64>) {
    // first pass: following instructions are from
    // simoes ps_calcs proximity fn using np
    // np notes:
    // dot is just multiplication, not dot product.
    // mul/div is componentwise, not sweeping or otherwise

    // k product
    let kp0 = rca.row_sum_tr();
    let mut kp = kp0.clone();

    // k country
    let kc0 = rca.column_sum();
    let mut kc = kc0.clone();

    // (from simoes, it loops 10 times but I don't know why.
    // On last pass, does additional mult by kp)
    for i in 0..19 {
        // temps needed because the calculations in the next step
        // modify kc and kp, but depend on their value at the beginning
        // of the loop pass
        let kc_temp = kc.clone();
        let kp_temp = kp.clone();

        kp = convert((rca.transpose() * &kc_temp).component_div(&kp0));
        if i < 18 {
            kc = (rca * &kp_temp).component_div(&kc0);
        }
    }
    println!("kp0: {}", kp0);
    println!("kc0: {}", kc0);
    println!("kp: {}", kp);
    println!("kc: {}", kc);

    let kc_mean = mean(&convert(kc.clone()));
    let kc_std = std(&convert(kc.clone()));
    let mut geo_complexity = kc;

    let kp_mean = mean(&convert(kp.clone()));
    let kp_std = std(&convert(kp.clone()));
    let mut prod_complexity = kp;

    println!("kp_mean: {}", kp_mean);
    println!("kc_mean: {}", kc_mean);
    println!("kp_std: {}", kp_std);
    println!("kc_std: {}", kc_std);

    geo_complexity.apply(|x| (x - kc_mean) / kc_std);
    prod_complexity.apply(|x| (x - kp_mean) / kp_std);

    (convert(geo_complexity), convert(prod_complexity))
}

// only for <U1, Dynamic> vectors
fn mean(m: &DMatrix<f64>) -> f64 {
    assert!(m.ncols() == 1);

    let col = m.columns(0,1);
    let n = col.len();
    let total = col.iter().sum::<f64>();

    total / n as f64
}

// only for <U1, Dynamic> vectors
// This is just temp, until I find a lib or something to calculate
// std deviation
fn std(m: &DMatrix<f64>) -> f64 {
    // The standard deviation is the square root of the
    // average of the squared deviations from the mean, i.e.,
    // `std = sqrt(mean(abs(x - x.mean())**2))`.

    assert!(m.ncols() == 1);

    let mean = mean(m);

    let col = m.columns(0,1);
    let n = col.len();
    let dev = col.iter()
        .map(|x| ((x - mean).abs()).powf(2.0));

    let mean_dev = dev.sum::<f64>() / (n - 1) as f64;

    mean_dev.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rca;

    #[test]
    fn test_std() {
        let m = DMatrix::from_vec(3,1,vec![1.0, 3.0, 5.0]);
        let std_dev = std(&m);
        assert_eq!(std_dev, 1.632993161855452);

        let m = DMatrix::from_vec(3,1,vec![1.0, 3.0, 6.0]);
        let std_dev = std(&m);
        assert_eq!(std_dev, 2.0548046676563256);

        let m = DMatrix::from_vec(4,1,vec![9.365921518323761,9.365168229974921,9.366119246144434,9.366618939884766]);
        let std_dev = std(&m);
        assert_eq!(std_dev, 0.0005215135001035631);

        panic!();
    }

    #[test]
    fn test_complexity() {
        println!("columns: product, rows: country");

        let m = DMatrix::from_vec(2,4,vec![100.0, 2000.0, 3.0, 4000.0, 500.0, 6000.0, 17.0, 23.0]);
        println!("matrix:\n{}", m);

        let rca = rca(&m);
        println!("rca:\n{}", rca);

        let complexity = complexity(&rca);
        println!("geo complexity:\n{}", complexity.0);
        println!("product complexity:\n{}", complexity.1);

        let expected_geo= DMatrix::from_vec(2,1,
            vec![
                0.0,
                -1.0,
            ]
        );
        println!("expected_geo:\n{}", expected_geo);

        let expected_product= DMatrix::from_vec(3,1,
            vec![
                0.0,
                1.414214,
                0.0,
            ]
        );
        println!("expected_product:\n{}", expected_product);

        assert_eq!(complexity.0, expected_geo);
        assert_eq!(complexity.1, expected_product);
    }
}
// expected from simoes
// (0    0.707107
// 1   -0.707107
// dtype: float64, 0   -0.058894
// 1   -1.309804
// 2    0.269453
// 3    1.099245

