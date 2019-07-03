oec-example:
    cargo build --release --example oec && time target/release/examples/oec ../ps_calcs/data/year_origin_hs92_4.tsv --year 2017
