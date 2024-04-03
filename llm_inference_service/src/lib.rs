
extern crate reqwest;

use std::io::Write;
use llm::{Model, ModelParameters};
use std::fs::File;
use std::fs::create_dir;
use std::path::Path;

pub struct LlmInferenceService {
    model: Box<dyn Model>
}

impl LlmInferenceService {
    pub fn new() -> Self {
        let file_path = "model/tinyllama-1.1b-chat-v1.0.Q2_K.gguf";
        if !Path::new(file_path).exists() {
            let resp = reqwest::blocking::get("https://huggingface.co/TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF/blob/main/tinyllama-1.1b-chat-v1.0.Q2_K.gguf").expect("Failed to download");
            let _ = create_dir("model");
            let out = File::create(file_path);
            let _ = out.expect("Failed to Write File").write_all(resp.bytes().as_ref().expect("Failed to convert to bytesd"));
        }
        let parameters = ModelParameters {           
            prefer_mmap: true,
            context_size: 2048,
            lora_adapters: None,
            use_gpu: false,
            gpu_layers: None,
            rope_overrides: None,
        };  
        let model = llm::load(
            // path to GGML file
            std::path::Path::new(file_path),
            llm::TokenizerSource::Embedded,
            // llm::ModelParameters
            parameters,
            // load progress callback
            llm::load_progress_callback_stdout
        )
        .unwrap_or_else(|err| panic!("Failed to load model: {err}"));

        Self { model }
    }

    pub fn infer(&self, query: &str) {
        let mut session = self.model.start_session(Default::default());
        let res = session.infer::<std::convert::Infallible>(
            // model to use for text generation
            self.model.as_ref(),
            // randomness provider
            &mut rand::thread_rng(),
            // the prompt to use for text generation, as well as other
            // inference parameters
            &llm::InferenceRequest {
                prompt: query.into(),
                parameters: &llm::InferenceParameters::default(),
                play_back_previous_tokens: false,
                maximum_token_count: Some(50),
            },
            // llm::OutputRequest
            &mut Default::default(),
            // output callback
            |r| match r {
                llm::InferenceResponse::PromptToken(t) | llm::InferenceResponse::InferredToken(t) => {
                    print!("{t}");
                    std::io::stdout().flush().unwrap();
                    Ok(llm::InferenceFeedback::Continue)
                }
                _ => Ok(llm::InferenceFeedback::Continue),
            }
        );
        match res {
            Ok(result) => println!("\n\nInference stats:\n{result}"),
            Err(err) => println!("\n{err}"),
        }
    }
}