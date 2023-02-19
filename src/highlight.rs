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

pub fn code_highlight(code: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut highlighter = Highlighter::new();

    let mut python_config = HighlightConfiguration::new(
        tree_sitter_python::language(),
        tree_sitter_python::HIGHLIGHT_QUERY,
        "",
        "",
    )?;

    python_config.configure(HIGHLIGHT_NAMES);
    let html_attrs: Vec<String> = HIGHLIGHT_NAMES
        .iter()
        .map(|s| format!("class=\"{}\"", s.replace('.', " ")))
        .collect();

    let highlights = highlighter.highlight(&python_config, code.as_bytes(), None, |_| None)?;

    let mut renderer = HtmlRenderer::new();
    renderer.render(highlights, code.as_bytes(), &|highlight| {
        html_attrs[highlight.0].as_bytes()
    })?;

    Ok(String::from_utf8_lossy(renderer.html.as_slice()).to_string())
}
