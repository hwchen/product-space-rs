use csv;
use failure::{Error, format_err};
use nalgebra::DMatrix;
use product_space::{self, ProductSpace, Mcp};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::fs::File;
use std::time::Instant;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let opt = CliOpt::from_args();

    println!("Reading data from: {:?}", opt.filepath);


    let start_ingest = Instant::now();
    let ps = ps_from_tsv_reader(opt.filepath)?;
    let end_ingest = start_ingest.elapsed();
    println!("ingest time: {}.{:03}",
        end_ingest.as_secs(),
        end_ingest.subsec_millis()
    );

    println!("");

    let start_rca = Instant::now();
    {
        let rca = ps.rca(&[2017], None)
            .ok_or_else(|| format_err!("no rca for 2017?"))?;
        println!("RCA test against simoes ps_calcs for 2017");
        println!("nzl::0204, expect 149.962669: {:?}", rca.get("nzl", "0204")?);
    }
    let end_rca = start_rca.elapsed();
    println!("time: {}.{:03}",
        end_rca.as_secs(),
        end_rca.subsec_millis()
    );

    println!("");

    let start_rca = Instant::now();
    {
        let rca = ps.rca(&[2015,2016,2017], None)
            .ok_or_else(|| format_err!("no rca for 2015-2017?"))?;
        println!("nzl::0204, 2015-2017: {}", rca.get("nzl", "0204")?);
    }
    let end_rca = start_rca.elapsed();
    println!("time: {}.{:03}",
        end_rca.as_secs(),
        end_rca.subsec_millis()
    );

    println!("");

    for year in 2015..=2017 {
        let rca = ps.rca(&[year as u32], None)
            .ok_or_else(|| format_err!("no rca for year"))?;
        println!("nzl::0204, {}: {}", year, rca.get("nzl", "0204")?);
    }

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

