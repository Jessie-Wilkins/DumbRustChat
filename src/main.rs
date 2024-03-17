#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

fn main() {
    LaunchBuilder::new(app).launch();
}

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