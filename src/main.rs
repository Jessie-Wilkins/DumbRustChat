#![allow(non_snake_case, unused)]
extern crate reqwest;

use std::io;
use std::fs::File;
use std::fs::create_dir;
use std::path::Path;

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

use std::io::Write;
use llm::ModelArchitecture::Llama;
use llm::{Model, 
    OutputRequest, ModelParameters};

pub struct LlmInferenceService {
    model: Box<dyn Model>
}

impl LlmInferenceService {
    pub fn new() -> Self {
        let file_path = "model/tinyllama-1.1b-chat-v1.0.Q2_K.gguf";
        if !Path::new(file_path).exists() {
            let mut resp = reqwest::blocking::get("https://huggingface.co/TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF/blob/main/tinyllama-1.1b-chat-v1.0.Q2_K.gguf").expect("Failed to download");
            let mut out = File::create(file_path);
            out.expect("Failed to Write File").write_all(resp.bytes().as_ref().expect("Failed to convert to bytes"));
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


fn main() {

    let mut inference_service = LlmInferenceService::new();
        
    LaunchBuilder::new(app).launch();
}

//Look into using Rustformers-gguf and a highly quantized version of a very small LLM (like https://huggingface.co/TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF ).

fn app(cx: Scope) -> Element {
    let mut user_message = use_state(cx, || "".to_string());
    let mut bot_message = use_state(cx, || "".to_string());

    cx.render(rsx! {
        input {
            oninput: move |evt| user_message.set(evt.value.clone()),
        }
        div {}
        button { 
            onclick: move |_| bot_message.set("You said: ".to_string()+user_message), 
            "Submit" 
        }
        h2 {"{bot_message}"}
    })
}

