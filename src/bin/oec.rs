// TODO move this to examples

use failure::{Error, format_err};
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let opt = CliOpt::from_args();

    println!("Reading data from: {:?}", opt.filepath);

    let f = File::open(opt.filepath)?;

    let records = BufReader::new(f)
        .lines()
        .skip(1) //skip header
        .map(|line| -> Result<Record, Error> { Record::from_tsv_row(&line?) });

    for record in records {
        println!("{:?}", record);
    }

    Ok(())
}

#[derive(Debug)]
struct Record {
    year: u32,
    origin: String,
    hs92: String,
    export_val: Option<f64>,
    import_val: Option<f64>,
    export_rca: Option<f64>,
    import_rca: Option<f64>,
}

impl Record {
    // TODO add in line numbers for better error handling
    pub fn from_tsv_row(row_str: &str) -> Result<Self, Error> {
        let mut cells = row_str.split('\t');

        let year = cells.next()
            .ok_or_else(|| format_err!("could not find year value"))?
            .parse::<u32>()?;
        let origin = cells.next()
            .ok_or_else(|| format_err!("could not find origin value"))?
            .to_owned();
        let hs92 = cells.next()
            .ok_or_else(|| format_err!("could not find hs92 value"))?
            .to_owned();

        let export_val = cells.next()
            .ok_or_else(|| format_err!("could not find export_val value"))?;
        let export_val = match export_val {
            "NULL" => None,
            s => Some(s.parse::<f64>()?)
        };

        let import_val = cells.next()
            .ok_or_else(|| format_err!("could not find import_val value"))?;
        let import_val = match import_val {
            "NULL" => None,
            s => Some(s.parse::<f64>()?)
        };

        let export_rca = cells.next()
            .ok_or_else(|| format_err!("could not find export_rca value"))?;
        let export_rca = match export_rca {
            "NULL" => None,
            s => Some(s.parse::<f64>()?)
        };

        let import_rca = cells.next()
            .ok_or_else(|| format_err!("could not find import_rca value"))?;
        let import_rca = match import_rca {
            "NULL" => None,
            s => Some(s.parse::<f64>()?)
        };

        Ok(Self {
            year,
            origin,
            hs92,
            export_val,
            import_val,
            export_rca,
            import_rca,
        })
    }
}

#[derive(Debug, StructOpt)]
struct CliOpt {
    #[structopt(parse(from_os_str))]
    filepath: PathBuf,
}
