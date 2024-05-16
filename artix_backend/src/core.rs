
use std::convert::Infallible;
use std::io::Write;
use std::path::PathBuf;

pub fn generate_response(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
    // Choose the specific tokenizer and the architecture of the used model
    let tokenizer = llm::TokenizerSource::Embedded;
    let model_architecture = llm::ModelArchitecture::Bloom;
    // extract model, use model bloom-560m-q5_1-ggjt to output coherent text based on input text
    let model = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/bloom-560m-q5_1-ggjt.bin");
    //let model = PathBuf::from("/mini10/bloom-560m-q5_1-ggjt.bin");

    // dynamically load a language model and set it with proper arguments
    let model = llm::load_dynamic(
        Some(model_architecture),
        &model,
        tokenizer,
        Default::default(),
        llm::load_progress_callback_stdout,
    )?;

    // Start a session for the loaded model, preparing it for inference.
    let mut model_session = model.start_session(Default::default());
    let mut resp_content = String::new();
    let inference = model_session.infer::<Infallible>(
        model.as_ref(),
        &mut rand::thread_rng(), // randomize the generators to generate different text
        &llm::InferenceRequest {
            prompt: (&prompt).into(),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            maximum_token_count: Some(10),
        },
        &mut Default::default(),
        |response| match response {
            llm::InferenceResponse::PromptToken(token) | llm::InferenceResponse::InferredToken(token) => {
                print!("{token}");
                std::io::stdout().flush().unwrap();
                resp_content.push_str(&token);
                Ok(llm::InferenceFeedback::Continue)
            }
            _ => Ok(llm::InferenceFeedback::Continue),
        },
    );

    // handle the inference result
    match inference {
        Ok(_) => Ok(resp_content),
        Err(e) => Err(Box::new(e)),
    }
}