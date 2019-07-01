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
