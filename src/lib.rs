use nalgebra::DMatrix;
use std::collections::HashMap;

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

// Currently just country and product.
// May make this more general in the future
pub struct ProductSpace {
    country_index: Vec<String>,
    product_index: Vec<String>,
    mcps: HashMap<u32, DMatrix<f64>>,
}

impl ProductSpace {
    // TODO Result and error handling for year range mistakes
    pub fn rca(
        &self,
        cutoff: Option<f64>,
        years: &[u32],
        ) -> Option<DMatrix<f64>>
    {
        let year_count = years.len();

        if year_count > 1 {
            let zeros = DMatrix::zeros(self.country_index.len(), self.product_index.len());
            let agg_mcp = years.iter()
                // in future, should return error if
                // year not present? Or maybe not
                .filter_map(|y| self.mcps.get(&y))
                .fold(zeros, |sum, x| sum + x);

            let mut res = rca(&agg_mcp);
            &mut apply_fair_share(&mut res, cutoff);
            Some(res)
        } else if year_count == 1 {
            // no extra allocation for mcp

            years.get(0)
                .and_then(|y| self.mcps.get(y))
                .map(|mcp| {
                    let mut res = rca(&mcp);
                    &mut apply_fair_share(&mut res, cutoff);
                    res
                })
        } else {
            None
        }
    }
}
