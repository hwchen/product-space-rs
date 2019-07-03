Here's timings, run on test data (year 2017 only) in oec example 1:

 ```
 mcp read time: 0.774
 mcp construct time: 0.610
 rca calc time: 0.005
 prox calc time: 0.055
 density calc time: 0.061
 ```

So even calculating from scratch (the exports Mcp) is about 0.12s In comparison, the clickhouse part of tesseract will take anywhwere from 0.05s for very fast queries, to perhaps 0.2s for a little slower than avg.I guess my question is: is there any reason why rca, proximity, and density can't be calculated and cached ahead of time? If I'm understanding correctly, you'd just have to select one row from the density in order to provide the answer to a query about a country's product density.
I may also check for optimizations at a further point (this is just naive, and using the linalg lib), but I think the perf is good enough for now.

Also, in terms of utilization, in a threadpool I can run full throttle 4 cores, 100 x (proximity + density), takes about 5.5s. 20 req/s is not super great, and each individual request was about double the time of when single-threaded, possibly because of moving the matrices around in memory. Since you mentioned about 200 concurrent visitors, an 8 or 16 core machine should be able to take care of calcs easily, even if from Mcp exports (assuming they don't all click on a calc at once)

Related to the RCA 1.5, this was a threshold that we use for some calculations (as cesar mentioned in the oec channel) where we will use a binary variable of < 1.5 = 0 and >= 1.5 = 1 (meaning you have rca for that product or you dont). And typically we use 1.0 as the cutoff though sometime we want to try other thresholds to see how the results vary.

For the calculations, the aggregation at every step would be about better. But we can optimize by doing a few ones. For instance, proximity can be noisy, so taking the average of the three previous years would help. RCA is noisy too, so using RCA>1 in three previous years is a good backwards condition. With those two, density would be a bit less noisy
