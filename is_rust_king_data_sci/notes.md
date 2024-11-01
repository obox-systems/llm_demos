# [Is Rust the New King of Data Science?](https://www.youtube.com/watch?v=mlcSpxicx-4&list=PLeb33PCuqDde8NiI_am5g7b2WWnyggE5t&index=4)

## Rationale

- Rust has all bare necesseties for data science and machine learning.
- Nearly all (*I won't say exactly all*) important algorithms are already implemented.

## Linfa

Analogous to scikit-learn` (Python)

### How to make a simple prediction model

Word: fetures - information from which you decide, label - final decision.

1. Have a dataset with features and labels. You can do that easily with `ndarray`.
2. Split features and labels: easily done with slices in `ndarray`.
3. Make a linfa dataset from features and labels.
4. Transform all data to what you need
5. Make an algorithm class, e.g.: `DecisionTree`.
6. Fit model to data. Like in Python `scikit-learn`.

And do whatever you want. You can for example export the internals of the decision tree.
