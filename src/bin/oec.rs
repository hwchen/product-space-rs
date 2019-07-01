use nalgebra::DMatrix;
use std::collections::HashSet;

// TODO move this to examples

use failure::{Error, format_err};
use product_space;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let opt = CliOpt::from_args();

    println!("Reading data from: {:?}", opt.filepath);

    let f = File::open(opt.filepath)?;

    let mcp = OecMcpMatrix::from_tsv_reader(opt.year, BufReader::new(f))?;

    println!("Export val");
    println!("ago::0106 {:?}", mcp.get_val_by_country_product("ago", "0106"));
    println!("bdi::1513 {:?}", mcp.get_val_by_country_product("bdi", "1513"));

    println!("RCA");
    println!("ago::0106 {:?}", mcp.get_rca_by_country_product("ago", "0106"));
    println!("bdi::1513 {:?}", mcp.get_rca_by_country_product("bdi", "1513"));

    println!("RCA test against simoes ps_calcs");
    println!("nzl::0204, expect 149.962669: {:?}", mcp.get_rca_by_country_product("nzl", "0204"));
    println!("svk::8703, expect 4.441051: {:?}", mcp.get_rca_by_country_product("svk", "8703"));

    println!("fair share test against simoes ps_calcs");
    println!("bra::2825, expect 1.0: {:?}", mcp.get_fair_share_by_country_product("bra", "2825"));
    println!("bra::1521, expect 1.0: {:?}", mcp.get_fair_share_by_country_product("bra", "1521"));
    println!("bra::1601, expect 1.0: {:?}", mcp.get_fair_share_by_country_product("bra", "1601"));
    println!("bra::1602, expect 1.0: {:?}", mcp.get_fair_share_by_country_product("bra", "1602"));

    Ok(())
}

/// Constructed
/// - for one year
/// - on country exports by product
/// - skipping null exports
///
/// matrix:
/// - row = countries
/// - columns = products
///
/// includes a basic kind-of index
/// country_index: a vec that you scan down to find index
/// product_index: a vec that you scan down to find index
///
#[derive(Debug)]
struct OecMcpMatrix {
    country_index: Vec<String>,
    product_index: Vec<String>,
    // TODO remove, this is just for testing purposes
    product_matrix: DMatrix<f64>,
    // TODO remove, this is just for testing purposes
    rca_matrix: DMatrix<f64>,
    fair_share_matrix: DMatrix<f64>,
}

impl OecMcpMatrix {
    // TODO add in line numbers for better error handling
    // Also, the fields are just hardcoded to the 'year_origin_hs92_4.tsv' file
    pub fn from_tsv_reader<R: Read>(year: u32, rdr: BufReader<R>) -> Result<Self, Error> {
        // country and product sets are need while building, because some countries
        // and some products may not exist in each year
        //
        // So this is a preprocessing step before putting everything in the matrix
        let mut country_set = HashSet::new();
        let mut product_set = HashSet::new();
        let mut rows = vec![];

        // first get rows for the selected year
        // build country set and product set along the way
        for row_str in rdr.lines().skip(1) {
            let row_str = row_str?;
            let mut cells = row_str.split('\t');

            let current_year = cells.next()
                .ok_or_else(|| format_err!("could not find year value"))?
                .parse::<u32>()?;

            if current_year == year {
                let country = cells.next()
                    .ok_or_else(|| format_err!("couldn't find country (origin) val"))?
                    .to_owned()
                    .clone();
                let product = cells.next()
                    .ok_or_else(|| format_err!("couldn't find product (hs92) val"))?
                    .to_owned()
                    .clone();

                let export_val = cells.next()
                    .ok_or_else(|| format_err!("could not find export_val val"))?;

                // skip row again if export value is NULL
                if export_val != "NULL" {
                    let export = export_val.parse::<f64>()?;
                    rows.push(Record {
                        country: country.clone(),
                        product: product.clone(),
                        val: export,
                    });

                    country_set.insert(country.to_string());
                    product_set.insert(product.to_string());
                }
                // Now skip
                // - import_val
                // - export_rca
                // - import_rca
                // by not advancing iter
            }
        }

        // Now that we have
        // - the set of countries
        // - the set of produts
        // - all rows, filtered for year and export val
        // we'll directly create the matrix
        let country_index: Vec<_> = country_set.into_iter().collect();
        let product_index: Vec<_> = product_set.into_iter().collect();

        let mut product_matrix = DMatrix::zeros(country_index.len(), product_index.len());

        for row in rows {
            let matrix_row_idx = country_index
                .iter()
                .position(|c| **c == row.country)
                .expect("Logic error: country missing from index");
            let matrix_col_idx = product_index
                .iter()
                .position(|c| **c == row.product)
                .expect("Logic error: product missing from index");

            let mut matrix_row = product_matrix.row_mut(matrix_row_idx);
            // this could be unchecked
            matrix_row[matrix_col_idx] = row.val;
        }

        // each allocates for a new matrix
        // use apply_x if want to do in place
        let rca_matrix = product_space::rca(&product_matrix);
        let fair_share_matrix = product_space::fair_share(&rca_matrix);

        Ok(OecMcpMatrix {
            country_index,
            product_index,
            product_matrix,
            rca_matrix,
            fair_share_matrix,
        })
    }

    pub fn get_val_by_country_product(&self, country: &str, product: &str) -> Result<f64, Error> {
        Self::get_by_country_product(
            &self.product_matrix,
            &self.country_index,
            &self.product_index,
            country,
            product,
        )
    }

    pub fn get_rca_by_country_product(&self, country: &str, product: &str) -> Result<f64, Error> {
        Self::get_by_country_product(
            &self.rca_matrix,
            &self.country_index,
            &self.product_index,
            country,
            product,
        )
    }

    pub fn get_fair_share_by_country_product(&self, country: &str, product: &str) -> Result<f64, Error> {
        Self::get_by_country_product(
            &self.fair_share_matrix,
            &self.country_index,
            &self.product_index,
            country,
            product,
        )
    }

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
            .ok_or_else(|| format_err!("Country {:?} not found", country))?;
        let matrix_col_idx = product_index
            .iter()
            .position(|c| *c == product)
            .ok_or_else(|| format_err!("Product {:?} not found", product))?;

        // these could be unchecked, because the country and product
        // indexes cannot be larger than matrix dimensions
        let matrix_row = m.row(matrix_row_idx);
        let res = matrix_row[matrix_col_idx];

        Ok(res)
    }
}


#[derive(Debug)]
struct Record {
    country: String,
    product: String,
    val: f64,
}

#[derive(Debug, StructOpt)]
struct CliOpt {
    #[structopt(parse(from_os_str))]
    filepath: PathBuf,

    #[structopt(long="year")]
    year: u32,
}
