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

Thanks! We denoise more if we do it for 1 year, for each of the previous 3 years, and then do the aggregations. That is RCA=1 if RCA(t) & RCA(t-1) & RCA(t-2) > 1 (we say a country has comparative advantage if it is observed for three consecutive years). Similarly Phi(p,p’) is average of three previous years. Thanks!

ok, I think that makes sense. So for rca and proximities I'll do the calculation all the way through for each year, and then aggregate, and then those will feed into density.

Cesar:
I think there are two use cases. One use case, is more centralized. There, we provide a set of predictions and metrics and use a criteria defined by us. This is priority, since it is the simplest. This use case involves also finding the right filtering of the data. We usually cut small countries (eg Tuvalu) and products that have few exporters and low volume, since they add noise to the data. Márcio is working on a data science project here and is looking at these distributions. Approximately, we want countries with at least 4m people and 1.5 billion in exports. Once we find those filters, the matrices should not change (unless we update the data). In the more analytical use case, a user may want to look at a custom grouping of countries (eg rcas for French speaking countries in west Africa). In that case we want to keep the proximities and complexities fixed, but we want to calculate rcas and densities for the custom grouping. This would be a second stage.
