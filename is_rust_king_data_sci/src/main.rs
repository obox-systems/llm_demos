use linfa::traits::Fit;
use linfa_trees::{DecisionTree, SplitQuality};

use linfa::prelude::*;

fn main() -> anyhow::Result<()> {
    let (train, test) = linfa_datasets::iris().split_with_ratio(0.2);

    let model = DecisionTree::params()
        .split_quality(SplitQuality::Gini)
        .max_depth(Some(100))
        .min_weight_leaf(0.1)
        .min_weight_split(0.1)
        .fit(&train)?;

    let predictions = model.predict(&test);

    println!(
        "Accuracy: {}",
        predictions.confusion_matrix(&test)?.accuracy()
    );

    Ok(())
}
