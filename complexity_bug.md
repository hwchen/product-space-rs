# Bug in complexity calcs?

`std` appears to be different between pandas and np. I thought it meant std deviation, but now am not sure since these two libraries have different results.

```
>>> kp = pd.DataFrame([9.365921518323761,9.365168229974921,9.366119246144434,9.366618939884766])
>>> kp.std()
0    0.000602
dtype: float64
>>> np_kp = np.array([9.365921518323761,9.365168229974921,9.366119246144434,9.366618939884766])
>>> np_kp.std()
0.0005215135001035631
>>> 
```
I've implemented it in a way that matches numpy

Looks like pandas has by default ddof=1, which means that d will be count - 1 when calculating variance, instead of just count, which accounts for the difference in result. Manually subtracting one matches simoes `ps_calcs` exactly. Ask simoes which it is.

See: `pandas/core/nanops.py`, lines 581 for `nanstd`, 616 for `nanvar`, 561 for `_get_counts_nanvar`.

Comment from Cesar:
> The std part of the normalization does not change any of the properties of the indicator (it is a constant factor in a relative metric), so this is not a biggy. By convention, we want the standard deviation of the resulting values to be equal to 1.
