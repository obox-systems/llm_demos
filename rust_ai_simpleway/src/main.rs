use anyhow::Context;
use dialoguer::Input;
use rust_bert::{
    gpt_neo::{
        GptNeoConfigResources, GptNeoMergesResources, GptNeoModelResources, GptNeoVocabResources,
    },
    pipelines::{
        common::ModelType,
        text_generation::{TextGenerationConfig, TextGenerationModel},
    },
    resources::RemoteResource,
};

fn main() -> anyhow::Result<()> {
    let model_res = RemoteResource::from_pretrained(GptNeoModelResources::GPT_NEO_1_3B);
    let config_res = RemoteResource::from_pretrained(GptNeoConfigResources::GPT_NEO_1_3B);
    let vocab_res = RemoteResource::from_pretrained(GptNeoVocabResources::GPT_NEO_1_3B);
    let merges_res = RemoteResource::from_pretrained(GptNeoMergesResources::GPT_NEO_1_3B);

    let gen_conf = TextGenerationConfig {
        model_type: ModelType::GPTNeo,
        model_resource: Box::new(model_res),
        config_resource: Box::new(config_res),
        vocab_resource: Box::new(vocab_res),
        merges_resource: Box::new(merges_res),
        num_beams: 5,
        no_repeat_ngram_size: 2,
        max_length: 100,
        ..Default::default()
    };

    let model = TextGenerationModel::new(gen_conf)?;

    let mut conversation = String::from("You are a helpful assistant.\n\n");

    loop {
        let input: String = Input::new()
            .with_prompt("User")
            .interact_text()
            .context("unable to get input from console")?;

        if input == "/q" {
            break;
        }

        conversation.push_str(format!("User: {}\n\nAssistant: ", input).as_str());

        let output = model.generate(&[conversation.as_str()], None);

        conversation.push_str(format!("{}\n\n", output.get(0).unwrap_or(&"".to_string())).as_str());
    }

    Ok(())
}
