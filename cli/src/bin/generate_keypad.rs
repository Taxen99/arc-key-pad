use std::{env, fs};

use keypad::{
    ao3_css_transform::ao3_css_transform,
    keypad::{Config, Passcode, Themes, generate_keypad_css, generate_keypad_html},
};

const HTML_TEMPLATE: &str = r##"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><title>Document</title><link rel="stylesheet" href="style.css"></head><body>
@@@
</body></html>"##;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let passcode = args.get(1).expect("please provide passcode");
    let passcode = Passcode::new(&passcode).unwrap();
    if passcode.get().len() > 3 {
        println!("\x1b[0;33m[WARNING] a four digit passcode will not fit on ao3\x1b[0m");
    }
    let theme = args.get(2).map(|x| x.as_str()).unwrap_or("default");

    let themes = fs::read_to_string("themes.toml").expect("provide themes.toml");
    let mut themes: Themes = toml::from_str(&themes).expect("invalid themes format");
    let theme = themes
        .0
        .remove(theme)
        .expect(&format!("'{}' is not a valid theme", theme));

    let config = Config { passcode, theme };
    fs::create_dir_all("output").expect("could not create output directory");
    let html = generate_keypad_html(&config);
    let full_html = HTML_TEMPLATE.replace("@@@", &html);
    let css = generate_keypad_css(&config);
    let ao3_css = ao3_css_transform(&css);
    fs::write("output/index.html", full_html).expect("could not write file");
    fs::write("output/style.css", css).expect("could not write file");
    fs::write("output/ao3_ready.html", html).expect("could not write file");
    fs::write("output/ao3_ready.css", ao3_css).expect("could not write file");
}
