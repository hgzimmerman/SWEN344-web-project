use serde::Serialize;
use serde::Deserialize;
use wasm_typescript_definition::TypescriptDefinition;

use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, TypescriptDefinition)]
pub struct Login {
    client_id: String
}

#[derive(Clone, Debug, Serialize, Deserialize, TypescriptDefinition)]
pub struct Yeet {
    hello: usize,
    there: f32,
    general: String,
    kenobi: Option<String>
}


#[derive(TypescriptDefinition, Serialize, Deserialize)]
#[serde(tag = "tag", content = "fields")]
pub enum FrontendMessage {
    Init { id: String, },
    ButtonState { selected: Vec<String>, time: u32, },
    Render { html: String, time: u32, },
}

