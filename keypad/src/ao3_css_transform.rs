use regex::Regex;

enum State {
    Outside,
    ParsingSelector(String),
    ParsingDeclaration(String),
}

struct Decl {
    sel: String,
    decl: String,
}

// TODO: buggy. fix
pub fn ao3_css_transform(css: &str) -> String {
    let comment_regex = Regex::new(r"(?s)/\*.*?\*/").unwrap();
    let css = comment_regex.replace_all(&css, "");
    let css = css.replace(";", "\n");
    let lines = css.lines();
    let mut state = State::Outside;
    let mut decls = Vec::new();
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match state {
            s @ State::Outside | s @ State::ParsingSelector(_) => {
                let mut sel = if let State::ParsingSelector(sel) = s {
                    sel
                } else {
                    "".into()
                };
                let more_sel = line.trim_end_matches('{');
                if !more_sel.is_empty() {
                    sel.push_str(more_sel);
                }
                if line.ends_with('{') {
                    state = State::ParsingDeclaration(more_sel.to_owned());
                } else {
                    state = State::ParsingSelector(sel);
                };
            }
            State::ParsingDeclaration(ref sel) => {
                // let decl = line.trim_end_matches('}');
                if line == "}" {
                    state = State::Outside;
                } else {
                    let decl = line;
                    let decl = Decl {
                        sel: sel.trim().into(),
                        decl: decl.trim().into(),
                    };
                    decls.push(decl);
                }
            }
        }
    }
    let mut output = String::new();
    for decl in decls {
        output.push_str(&format!("{} {{\n    {}\n}}\n", decl.sel, decl.decl));
    }
    output = output.trim().to_owned();
    output
}
