use burn::{nn::Embedding, prelude::*, tensor::activation::softmax};
use nn::{
    loss::{CrossEntropyLoss, CrossEntropyLossConfig},
    EmbeddingConfig,
};

pub struct BigramModel<B: Backend> {
    embedding: Embedding<B>,
    cross_entropy: CrossEntropyLoss<B>,
}

impl<B: Backend> BigramModel<B> {
    pub fn new(vocab_size: usize, device: &B::Device) -> Self {
        BigramModel {
            embedding: EmbeddingConfig::new(vocab_size, vocab_size).init(device),
            cross_entropy: CrossEntropyLossConfig::new().init(device),
        }
    }

    pub fn forward(&self, tensor: Tensor<B, 2, Int>) -> Tensor<B, 3, Float> {
        self.embedding.forward(tensor)
    }

    pub fn forward_with_targets(
        &self,
        tensor: Tensor<B, 2, Int>,
        targets: Tensor<B, 2, Int>,
    ) -> (Tensor<B, 2, Float>, Tensor<B, 1, Float>) {
        // let [batch_size, times, channles] = logits.shape().dims();

        let logits = self.forward(tensor).squeeze(1);
        let targets = targets.squeeze(1);

        let loss = self.cross_entropy.forward(logits.clone(), targets);

        (logits, loss)
    }

    pub fn generate(&self, tensor: Tensor<B, 2, Int>, max_new_tokens: usize) -> Tensor<B, 2, Int> {
        let batch_length = tensor.shape().dims::<2>()[0];

        let mut tensor = tensor;

        for _ in 0..max_new_tokens {
            let logits: Tensor<B, 3, Float> = self.forward(tensor.clone());

            let times_length = logits.shape().dims::<3>()[1];
            let channels_length = logits.shape().dims::<3>()[2];

            let last_logits: Tensor<B, 2, Float> = logits
                .slice([
                    0..batch_length,
                    (times_length..(times_length + 1)),
                    0..channels_length,
                ])
                .squeeze(1);

            let probs = softmax(last_logits, 1);
        }

        todo!()
    }
}
