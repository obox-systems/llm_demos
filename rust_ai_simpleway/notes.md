# [Rust Artificial Intelligence (The Simple Way)](https://www.youtube.com/watch?v=StMP7g-0wK4&list=PLeb33PCuqDde8NiI_am5g7b2WWnyggE5t&index=17)

IMHO: A strange way. I wonder, did that GPT-neo company really gave him money? For 50k views?

## Preliminary things

- Install "libtorch"

## Model internals

- ".ot"
- config
- vocab
- messages

## Used technologies

- GPT-Neo (*WTF is this*)
- rust-bert

rust-bert actually has "bindings" for GPT-Neo.

## What was done in the video

1. Load `RemoteResource` (those 4 files).
2. Make `TextGenerationConfig`, that uses those resources.
3. Make a `TextGenerationModel` out of the config.
4. Then based on that model you would call `.generate()`.
