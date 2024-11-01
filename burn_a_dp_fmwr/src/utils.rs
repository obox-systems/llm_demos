use crate::{BATCH_SIZE, BLOCK_SIZE};
use burn::prelude::*;
use rand::Rng;

use crate::Backend;

pub fn get_1_dim_length(tensor: &Tensor<Backend, 1, Int>) -> usize {
    tensor.shape().dims::<1>()[0]
}

fn get_sample(
    tensor: &Tensor<Backend, 1, Int>,
) -> (Tensor<Backend, 1, Int>, Tensor<Backend, 1, Int>) {
    let length = get_1_dim_length(tensor);

    let offset = rand::thread_rng().gen_range(0..(length - BLOCK_SIZE - 1));

    (
        tensor.clone().slice([offset..(offset + BLOCK_SIZE)]),
        tensor.clone().slice([offset + 1..offset + BLOCK_SIZE + 1]),
    )
}

pub fn get_batch(
    tensor: &Tensor<Backend, 1, Int>,
) -> (Tensor<Backend, 2, Int>, Tensor<Backend, 2, Int>) {
    let (xs, ys): (Vec<Tensor<Backend, 1, Int>>, Vec<Tensor<Backend, 1, Int>>) = (0..BATCH_SIZE)
        .into_iter()
        .map(|_| get_sample(tensor))
        .unzip();

    (Tensor::stack(xs, 0), Tensor::stack(ys, 0))
}

pub fn multinomial(probs: impl AsRef<[f32]>) -> usize {
    let mut sample = rand::thread_rng().gen_range(0.0..probs.as_ref().iter().sum());

    for (i, &p) in probs.as_ref().iter().enumerate() {
        sample -= p;
        if sample <= 0.0 {
            return i;
        }
    }

    return 0;
}
