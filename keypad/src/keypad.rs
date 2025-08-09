use std::{collections::HashMap, env, fs};

use serde::Deserialize;
use serde::de::Error;

#[derive(Clone)]
pub struct Theme {
    border: String,
    colors: Vec<String>,
    pattern: [String; 3],
}

// TODO: kinda funky
impl<'de> Deserialize<'de> for Theme {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[allow(unused)]
        struct ThemeUnverified {
            border: String,
            colors: Vec<String>,
            pattern: [String; 3],
        }
        let theme = ThemeUnverified::deserialize(deserializer)?;
        for x in theme.pattern.iter() {
            for c in x.chars() {
                let _ = theme
                    .colors
                    .get(
                        c.to_digit(10)
                            .ok_or_else(|| D::Error::custom("malformed pattern in theme"))?
                            as usize,
                    )
                    .ok_or_else(|| D::Error::custom("malformed pattern in theme"))?;
            }
        }
        Ok(Theme {
            border: theme.border,
            colors: theme.colors,
            pattern: theme.pattern,
        })
    }
}

#[derive(Deserialize)]
pub struct Themes(pub HashMap<String, Theme>);

pub struct Passcode(String);

impl Passcode {
    pub fn new(x: &str) -> Result<Passcode, String> {
        for c in x.chars() {
            match c {
                '1'..='9' => (),
                _ => {
                    return Err(format!(
                        "'{}' is not a valid passcode. please use only numbers 1-9",
                        x
                    ));
                }
            }
        }
        if x.len() <= 1 {
            return Err("please use more than 1 digit".into());
        }
        if x.len() > 4 {
            return Err("please do not use more than 4 digits".into());
        }
        // if x.len() > 3 {
        //     println!("\x1b[0;33m[WARNING] a four digit passcode will not fit on ao3\x1b[0m");
        // }
        Ok(Self(x.into()))
    }
    pub fn get(&self) -> &str {
        &self.0
    }
}

pub struct Config {
    pub passcode: Passcode,
    pub theme: Theme,
}

const HTML_PRELUDE: &str = r##"
<div class="kpmcon">
<a href="#r_">reset</a>
"##;
const HTML_EPILOGUE: &str = r##"
</div>
<details class="secret">
<summary id="this-element-is-important-do-not-delete-it"></summary>
<h1 id="put-your-own-content-here">my secret header</h1>
</details>
"##;

fn emit_keypad_layer_html(config: &Config, buf: &mut String, layers: &mut String) {
    if layers.len() == config.passcode.0.len() {
        return;
    }
    buf.push_str(&format!(r#"<p class="kp kp_{}">"#, layers));
    for i in 1..=9 {
        layers.push((b'0' + i as u8) as char);
        buf.push_str(&format!(r##"<a class="bt" href="#c{}"></a>"##, layers));
        layers.pop();
    }
    buf.push_str("</p>");
    for i in 1..=9 {
        layers.push((b'0' + i as u8) as char);
        emit_keypad_layer_anchor_html(config, buf, layers);
        emit_keypad_layer_html(config, buf, layers);
        layers.pop();
    }
}
fn emit_keypad_layer_anchor_html(config: &Config, buf: &mut String, layers: &mut String) {
    if layers.len() > config.passcode.0.len() {
        panic!("uwu");
    }
    let class = if layers.len() == config.passcode.0.len() {
        "kpfin"
    } else {
        "kpnfin"
    };
    buf.push_str(&format!(
        r##"<p class="invap"><a class="inva {}" name="c{}" id="c{}"></a></p>"##,
        class, layers, layers
    ));
}

const CSS_PRELUDE: &str = r##"
.kp {
    display: grid;
    grid-template-columns: min-content min-content min-content;
}
.bt {
    width: 1.2em;
    height: 1.2em;
    font-size: xx-large;
    padding: 10px;
    margin: 3px;
    display: inline-block;
    background-color: white;
    display: flex;
    justify-content: center;
    align-items: center;
    text-decoration: none;
    font-family: 'Courier New', Courier, monospace;
    font-weight: 900;
    color: black;
}
.bt:nth-child(1)::before {
    content: "1";
}
.bt:nth-child(2)::before {
    content: "2";
}
.bt:nth-child(3)::before {
    content: "3";
}
.bt:nth-child(4)::before {
    content: "4";
}
.bt:nth-child(5)::before {
    content: "5";
}
.bt:nth-child(6)::before {
    content: "6";
}
.bt:nth-child(7)::before {
    content: "7";
}
.bt:nth-child(8)::before {
    content: "8";
}
.bt:nth-child(9)::before {
    content: "9";
}
.bt:hover {
    filter: brightness(97%);
}
.bt:focus {
    filter: brightness(85%);
}
.inva {
    display:none;
}
.invap {
    display:none;
}
.kp {
    display: none;
}
.kpmcon:not(:has(a.kpnfin:target)) .kp_ {
    display: grid;
}
.invap:has(.inva:target) + .kp {
    display: grid;
}
.kpmcon:not(:has(#c***:target)):has(.kpfin:target)::after {
    content: "Incorrect Passcode";
    color: red;
}
.secret {
    display: none;
}
.kpmcon:has(#c***:target)~.secret {
    display: block;
}
.secret[open] {
    display: block;
}
.secret summary::after {
    content: "🔑 click to open";
}
.secret summary:hover {
    color: green;
}
.secret[open] summary::after {
    content: "🔑 click to close";
}
.secret[open] summary:hover {
    color: red;
}
.kpmcon:has(~.secret[open]) {
    display: none;
}
details.secret summary::-webkit-details-marker, details.secret summary::marker {
    display: none;
    content: "";
}
"##;

fn generate_keypad_theme_css(config: &Config, buf: &mut String) {
    let border = if config.theme.border.is_empty() {
        "1px solid black"
    } else {
        &config.theme.border
    };
    buf.push_str(&format!(
        r##".bt {{
    border: {border};
}}
"##
    ));
    let colors = config
        .theme
        .colors
        .iter()
        .map(|x| x.replace(";", ";\n").trim().to_owned())
        .collect::<Vec<_>>();
    for (row, x) in config.theme.pattern.iter().enumerate() {
        for (col, i) in x.chars().enumerate() {
            let p = row * 3 + col + 1;
            let content = &colors
                .get(i.to_digit(10).expect("malformed pattern in theme") as usize)
                .expect("malformed pattern in themes");
            buf.push_str(&format!(
                r##".bt:nth-child({p}) {{
    {content};
}}
"##
            ));
        }
    }
}

pub fn generate_keypad_html(config: &Config) -> String {
    let mut html = String::new();
    html.push_str(HTML_PRELUDE);
    emit_keypad_layer_html(config, &mut html, &mut String::new());
    html.push_str(HTML_EPILOGUE);
    html = html.trim().to_owned();
    html
}
pub fn generate_keypad_css(config: &Config) -> String {
    let mut css = String::new();
    css.push_str(CSS_PRELUDE);
    generate_keypad_theme_css(config, &mut css);
    let css = css.replace("***", &config.passcode.0);
    css
}
