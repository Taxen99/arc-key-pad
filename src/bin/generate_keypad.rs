use std::{env, fs};

struct Config {
    depth: usize,
}

const HTML_PRELUDE: &str = r##"
<a href="#r_">reset</a>
<div class="kpmcon">
"##;
const HTML_EPILOGUE: &str = r##"
</div>
"##;

// fn emit_kp_div(config: &Config, buf: &mut String, layers: &mut String) {
//     buf.push_str(&format!(r#"<div class="kp kp_{}">"#, layers));
//     for i in 1..=9 {
//         // <a href="#c1" class="bt">1</a>
//         layers.push((b'0' + i as u8) as char);
//         buf.push_str(&format!(
//             r##"<a href="#c{}" class="bt" name="c{}" id="c{}">{}</a>"##,
//             layers, layers, layers, i
//         ));
//         layers.pop();
//     }
//     for i in 1..=9 {
//         layers.push((b'0' + i as u8) as char);
//         emit_keypad_layer(config, buf, layers);
//         layers.pop();
//     }

//     buf.push_str("</div>");
// }

fn emit_keypad_layer_html(config: &Config, buf: &mut String, layers: &mut String) {
    if layers.len() == config.depth {
        return;
    }
    // buf.push_str("<div>");
    // for i in 1..=9 {
    //     // <a href="#c1" class="bt">1</a>
    //     layers.push((b'0' + i as u8) as char);
    //     buf.push_str(&format!(
    //         r##"<a class="inva" name="c{}" id="c{}"></a>"##,
    //         layers, layers
    //     ));
    //     layers.pop();
    // }
    buf.push_str(&format!(r#"<p class="kp kp_{}">"#, layers));
    for i in 1..=9 {
        // <a href="#c1" class="bt">1</a>
        layers.push((b'0' + i as u8) as char);
        buf.push_str(&format!(
            // r##"<a href="#c{}" class="bt" name="c{}" id="c{}">{}</a>"##,
            r##"<a class="bt" href="#c{}"></a>"##,
            layers
        ));
        layers.pop();
    }
    buf.push_str("</p>");
    for i in 1..=9 {
        layers.push((b'0' + i as u8) as char);
        emit_keypad_layer_html(config, buf, layers);
        layers.pop();
    }

    // buf.push_str("</div>");
}
fn emit_keypad_layer_anchor_html(config: &Config, buf: &mut String, layers: &mut String) {
    if layers.len() == config.depth {
        return;
    }
    let class = if layers.len() == config.depth - 1 {
        "kpfin"
    } else {
        "kpnfin"
    };
    for i in 1..=9 {
        layers.push((b'0' + i as u8) as char);
        buf.push_str(&format!(
            r##"<a class="inva {}" name="c{}" id="c{}"></a>"##,
            class, layers, layers
        ));
        layers.pop();
    }
    for i in 1..=9 {
        layers.push((b'0' + i as u8) as char);
        emit_keypad_layer_anchor_html(config, buf, layers);
        layers.pop();
    }

    // buf.push_str("</div>");
}

fn generate_keypad_html(config: &Config) -> String {
    let mut html = String::new();
    html.push_str(HTML_PRELUDE);
    html.push_str(r##"<p class="kplincon">"##);
    emit_keypad_layer_anchor_html(config, &mut html, &mut String::new());
    html.push_str("</p>");
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
    border: 5px ridge white;
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

.bt:nth-child(even) {
    color: pink;
}
.bt:nth-child(odd) {
    color: #89cff0;
}

.bt:hover {
    filter: brightness(95%);
}
.bt:focus {
    filter: brightness(70%);
}

.inva {
display:none;
}
.kp {
    display: none;
}
.kpmcon:not(:has(a.kpnfin:target)) .kp_ {
    display: grid;
}
"##;

fn emit_keypad_layer_css(config: &Config, buf: &mut String, layers: &mut String) {
    if layers.len() == config.depth {
        return;
    }

    /*
        .kp {
        display: none;
        }
       #cx:target ~ .kp_x {
         display: grid;
       }
    */

    // buf.push_str(&format!(r#"<div class="kp kp_{}">"#, layers));
    for i in 1..=9 {
        // <a href="#c1" class="bt">1</a>
        layers.push((b'0' + i as u8) as char);
        buf.push_str(&format!(
            r##"
.kplincon:has(#c{}:target) ~ .kp_{}{{
display: grid;
}}
"##,
            layers, layers
        ));
        layers.pop();
    }
    for i in 1..=9 {
        layers.push((b'0' + i as u8) as char);
        emit_keypad_layer_css(config, buf, layers);
        layers.pop();
    }

    // buf.push_str("</div>");
}

fn generate_keypad_css(config: &Config) -> String {
    let mut css = String::new();
    css.push_str(&CSS_PRELUDE);
    emit_keypad_layer_css(config, &mut css, &mut String::new());
    css
}

const HTML_TEMPLATE: &str = r##"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><title>Document</title><link rel="stylesheet" href="style.css"></head><body>
@@@
<div class="secret">
<h1>my secret header</h1>
</div>
</body></html>"##;

const CSS_TEMPLATE: &str = r##"
@@@
.secret {
    display: none;
}
.kpmcon:has(#c***:target) ~ .secret {
    display: block;
}
.kpmcon:not(:has(#c***:target)):has(.kpfin:target)::after {
    content: "Incorrect Passcode";
    color: red;
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

    let config = Config { depth: pwd.len() };
    let html = generate_keypad_html(&config);
    let html = HTML_TEMPLATE.replace("@@@", &html);
    fs::write("res/index.html", html).unwrap();
    let css = generate_keypad_css(&config);
    let css = CSS_TEMPLATE.replace("@@@", &css);
    let css = css.replace("***", &pwd);
    fs::write("res/style.css", css).unwrap();
}
