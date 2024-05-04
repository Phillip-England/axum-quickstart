use crate::state;
use std::fs;
use std::sync::Arc;

use axum::{response::{Html, IntoResponse}, Extension};
use askama::Template;
use syntect::{easy::HighlightLines, highlighting::{Style, ThemeSet}, html::{styled_line_to_highlighted_html, IncludeBackground}, parsing::SyntaxSet, util::{as_24_bit_terminal_escaped, LinesWithEndings}};


#[derive(Template)]
#[template(path = "base.html")]
struct BaseTemplate<'a> {
    title:  &'a str,
    content: &'a str,
}

#[derive(Debug)]
struct CodeBlock {
    start: usize,
    end: usize,
    html_content: String,
    inner_content: String,
    language: String,
    language_extension: String,
}

impl CodeBlock {

    fn set_language_extension (&mut self) {
        // values on the left are what markdown places as class names
        // values on the right are the file extensions
        let lang_ext = match self.language.as_str() {
            "rust" => "rs",
            "python" => "py",
            "js" => "js",
            "typescript" => "ts",
            "html" => "html",
            "css" => "css",
            "scss" => "scss",
            "json" => "json",
            "yaml" => "yaml",
            "toml" => "toml",
            "markdown" => "md",
            _ => "txt",
        };
        self.language_extension = lang_ext.to_string();
    }

    fn set_inner_content(&mut self) {
        let start_index = self.html_content.find("<code").unwrap();
        let end_index = self.html_content.find("</code>").unwrap();
        let sub_string = &self.html_content[start_index..end_index];
        let start_index = sub_string.find(">").unwrap();
        let sub_string = &sub_string[start_index+1..];
        self.inner_content = sub_string.to_string();
    }

    fn highlight(&self, syntax_set: &SyntaxSet, theme_set: &ThemeSet) -> String {
        let syntax = syntax_set.find_syntax_by_extension(&self.language).unwrap();
        let mut h = HighlightLines::new(syntax, &theme_set.themes["base16-ocean.dark"]);
        let mut highlighted_html = String::new();
        for line in LinesWithEndings::from(&self.inner_content) {
            let ranges = h.highlight_line(line, &syntax_set).unwrap();
            let html_for_line = styled_line_to_highlighted_html(&ranges, IncludeBackground::Yes).unwrap();
            highlighted_html.push_str(&html_for_line);
        }
        highlighted_html
    }
}

pub async fn home(Extension(shared_state): Extension<Arc<state::AppState>>) -> impl IntoResponse {
    
    // getting markdown content
    let md_str = fs::read_to_string("./content/home.md").unwrap();
    let mut md_content = markdown::to_html(&md_str);

    // collecting pre start and end indexes
    let mut pre_start_indexes = Vec::new();
    let mut pre_end_indexes = Vec::new();
    for (index, _) in md_content.char_indices() {
        let chunk_length = 4;
        if index + chunk_length > md_content.len() {
            continue;
        }
        let chunck = &md_content[index..index+chunk_length];
        if chunck == "<pre" {
            pre_start_indexes.push(index);
        }
        if chunck == "</pr" {
            pre_end_indexes.push(index+chunk_length+2);
        }
    }

    // highlighting code blocks
    for (index, start_index) in pre_start_indexes.iter().enumerate() {
        let end_index = pre_end_indexes[index];
        let pre_content = &md_content[*start_index..end_index];
        if pre_content.contains("language-") {
            let lang_start_index = pre_content.find("language-").unwrap();
            let lang_split = &pre_content[lang_start_index..];
            let lang_split_end = lang_split.find("\"").unwrap();
            let lang_class = &lang_split[..lang_split_end];
            let lang_class = lang_class.replace("language-", "");
            let mut code_block = CodeBlock {
                start: *start_index,
                end: end_index,
                html_content: pre_content.to_string(),
                language: lang_class,
                language_extension: "".to_string(),
                inner_content: "".to_string(),
            };
            code_block.set_language_extension();
            code_block.set_inner_content();
    
            let highlighted_code_html = code_block.highlight(&shared_state.syntax_set, &shared_state.theme_set);
            md_content = md_content.replace(&code_block.inner_content, &highlighted_code_html);
        }
    }


    // let syntax = shared_state.syntax_set.find_syntax_by_extension("md").unwrap();
    // let mut h = HighlightLines::new(syntax, &shared_state.theme_set.themes["base16-ocean.dark"]);

    // let mut highlighted_html = String::new();
    // for line in LinesWithEndings::from(&md_content) {
    //     let ranges = h.highlight_line(line, &shared_state.syntax_set).unwrap();
    //     let html_for_line = styled_line_to_highlighted_html(&ranges, IncludeBackground::Yes).unwrap();
    //     highlighted_html.push_str(&html_for_line);
    // }

    let template = BaseTemplate {
        title: "Home",
        content: &md_content,
    };
    Html(template.render().unwrap())
}

pub async fn not_found() -> impl IntoResponse {
    "Not Found!"
}
