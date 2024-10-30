use anyhow::Context;
use async_openai::{
    types::{
        ChatCompletionFunctionsArgs, ChatCompletionRequestAssistantMessage,
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestAssistantMessageContent,
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestToolMessageArgs,
        ChatCompletionRequestUserMessageArgs, ChatCompletionToolArgs,
        CreateChatCompletionRequestArgs, FunctionObject, FunctionObjectArgs,
    },
    Client,
};
use chrono::Local;
use dialoguer::Input;
use rand::Rng;
use serde::Deserialize;
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();

    let functions = vec![
        ChatCompletionToolArgs::default()
            .function(
                FunctionObjectArgs::default()
                    .name("get_current_date_time")
                    .description("Get the current date and time")
                    .build()?,
            )
            .build()?,
        ChatCompletionToolArgs::default()
            .function(
                FunctionObjectArgs::default()
                    .name("get_random_number_in_range")
                    .description("Get a random number in a range")
                    .parameters(json!( {
                        "type": "object",
                        "properties": {
                            "min": {
                                "type": "number",
                                "description": "minimum of the range (inclusive)"
                            },
                            "max": {
                                "type": "number",
                                "description": "maximum of the range (inclusive)"
                            },
                        },
                        "required": ["min", "max"]
                    }))
                    .build()?,
            )
            .build()?,
    ];

    let mut messages = vec![ChatCompletionRequestSystemMessageArgs::default()
        .content("You are a helpful assistant.")
        .build()?
        .into()];

    loop {
        let input: String = Input::new()
            .with_prompt("User")
            .interact_text()
            .context("unable to get input from console")?;

        if input == "/q" {
            break;
        }

        messages.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(input)
                .build()?
                .into(),
        );

        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-4o-mini")
            .messages(messages.clone())
            .tools(functions.clone())
            .tool_choice("auto")
            .build()?;

        let response = client
            .chat()
            .create(request)
            .await
            .context("unable to send messages to OpenAI")?;

        if let Some(response) = response.choices.get(0) {
            if let Some(c) = response.clone().message.content {
                messages.push(
                    ChatCompletionRequestAssistantMessageArgs::default()
                        .content(ChatCompletionRequestAssistantMessageContent::Text(c))
                        .build()?
                        .into(),
                )
            } else if let Some(func_calls) = response.clone().message.tool_calls {
                messages.push(
                    ChatCompletionRequestAssistantMessageArgs::default()
                        .tool_calls(func_calls)
                        .build()?
                        .into(),
                )
            };

            if let Some(func_calls) = response.clone().message.tool_calls {
                for f in func_calls {
                    println!("Called function '{}'", f.function.name);
                    match f.function.name.as_str() {
                        "get_current_date_time" => {
                            let timestamp = Local::now().to_string();

                            messages.push(
                                ChatCompletionRequestToolMessageArgs::default()
                                    .tool_call_id(f.id)
                                    .content(timestamp)
                                    .build()?
                                    .into(),
                            );
                        }

                        "get_random_number_in_range" => {
                            #[derive(Deserialize)]
                            struct RandomArgs {
                                min: isize,
                                max: isize,
                            }

                            let args: RandomArgs = serde_json::from_str(&f.function.arguments)?;

                            let num = rand::thread_rng().gen_range(args.min..=args.max);

                            messages.push(
                                ChatCompletionRequestToolMessageArgs::default()
                                    .tool_call_id(f.id)
                                    .content(num.to_string())
                                    .build()?
                                    .into(),
                            );
                        }

                        otherwise => {
                            println!("LLM requested unknown function: {}", otherwise);
                        }
                    }
                }

                let new_request = CreateChatCompletionRequestArgs::default()
                    .model("gpt-4o-mini")
                    .messages(messages.clone())
                    .build()?;

                let new_response = client.chat().create(new_request).await?;

                if let Some(response) = new_response
                    .choices
                    .get(0)
                    .map(|c| c.clone().message.content)
                    .flatten()
                {
                    println!("Assistant: {}", response);
                } else {
                    println!("No response");
                }
            } else if let Some(response) = response.clone().message.content {
                println!("Assistant: {}", response);
            } else {
                println!("No response");
            }
        } else {
            println!("No response");
        }
    }

    Ok(())
}
