use csv;
use failure::Error;
use nalgebra::DMatrix;
use product_space::{self, ProductSpace};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::fs::File;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let opt = CliOpt::from_args();

    println!("Reading data from: {:?}", opt.filepath);


    let _ps = ps_from_tsv_reader(opt.filepath)?;

    Ok(())
}

/// Constructed
/// - on country exports by product
/// - skipping null exports
///
/// matrix:
/// - row = countries
/// - columns = products
pub fn ps_from_tsv_reader(filepath: PathBuf) -> Result<ProductSpace, Error> {
    // country and product sets are needed while building, because some countries
    // and some products may not exist in each year
    //
    // So this is a preprocessing step before putting everything in the matrix
    let mut country_set = HashSet::new();
    let mut product_set = HashSet::new();
    let mut year_set = HashSet::new();

    // 2 passes are needed, unless files are sorted.
    // But maybe simpler with 2 passes.
    //
    // 1st pass to get all product and countries, to know size of matrices
    // 2nd pass to create matrices.

    // first pass
    let f = File::open(&filepath)?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(f);

    for result in rdr.deserialize() {
        let record: Record = result?;
        country_set.insert(record.country.to_string());
        product_set.insert(record.product.to_string());
        year_set.insert(record.year);
    }

    // now build all matrics in preparation for mutating
    let mut mcps: HashMap<u32,_> = year_set.into_iter()
        .map(|y| (y, DMatrix::zeros(country_set.len(), product_set.len())))
        .collect();

    let country_idx: HashMap<_,_> = country_set.into_iter()
        .enumerate()
        .map(|(v,k)| (k,v))
        .collect();
    let product_idx: HashMap<_,_> = product_set.into_iter()
        .enumerate()
        .map(|(v,k)| (k,v))
        .collect();

    // now 2nd pass
    let f = File::open(&filepath)?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(f);

    for result in rdr.deserialize() {
        let record: Record = result?;

        if record.val != "NULL" {
            let export = record.val.parse::<f64>()?;

            let mcp = mcps.get_mut(&record.year)
                .expect("logic error, year must be in");

            let matrix_row_idx = country_idx.get(&record.country)
                .expect("logic error, country must be in");
            let matrix_col_idx = product_idx.get(&record.product)
                .expect("logic error, product must be in");

            let mut matrix_row = mcp.row_mut(*matrix_row_idx);
            // this could be unchecked
            matrix_row[*matrix_col_idx] = export;
        }
    }

    // TODO change indexes in lib?
    let mut country_idx = country_idx.iter().collect::<Vec<_>>();
    country_idx.sort_by_key(|&(_country, idx)| idx);
    let country_idx = country_idx.iter()
        .map(|(c, _)| c.to_string())
        .collect::<Vec<_>>();

    let mut product_idx = product_idx.iter().collect::<Vec<_>>();
    product_idx.sort_by_key(|&(_product, idx)| idx);
    let product_idx = product_idx.iter()
        .map(|(p, _)| p.to_string())
        .collect::<Vec<_>>();

    Ok(product_space::ProductSpace::new(
        country_idx,
        product_idx,
        mcps,
    ))
}

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename="origin")]
    country: String,
    #[serde(rename="hs92")]
    product: String,
    year: u32,
    #[serde(rename="export_val")]
    val: String, // parse to f64 after, but have to handle NULL
}

#[derive(Debug, StructOpt)]
struct CliOpt {
    #[structopt(parse(from_os_str))]
    filepath: PathBuf,
}

