use nalgebra::DMatrix;

/// rca is (a/b) / (c/d)
/// where
/// a: dim1 member x dim2 member    (e.g. job type per city)
/// b: dim1 all    x dim2 member    (e.g. all jobs per city)
/// c: dim1 member x dim2 all       (e.g. job type per all cities)
/// d: dim1 all    x dim2 all       (e.g. all jobs per all cities)
///
/// a/b = ratio of job type compared to all jobs in a city
/// c/d = ratio of job type compared to all jobs in all cities
///
/// (a/b) / (c/d) = representation of job type in a city v. all cities
///
/// The matrix should have dim1 indexed to columns, and dim2 to rows
/// (e.g. jobs to columns, and countries to rows)
///
/// No nulls in matrix; only zero values allowed
pub fn rca(m: &DMatrix<f64>) -> DMatrix<f64> {
    // Implementation:
    //
    // The given matrix is already `a`.
    // `b` is a vector of the sums of cols in each row
    // `c` is a vector of the sums of rows in each col
    // `d` is a scalar of the sum of all values in matrix

    // this creates a new matrix to be the basis for the output
    let a = (*m).clone();

    // find `b`
    // the matrix op is col_sum, but it means adding all cols
    // in a row
    let b = a.column_sum();

    // find `c`
    // the matrix op is col_sum, but it means adding all rows
    // in a col
    let c = a.row_sum();

    // find `d`
    let d = a.sum();

    // c/d
    let mut c_d = c;
    c_d.apply(|x| x / d);
    //dbg!(&c_d);

    // to get a/b, sweep b across a
    let mut a_b = a;
    for i in 0..a_b.nrows() {
        let mut a_b_row = a_b.row_mut(i);
        a_b_row.apply(|a_val| a_val / b[i]);
    }
    //dbg!(&a_b);

    // to get (a/b)/(c/d) sweep c_d across a_b
    let mut a_b_c_d = a_b;
    for i in 0..a_b_c_d.ncols() {
        let mut a_b_c_d_col = a_b_c_d.column_mut(i);
        a_b_c_d_col.apply(|a_b_val| a_b_val / c_d[i]);
    }

    a_b_c_d
}

pub fn apply_fair_share(m: DMatrix<f64>) -> DMatrix<f64> {
    let mut m = m;

    m.apply(|x| if x >= 1.0 { 1.0 } else { 0.0 });

    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_rca() {
        println!("columns: product, rows: country");
        println!("b: [9, 12]");
        println!("c: [3, 7, 11]");
        println!("d: [21]");
        println!("a/b: [1/9, 3/9, 5/9 | 2/12, 4/12, 6/12]");
        println!("c/d: [3/21, 7/21, 11/21]");
        println!("(a/b)/(c/d): [(1/9)/(3/21), (3/9)/(7/21), (5/9)/(11/21) | (2/12)/(3/21), (4/12)/(7/21), (6/12)/(11/21)]");

        let m = DMatrix::from_vec(2,3,vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        println!("{}", m);
        let res = rca(&m);
        println!("{}", res);

        let expected = DMatrix::from_vec(2,3,vec![0.7777777777777778,1.1666666666666667,1.0,1.0,1.0606060606060606,0.9545454545454545]);

        assert_eq!(res, expected);
    }
}
