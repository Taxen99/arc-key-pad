use std::{collections::HashMap, env, fs};

use serde::Deserialize;

#[derive(Deserialize)]
struct Theme {
    border: String,
    colors: Vec<String>,
    pattern: [String; 3],
}

#[derive(Deserialize)]
struct Themes(HashMap<String, Theme>);

struct Config {
    depth: usize,
    theme: Theme,
}

const HTML_PRELUDE: &str = r##"
<div class="kpmcon">
<a href="#r_">reset</a>
"##;
const HTML_EPILOGUE: &str = r##"
</div>
"##;

fn emit_keypad_layer_html(config: &Config, buf: &mut String, layers: &mut String) {
    if layers.len() == config.depth {
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
    if layers.len() > config.depth {
        panic!("uwu");
    }
    let class = if layers.len() == config.depth {
        "kpfin"
    } else {
        "kpnfin"
    };
    buf.push_str(&format!(
        r##"<p class="invap"><a class="inva {}" name="c{}" id="c{}"></a></p>"##,
        class, layers, layers
    ));
}

fn generate_keypad_html(config: &Config) -> String {
    let mut html = String::new();
    html.push_str(HTML_PRELUDE);
    emit_keypad_layer_html(config, &mut html, &mut String::new());
    html.push_str(HTML_EPILOGUE);
    html
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
"##;

fn generate_keypad_css(config: &Config) -> String {
    let mut css = String::new();
    css.push_str(CSS_PRELUDE);
    let border = if config.theme.border.is_empty() {
        "1px solid black"
    } else {
        &config.theme.border
    };
    css.push_str(&format!(
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
            css.push_str(&format!(
                r##".bt:nth-child({p}) {{
    {content};
}}
"##
            ));
        }
    }
    css
}

const HTML_TEMPLATE: &str = r##"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><title>Document</title><link rel="stylesheet" href="style.css"></head><body>
@@@
<details class="secret">
<summary></summary>
<h1>my secret header</h1>
</details>
</body></html>"##;

const CSS_TEMPLATE: &str = r##"
@@@
.secret {
	display: none;
}
.kpmcon:has(#c124:target)~.secret {
	display: block;
}
.secret[open] {
	display: block;
}
.secret summary::marker {
	content: "🔑 click to open";
}
.secret summary:hover {
	color: green;
}
.secret[open] summary::marker {
	content: "🔑 click to close";
}
.secret[open] summary:hover {
	color: red;
}
.kpmcon:has(~.secret[open]) {
	display: none;
}
"##;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let pwd = args.get(1).expect("please provide passcode");
    for c in pwd.chars() {
        match c {
            '1'..='9' => (),
            _ => panic!(
                "'{}' is not a valid passcode. please use only numbers 1-9",
                pwd
            ),
        }
    }
    assert!(pwd.len() > 1, "please use more than 1 digit");
    assert!(pwd.len() <= 4, "please do not use more than 4 digits");
    if pwd.len() > 3 {
        println!("\x1b[0;33m[WARNING] a four digit passcode will not fit on ao3\x1b[0m");
    }
    let theme = args.get(2).map(|x| x.as_str()).unwrap_or("default");

    let themes = fs::read_to_string("themes.toml").expect("provide themes.toml");
    let mut themes: Themes = toml::from_str(&themes).expect("invalid themes format");
    let theme = themes
        .0
        .remove(theme)
        .expect(&format!("'{}' is not a valid theme", theme));

    let config = Config {
        depth: pwd.len(),
        theme,
    };
    fs::create_dir_all("output").expect("could not create output directory");
    let html = generate_keypad_html(&config);
    let html = HTML_TEMPLATE.replace("@@@", &html);
    fs::write("output/index.html", html).expect("could not write file");
    let css = generate_keypad_css(&config);
    let css = CSS_TEMPLATE.replace("@@@", &css);
    let css = css.replace("***", &pwd);
    fs::write("output/style.css", css).expect("could not write file");
}
