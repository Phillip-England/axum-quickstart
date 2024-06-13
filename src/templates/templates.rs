use askama::Template;


#[derive(Template)]
#[template(path = "template/base.html")]
pub struct Base<'a> {
    pub title:  &'a str,
    pub show_bars: bool
}

#[derive(Template)]
#[template(path = "page/login.html")]
pub struct Login<'a> {
    pub title:  &'a str,
    pub login_err: &'a str,
    pub show_bars: bool,
}

#[derive(Template)]
#[template(path = "page/admin_panel.html")]
pub struct AdminPanel<'a> {
    pub title:  &'a str,
    pub show_bars: bool,
	pub path: &'a str,
}