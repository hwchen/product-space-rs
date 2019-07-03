use nalgebra::DMatrix;
use std::collections::HashMap;

mod mcp;
pub use mcp::Mcp;

mod rca;
pub use rca::{
    apply_fair_share,
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
    country_idx: Vec<String>,
    product_idx: Vec<String>,
    mcps: HashMap<u32, DMatrix<f64>>,
}

impl ProductSpace {
    // TODO Result and error handling for year range mistakes
    pub fn rca(
        &self,
        years: &[u32],
        cutoff: Option<f64>,
        ) -> Option<Rca>
    {
        let year_count = years.len();

        if year_count > 1 {
            let zeros = DMatrix::zeros(self.country_idx.len(), self.product_idx.len());
            let agg_mcp = years.iter()
                // in future, should return error if
                // year not present? Or maybe not
                .filter_map(|y| self.mcps.get(&y))
                .fold(zeros, |sum, x| sum + x);

            let mut res = rca(&agg_mcp);
            if cutoff.is_some() {
                &mut apply_fair_share(&mut res, cutoff);
            }
            Some(Rca {
                country_idx: self.country_idx.clone(),
                product_idx: self.product_idx.clone(),
                m: res,
            })
        } else if year_count == 1 {
            // no extra allocation for mcp

            years.get(0)
                .and_then(|y| self.mcps.get(y))
                .map(|mcp| {
                    let mut res = rca(&mcp);
                    if cutoff.is_some() {
                        &mut apply_fair_share(&mut res, cutoff);
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

// TODO put indexes in Arc to avoid copying?
pub struct Rca {
    country_idx: Vec<String>,
    product_idx: Vec<String>,
    m: DMatrix<f64>,
}

impl Mcp for Rca {
    fn matrix(&self) -> &DMatrix<f64> {
        &self.m
    }
    fn country_index(&self) -> &[String] {
        &self.country_idx
    }
    fn product_index(&self) -> &[String] {
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
            country_idx: vec!["a".into(), "b".into()],
            product_idx: vec!["01".into(), "02".into(), "03".into()],
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
