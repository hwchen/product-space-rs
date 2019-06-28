Testing against `alexandersimoes/ps_calcs`, `oec_to_mcp.py`
Year: 2017

`ps_calcs`:
```
mochi:ps_calcs (master|✚1) > time python oec_to_mcp.py
hs92
4701    175.777804
0204    149.962669
3501    149.084057
0507    129.925431
0402    123.573472
0405    107.507062
0208     71.614561
4403     60.366482
1603     55.275577
0409     52.921210
Name: nzl, dtype: float64
origin
svk    4.441051
jpn    3.202511
gib    3.002885
cze    2.725152
can    2.693870
esp    2.677362
deu    2.593409
svn    2.500016
gbr    2.486583
mex    2.352253
Name: 8703, dtype: float64

The top 10 HS product codes that Brazil has RCA in:

hs92
2825    1.0
1521    1.0
1601    1.0
1602    1.0
1603    1.0
7103    1.0
2821    1.0
1701    1.0
4012    1.0
4011    1.0
Name: bra, dtype: float64
```
`product-space-rs`:
```
mochi:product-space (master|✔) > cargo build --release --bin oec && time target/release/oec ../ps_calcs/data/year_origin_hs92_4.tsv --year 2017
    Finished release [optimized] target(s) in 0.03s
Reading data from: "../ps_calcs/data/year_origin_hs92_4.tsv"
Export val
ago::0106 Ok(2683401.0)
bdi::1513 Ok(213505.0)
RCA
ago::0106 Ok(1.0989446890765864)
bdi::1513 Ok(2.888392654640275)
RCA test against simoes ps_calcs
nzl::0204, expect 149.962669: Ok(149.96266870448784)
svk::8703, expect 4.441051: Ok(4.441050571648362)
fair share test against simoes ps_calcs
bra::2825, expect 1.0: Ok(1.0)
bra::1521, expect 1.0: Ok(1.0)
bra::1601, expect 1.0: Ok(1.0)
bra::1602, expect 1.0: Ok(1.0)
```

`ps_calcs` is about 6.20s, `product-space-rs` is about 1.42s
The work should be pretty similar;
although `product-space-rs` reads more directly into a matrix and skips unneeded rows, while `ps_calcs` reads everything into a pandas df first;
although `ps_calcs` does do a sort for each printing of "top x", but not doing any of those print "top x" basically doesn't change the timing.

(use `cat year_origin_hs92_4.tsv| ag '\t1513\t'  | ag '2017\t' | ag bdi` to check particular values)
