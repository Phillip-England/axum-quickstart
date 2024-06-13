


pub fn template_base(title: &str, header: &str, nav_menu: &str, content: &str) -> String {
    return format!(/*html*/r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <link rel="stylesheet" href="/static/css/output.css">
            <link rel="stylesheet" href="/static/css/animus.css">
            <script src="https://unpkg.com/htmx.org@1.9.12"></script>
            <script src="/static/js/index.js"></script>
            <title>{} - CFA Suite</title>
        </head>
        <body hx-boost='true'>
            {}{}
            <main class='p-6'>{}</main>
            <script>aktr.mount()</script>
        </body>
        </html>
    "#, title, header, nav_menu, content);
}

pub fn template_guest(title: &str, path: &str, content: &str) -> String {
    return template_base(title, &header(""), "", content);
}

pub fn template_admin(title: &str, path: &str, content: &str) -> String {
    return template_base(title, &admin_header(), &admin_nav_menu(path), content);
}

pub fn header(icons: &str) -> String {
    return format!(/*html*/r#"
        <header class='flex items-center justify-between p-4 border-b'>
            <div class='flex items-center'>
                <img class='w-[150px]' src="/static/img/logo.svg" alt="CFA Suite Logo">
            </div>
            <div class='flex items-center justify-between'>{}</div>
        </header>
    "#, icons);
}

pub fn header_bars() -> String {
    return format!(/*html*/r#"
        <div id='header-bars'>
            <svg class="w-8 h-8" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                <path stroke="currentColor" stroke-linecap="round" stroke-width="2" d="M5 7h14M5 12h14M5 17h14"/>
            </svg>
        </div>
        <script>
            aktr.onMount(async () => {{
                let nav = qs("\#nav")
                let overlay = qs("\#nav-overlay")
                let bars = qs("\#header-bars")
                onClick(bars, (e) => {{
                    nav.classList.remove('fade-out', 'hidden')
                    overlay.classList.remove('fade-out-half', 'hidden')
                    nav.classList.add('fade-in')
                    overlay.classList.add('fade-in-half')
                }})
            }})
        </script>
    "#);
}

pub fn template_nav(nav_items: &str) -> String {
    return format!(/*html*/r#"
        <nav id='nav' class='flex flex-col hidden opacity-0 border-r absolute bg-white top-0 left-0 w-[75%] h-full z-30'>
            <div class='flex items-center p-4 border-b'>
                <img class='w-[150px]' src="/static/img/logo.svg" alt="CFA Suite Logo">
            </div>
            <ul class='flex flex-col gap-1 p-1'>{}</ul>
        </nav>
        <script>
            aktr.onMount(async () => {{
                let nav = qs("\#nav")
                let overlay = qs("\#nav-overlay")
                onClick(overlay, async (e) => {{
                    nav.classList.add('fade-out')
                    overlay.classList.add('fade-out-half')
                    await sleep(200)
                    nav.classList.add('hidden')
                    overlay.classList.add('hidden')
                    nav.classList.remove('fade-in')
                    overlay.classList.remove('fade-in-half')
                }})
            }})
        </script>
        {}
    "#, nav_items, nav_overlay());
}

pub fn admin_nav_menu(path: &str) -> String {
    return format!(/*html*/r#"{}"#,
        template_nav(&format!(r#"{}{}"#,
            nav_item("Admin Panel", "/admin", path),
            nav_item("Logout", "/logout", path),
        )
    ));
}

pub fn nav_item(title: &str, href: &str, path: &str) -> String {
    let active_class = if path == href { "bg-gray-100" } else { "" };
    return format!(/*html*/r#"
        <li class='flex border rounded {}'>
            <a class='p-4 w-full' href="{}">{}</a>
        </li>
    "#, active_class, href, title);
}

pub fn placeholder(id: &str) -> String {
    return format!(/*html*/r#"
        <div id="{}" class="hidden"></div>
    "#, id)
}

pub fn nav_overlay() -> String {
    return format!(/*html*/r#"
        <div id='nav-overlay' class='absolute hidden opacity-0 top-0 bg-black opacity-50 h-full w-full z-20'></div>
    "#);
}

pub fn admin_header() -> String {
    return header(&header_bars())
}

pub fn template_form(title: &str, action: &str, err: &str, content: &str) -> String {
    return format!(/*html*/r#"
            <form class='flex flex-col gap-12' action="{}" method="POST">
                <h2 class='text-xl font-semibold'>{}</h2>
                {}{}
            </form>
        "#,
        action,
        title,
        form_err(err),
        content,
    );
}

pub fn form_err(err: &str) -> String {
    let mut err_html = format!(r#"
            <p class='text-sm text-error'>{}</p>
        "#, 
    err);
    if err == "" {
        err_html = String::from("<p class='invisible'>invisible</p>");
    }
    return err_html;
}

pub fn form_login(err: &str) -> String {
    return template_form("Login", "/", err, &format!(/*html*/r#"{}{}{}"#,
        form_input("text", "Email", "email"),
        form_input("password", "Password", "password"),
        form_submit("Login"),
    ));
}

pub fn form_input(input_type: &str, label: &str, name: &str) -> String {
    return format!(/*html*/r#"
        <div class='flex flex-col gap-1'>
            <label class='text-sm' for="{}">{}</label>
            <input type="{}" name="{}" autocomplete="current-{}" class='px-2 py-1 text-sm border rounded'>
        </div>
    "#, name, label, input_type, name, name);
}

pub fn form_submit(label: &str) -> String {
    return format!(/*html*/r#"
        <button type="submit" class='p-2 border bg-primary text-sm rounded text-white'>{}</button>
    "#, label);
}
