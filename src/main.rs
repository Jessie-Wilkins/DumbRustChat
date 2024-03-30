#![allow(non_snake_case, unused)]

use llm_inference_service;

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;



fn main() {

    let mut inference_service = llm_inference_service::LlmInferenceService::new();
        
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

