use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Deserialize, Serialize)]
struct MyObject {
    name: String,
    favor: String,
}

#[function_component(App)]
pub fn app() -> Html {
//    !("function component App Module!");
    let name = "Davirian";
    gloo::console::log!(name);
    let object = MyObject {
        name: "Davirian".to_owned(),
        favor: "Rust".to_owned(),
    };
    gloo::console::log!(serde_json::to_string_pretty(&object).unwrap());
    html! {
        <h1>{"Hello World!"}</h1>
    }
}
