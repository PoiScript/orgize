use orgize::ast::Headline;

pub fn language_comments(language: &str) -> Option<(&str, &str)> {
    match language {
        "c" | "cpp" | "c++" | "go" | "js" | "javascript" | "ts" | "typescript" | "rust"
        | "vera" | "jsonc" => Some(("//", "")),
        "toml" | "tml" | "yaml" | "yml" | "conf" | "gitconfig" | "conf-toml" | "sh" | "shell"
        | "bash" | "zsh" | "fish" => Some(("#", "")),
        "lua" | "sql" => Some(("--", "")),
        "lisp" | "emacs-lisp" | "elisp" => Some((";;", "")),
        "xml" | "html" | "svg" => Some(("<!--", "-->")),
        _ => None,
    }
}

pub fn language_execute_command(language: &str) -> Option<&str> {
    match language {
        "js" | "javascript" => Some("node"),
        "sh" | "bash" => Some("bash"),
        "py" | "python" => Some("python"),
        "fish" => Some("fish"),
        _ => None,
    }
}

pub fn headline_slug(headline: &Headline) -> String {
    headline.title().fold(String::new(), |mut acc, elem| {
        for ch in elem.to_string().chars() {
            if ch.is_ascii_graphic() {
                acc.push(ch);
            }
        }
        acc
    })
}
