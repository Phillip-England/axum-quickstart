
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
pub struct AppState {
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,
}

pub fn new_app_state() -> AppState {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    AppState {
        syntax_set: ps,
        theme_set: ts,
    }
}