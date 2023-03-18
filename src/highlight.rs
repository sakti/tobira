use lazy_static::lazy_static;
use strum_macros::EnumString;
use tree_sitter_highlight::{HighlightConfiguration, Highlighter, HtmlRenderer};

const HIGHLIGHT_NAMES: &[&str; 18] = &[
    "attribute",
    "constant",
    "function.builtin",
    "function",
    "keyword",
    "operator",
    "property",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "string",
    "string.special",
    "tag",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.parameter",
];

#[derive(Debug, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Language {
    C,
    Go,
    Python,
    Rust,
    Typescript,
}

lazy_static! {
    static ref C_CONFIG: HighlightConfiguration = {
        let mut c_config = HighlightConfiguration::new(
            tree_sitter_c::language(),
            tree_sitter_c::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .unwrap();
        c_config.configure(HIGHLIGHT_NAMES);

        c_config
    };
    static ref GO_CONFIG: HighlightConfiguration = {
        let mut go_config = HighlightConfiguration::new(
            tree_sitter_go::language(),
            tree_sitter_go::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .unwrap();
        go_config.configure(HIGHLIGHT_NAMES);

        go_config
    };
    static ref PYTHON_CONFIG: HighlightConfiguration = {
        let mut python_config = HighlightConfiguration::new(
            tree_sitter_python::language(),
            tree_sitter_python::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .unwrap();
        python_config.configure(HIGHLIGHT_NAMES);

        python_config
    };
    static ref RUST_CONFIG: HighlightConfiguration = {
        let mut rust_config = HighlightConfiguration::new(
            tree_sitter_rust::language(),
            tree_sitter_rust::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .unwrap();
        rust_config.configure(HIGHLIGHT_NAMES);
        rust_config
    };
    static ref TYPESCRIPT_CONFIG: HighlightConfiguration = {
        let mut typescript_config = HighlightConfiguration::new(
            tree_sitter_typescript::language_typescript(),
            tree_sitter_typescript::HIGHLIGHT_QUERY,
            "",
            tree_sitter_typescript::LOCALS_QUERY,
        )
        .unwrap();
        typescript_config.configure(HIGHLIGHT_NAMES);
        typescript_config
    };
}

pub fn code_highlight(lang: Language, code: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut highlighter = Highlighter::new();

    let config: &HighlightConfiguration = match lang {
        Language::C => &C_CONFIG,
        Language::Go => &GO_CONFIG,
        Language::Python => &PYTHON_CONFIG,
        Language::Rust => &RUST_CONFIG,
        Language::Typescript => &TYPESCRIPT_CONFIG,
    };

    let html_attrs: Vec<String> = HIGHLIGHT_NAMES
        .iter()
        .map(|s| format!("class=\"{}\"", s.replace('.', " ")))
        .collect();

    let highlights = highlighter.highlight(config, code.as_bytes(), None, |_| None)?;

    let mut renderer = HtmlRenderer::new();
    renderer.render(highlights, code.as_bytes(), &|highlight| {
        html_attrs[highlight.0].as_bytes()
    })?;

    Ok(String::from_utf8_lossy(renderer.html.as_slice()).to_string())
}
