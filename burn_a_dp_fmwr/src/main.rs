use burn::{backend::Wgpu, prelude::*, tensor::activation::softmax};
use clap::{Parser, Subcommand};
use colored::Colorize;
use itertools::Itertools;
use nn::{EmbeddingConfig, LinearConfig};
use utils::{get_1_dim_length, get_batch};

mod bigram_model;
mod utils;

#[derive(Parser)]
struct CliArgs {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Subcommand)]
enum CliCommand {
    BasicExample,
    Linear,
    Softmax,
    Embedding,
    Ongoing,
}

pub type Backend = Wgpu;

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    match args.command {
        CliCommand::BasicExample => basic_example()?,
        CliCommand::Linear => linear()?,
        CliCommand::Softmax => my_softmax()?,
        CliCommand::Embedding => my_embedding()?,
        CliCommand::Ongoing => ongoing()?,
    }

    Ok(())
}

fn basic_example() -> anyhow::Result<()> {
    let device = Default::default();

    let tensor_1 = Tensor::<Backend, 2>::from_data([[1., 2., 3.], [2., 3., 4.]], &device);
    let tensor_2 = Tensor::<Backend, 2>::ones_like(&tensor_1); // EXPLANATION: the tensor will have
                                                               // the same shape, but filled with 1.

    println!("Tensor 1: {}", tensor_1);
    println!("Tensor 2: {}", tensor_2);
    println!("Tensor 1 + Tensor 2: {}", tensor_1 + tensor_2);

    Ok(())
}

fn linear() -> anyhow::Result<()> {
    let device = Default::default();

    let tensor = Tensor::<Backend, 1>::from_data([1., 2., 3.], &device);

    let linear = LinearConfig::new(3, 2).init(&device);

    println!("{}: {}\n", "Tensor before".bold(), tensor);

    let tensor_passed = linear.forward(tensor);

    println!("{}: {}\n", "Tensor after Linear".bold(), tensor_passed);

    Ok(())
}

fn my_softmax() -> anyhow::Result<()> {
    let device = Default::default();

    let tensor = Tensor::<Backend, 1>::from_data([1., 2., 3.], &device);

    println!("{}: {}\n", "Tensor before".bold(), tensor);

    let tensor_passed = softmax(tensor, 0);

    println!("{}: {}\n", "Tensor after softmax".bold(), tensor_passed);

    Ok(())
}

fn my_embedding() -> anyhow::Result<()> {
    let device = Default::default();

    let tensor = Tensor::<Backend, 1, Int>::from_data([0, 1, 2], &device);

    println!("{}: {}\n", "Tensor before".bold(), tensor);

    let embedding_layer = EmbeddingConfig::new(3, 10).init(&device);

    let tensor_passed = embedding_layer.forward(tensor.unsqueeze::<2>());

    println!("{}: {}\n", "Tensor after embedding".bold(), tensor_passed);

    Ok(())
}

const TRAIN_VAL_SPLIT_FACTOR: f32 = 0.9;

const BLOCK_SIZE: usize = 8;
const BATCH_SIZE: usize = 4;

fn ongoing() -> anyhow::Result<()> {
    let device = Default::default();

    let shakespear = include_str!("../shakespear_romeo.txt");

    println!(
        "{}:\n{}\n",
        "Symbols in the middle of R&J".bold(),
        &shakespear[20000..20600]
    );

    let vocab: Vec<char> = shakespear.chars().unique().collect();

    println!(
        "{}:\n{:?}\n",
        format!("Vocabulary (total {})", vocab.len()).bold(),
        vocab
    );

    println!("{}: {:?}\n", "'hello' encoded".bold(), encode("hello"));

    let test_arr = [104, 101, 108, 108, 111];

    println!(
        "{}: {}\n",
        format!("{:?} decoded", test_arr).bold(),
        decode(test_arr)
    );

    let text_encoded = encode(shakespear);

    let text_tensor = Tensor::<Backend, 1, Int>::from_data(text_encoded.as_slice(), &device);

    println!(
        "{}: {}\n",
        "Text tensor start".bold(),
        &text_tensor.clone().slice([0..100])
    );

    let text_length = get_1_dim_length(&text_tensor);

    let split = ((text_length as f32) * TRAIN_VAL_SPLIT_FACTOR) as usize;

    let train_tensor = text_tensor.clone().slice([0..split]);
    let test_tensor = text_tensor.clone().slice([split..text_length]);

    println!(
        "{}: {} / {}",
        format!("Train/test split (out of {})", text_length),
        get_1_dim_length(&train_tensor),
        get_1_dim_length(&test_tensor)
    );

    let (ex_x, ex_y) = get_batch(&train_tensor);

    println!(
        "{}: {}\n{}: {}\n",
        "Batch from train (inputs)".bold(),
        ex_x,
        "Batch from train (targets)".bold(),
        ex_y
    );

    Ok(())
}

fn encode(data: impl AsRef<str>) -> Vec<usize> {
    data.as_ref().chars().map(|c| c as usize).collect()
}

fn decode(data: impl AsRef<[usize]>) -> String {
    data.as_ref()
        .into_iter()
        .map(|c| (*c as u8) as char)
        .collect()
}
