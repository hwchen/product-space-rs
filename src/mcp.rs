use nalgebra::DMatrix;
use std::collections::HashMap;

use crate::Error;

pub trait Mcp {
    fn matrix(&self) -> &DMatrix<f64>;
    fn country_index(&self) -> &HashMap<String, usize>;
    fn product_index(&self) -> &HashMap<String, usize>;

    fn get(&self, country: &str, product: &str) -> Result<f64, Error> {
        get_by_country_product(
            &self.matrix(),
            &self.country_index(),
            &self.product_index(),
            country,
            product,
        )
    }

    fn get_country(&self, country: &str) -> Result<Vec<f64>, Error> {
        get_by_country(
            &self.matrix(),
            &self.country_index(),
            country,
        )
    }
}

// TODO: put in util module?
fn get_by_country_product(
    m: &DMatrix<f64>,
    country_index: &HashMap<String, usize>,
    product_index: &HashMap<String, usize>,
    country: &str,
    product: &str,
    ) -> Result<f64, Error>
{
    let matrix_row_idx = country_index.get(country)
        .ok_or_else(|| Error::MissingIndex { member: country.into(), index: "country".into() })?;
    let matrix_col_idx = product_index.get(product)
        .ok_or_else(|| Error::MissingIndex { member: product.into(), index: "product".into() })?;

    // these could be unchecked, because the country and product
    // indexes cannot be larger than matrix dimensions
    let matrix_row = m.row(*matrix_row_idx);
    let res = matrix_row[*matrix_col_idx];

    Ok(res)
}

// TODO: put in util module?
fn get_by_country(
    m: &DMatrix<f64>,
    country_index: &HashMap<String, usize>,
    country: &str,
    ) -> Result<Vec<f64>, Error>
{
    let matrix_row_idx = country_index.get(country)
        .ok_or_else(|| Error::MissingIndex { member: country.into(), index: "country".into() })?;

    // these could be unchecked, because the country and product
    // indexes cannot be larger than matrix dimensions
    let matrix_row = m.row(*matrix_row_idx);

    Ok(matrix_row.iter().cloned().collect())
}
