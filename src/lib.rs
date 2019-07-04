use nalgebra::DMatrix;
use std::collections::HashMap;

mod mcp;
pub use mcp::Mcp;

mod rca;
pub use rca::{
    apply_fair_share,
    apply_fair_share_into,
    apply_rca,
    fair_share,
    rca,
};

mod proximity;
pub use proximity::proximity;

mod density;
pub use density::density;

mod distance;
pub use distance::distance;

mod complexity;
pub use complexity::complexity;

mod error;
pub use error::Error;

// Currently just country and product.
// May make this more general in the future
pub struct ProductSpace {
    country_idx: HashMap<String, usize>,
    product_idx: HashMap<String, usize>,
    mcps: HashMap<u32, DMatrix<f64>>,
}

impl ProductSpace {
    /// if years not found, either returns None or silently skips
    /// for aggregating, will either
    /// for cutoff, rca(t) = 1 if rca(t-1) > cutoff and rca(t-2) > cutoff...
    /// - otherwise just average
    pub fn rca(
        &self,
        years: &[u32],
        cutoff: Option<f64>,
        ) -> Option<Rca>
    {
        if years.len() > 0 {
            let zeros = DMatrix::from_element(
                self.country_idx.len(),
                self.product_idx.len(),
                1.0,
            );

            // for cutoff, rca(t) = 1 if rca(t-1) > cutoff and rca(t-2) > cutoff...
            //
            // else just avg the rca
            let mut res = years.iter()
                // silently removes missing years
                .filter_map(|y| self.mcps.get(y))
                .fold(zeros, |mut z, mcp| {
                    let mut rca_matrix = rca(&mcp);
                    if cutoff.is_some() {
                        apply_fair_share_into(&mut rca_matrix, &mut z, cutoff);
                    } else {
                        // just average as default?
                        // do the sum part here, divide at end
                        z += rca_matrix;
                    }
                    z
                });

            // avg if no cutoff
            if cutoff.is_none() {
                res.apply(|x| x / years.len() as f64)
            }

            Some(Rca {
                country_idx: self.country_idx.clone(),
                product_idx: self.product_idx.clone(),
                m: res,
            })
        } else if years.len() == 1 {
            // no extra allocation for mcp

            years.get(0)
                .and_then(|y| self.mcps.get(y))
                .map(|mcp| {
                    let mut res = rca(&mcp);
                    if cutoff.is_some() {
                        apply_fair_share(&mut res, cutoff);
                    }
                    Rca {
                        country_idx: self.country_idx.clone(),
                        product_idx: self.product_idx.clone(),
                        m: res,
                    }
                })
        } else {
            None
        }
    }
}

impl ProductSpace {
    pub fn new(country_idx: HashMap<String, usize>, product_idx: HashMap<String, usize>, mcps: HashMap<u32, DMatrix<f64>>) -> Self {
        Self {
            country_idx,
            product_idx,
            mcps,
        }
    }
}

// TODO put indexes in Arc to avoid copying?
pub struct Rca {
    country_idx: HashMap<String, usize>,
    product_idx: HashMap<String, usize>,
    m: DMatrix<f64>,
}

impl Mcp for Rca {
    fn matrix(&self) -> &DMatrix<f64> {
        &self.m
    }
    fn country_index(&self) -> &HashMap<String, usize> {
        &self.country_idx
    }
    fn product_index(&self) -> &HashMap<String, usize> {
        &self.product_idx
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_ps_interface() {
        let vals = DMatrix::from_vec(2,3,vec![1.0,2.0,3.0,4.0,5.0,6.0]);
        let mut mcps = HashMap::new();
        mcps.insert(2017, vals);

        let ps = ProductSpace {
            country_idx: [("a".to_string(),0usize), ("b".to_string(),1)].iter().cloned().collect(),
            product_idx: [("01".to_string(),0usize), ("02".to_string(),1), ("03".to_string(),2)].iter().cloned().collect(),
            mcps,
        };

        let rca = ps.rca(&[2017], None).unwrap();

        let expected = DMatrix::from_vec(2,3,vec![0.7777777777777778,1.1666666666666667,1.0,1.0,1.0606060606060606,0.9545454545454545]);

        assert_eq!(rca.m, expected);

        let val = rca.get("a", "01").unwrap();
        assert_eq!(val, 0.7777777777777778);

        let vals = rca.get_country("b").unwrap();
        assert_eq!(vals, vec![1.1666666666666667, 1.0, 0.9545454545454545]);
    }
}
