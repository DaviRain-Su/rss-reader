use gloo::console::log;
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

#[derive(Deserialize, Serialize, Debug)]
struct RssArticle {}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct RssResponse {
    content: String,
}

#[derive(Deserialize, Serialize)]
struct RssRequest {
    address: String,
    subscribe_url: Vec<String>,
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| RssRequest {
        address: "davirain.eth".into(),
        subscribe_url: vec!["https://guoyu.mirror.xyz/".into()],
    });

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let response = gloo::net::http::Request::get("https://guoyu.mirror.xyz/")
                    .send()
                    .await
                    .unwrap();
                //                log!(serde_json::to_string_pretty(&response))
                log!(serde_json::to_string_pretty(&response.status()).unwrap());
            })
        })
    };

    html! {
        <>
            <h1>{"Hello World!"}</h1>
            <button {onclick}> {"rss click"}</button>
        </>
    }
}
