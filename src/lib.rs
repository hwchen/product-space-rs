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

mod error;
pub use error::Error;

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
        ) -> Option<Rca>
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
            if cutoff.is_some() {
                &mut apply_fair_share(&mut res, cutoff);
            }
            Some(Rca {
                country_index: self.country_index.clone(),
                product_index: self.product_index.clone(),
                matrix: res,
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
                        country_index: self.country_index.clone(),
                        product_index: self.product_index.clone(),
                        matrix: res,
                    }
                })
        } else {
            None
        }
    }
}

// TODO put indexes in Arc to avoid copying?
pub struct Rca {
    country_index: Vec<String>,
    product_index: Vec<String>,
    matrix: DMatrix<f64>,
}

impl Rca {
    pub fn get_value(&self, country: &str, product: &str) -> Result<f64, Error> {
        get_by_country_product(
            &self.matrix,
            &self.country_index,
            &self.product_index,
            country,
            product,
        )
    }

//    pub fn get_country(&self, country: &str, product: &str) -> Result<f64, Error> {
//        get_by_country(
//            &self.matrix,
//            &self.country_index,
//            &self.product_index,
//            country,
//            product,
//        )
//    }
}

// TODO: put in util module?
fn get_by_country_product(
    m: &DMatrix<f64>,
    country_index: &[String],
    product_index: &[String],
    country: &str,
    product: &str
    ) -> Result<f64, Error>
{
    let matrix_row_idx = country_index
        .iter()
        .position(|c| *c == country)
        .ok_or_else(|| Error::MissingIndex { member: country.into(), index: "country".into() })?;
    let matrix_col_idx = product_index
        .iter()
        .position(|c| *c == product)
        .ok_or_else(|| Error::MissingIndex { member: product.into(), index: "product".into() })?;

    // these could be unchecked, because the country and product
    // indexes cannot be larger than matrix dimensions
    let matrix_row = m.row(matrix_row_idx);
    let res = matrix_row[matrix_col_idx];

    Ok(res)
}
