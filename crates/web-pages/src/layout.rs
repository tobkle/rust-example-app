use dioxus::prelude::*;
use web_assets::files::favicon_svg;

#[component]
pub fn Layout(title: String, children: Element) -> Element {
    // For SSR, we use Fragment and handle HTML structure in the render function
    rsx! {
        div { class: "layout",
            h1 { "{title}" }
            main {
                {children}
            }
        }
    }
}

pub fn render_with_html_wrapper(title: &str, content: &str) -> String {
    format!(
r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <link rel="icon" type="image/svg+xml" href="{}">
</head>
<body>
    {}
</body>
</html>"#,
        title, favicon_svg.name, content
    )
}