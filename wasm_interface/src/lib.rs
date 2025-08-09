use std::sync::LazyLock;

use keypad::{
    ao3_css_transform::ao3_css_transform,
    keypad::{Config, Passcode, Themes, generate_keypad_css, generate_keypad_html},
};
use wasm_bindgen::prelude::wasm_bindgen;

const THEMES_STR: &str = include_str!("../../themes.toml");

static THEMES: LazyLock<Themes> =
    LazyLock::new(|| toml::from_str(THEMES_STR).expect("invalid themes format"));

#[wasm_bindgen]
pub struct KeypadConfig {
    config: Config,
}

// FIXME: eliminate panics

#[wasm_bindgen]
impl KeypadConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(passcode: &str, theme: &str) -> Result<Self, String> {
        if passcode.len() > 3 {
            return Err("password must be at most 3 digits long for use in ao3".into());
        }
        let config = Config {
            passcode: Passcode::new(passcode)?,
            theme: THEMES.0.get(theme).ok_or("provide a valid theme")?.clone(),
        };
        Ok(Self { config: config })
    }
}

#[wasm_bindgen]
pub fn themes() -> Vec<String> {
    THEMES.0.keys().cloned().collect()
}

#[wasm_bindgen]
pub fn wasm_generate_keypad_css(keypad_config: &KeypadConfig) -> String {
    let css = generate_keypad_css(&keypad_config.config);
    let ao3_css = ao3_css_transform(&css);
    ao3_css
}

#[wasm_bindgen]
pub fn wasm_generate_keypad_html(keypad_config: &KeypadConfig) -> String {
    let html = generate_keypad_html(&keypad_config.config);
    html
}
