use askama::Template;


#[derive(Template)]
#[template(path = "shell/base.html")]
pub struct Base<'a> {
    pub title:  &'a str,
    pub show_bars: bool
}

#[derive(Template)]
#[template(path = "view/login.html")]
pub struct Login<'a> {
    pub title:  &'a str,
    pub login_err: &'a str,
    pub show_bars: bool
}

#[derive(Template)]
#[template(path = "view/admin_panel.html")]
pub struct AdminPanel<'a> {
    pub title:  &'a str,
    pub show_bars: bool
}