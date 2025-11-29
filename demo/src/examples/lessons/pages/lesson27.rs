//! Lesson 27: SEO with head! macro
//!
//! Building complete HTML documents with the head! macro

use azumi::html;

#[azumi::component]
pub fn seo_demo<'a>(
    title: &'a str,
    description: &'a str,
    image_url: &'a str,
) -> impl azumi::Component + 'a {
    html! {
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />

                // Use the head! macro for SEO tags
                {azumi::head! {
                    title: title,
                    description: description,
                    image: image_url
                }}

                <style src="/static/pages/lesson27.css" />
            </head>

            <body>
                <style src="/static/pages/lesson27.css" />
                <div class="page">
                    <h1>"Lesson 27: head! Macro"</h1>
                    <p class="intro">"Use " <code>"head!"</code> " for all meta tags and stylesheets"</p>

                    <div class="preview">
                        <img src={image_url} alt="Share preview" class="preview-img" />
                        <h2>{title}</h2>
                        <p>{description}</p>
                    </div>

                    <div class="example">
                        <h3>"Code:"</h3>
                        <pre>
    "use azumi::{{html, head}};\n\n"
    "html! {\n"
    "  <html>\n"
    "    {{head! {{\n"
    "      title: {{title}},\n"
    "      description: {{description}},\n"
    "      image: {{image_url}}\n"
    "    }}}}\n"
    "    <body>...</body>\n"
    "  </html>\n"
    "}"
                        </pre>
                    </div>
                </div>
            </body>
        </html>
    }
}

pub async fn lesson27_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! {
        @seo_demo(
            title="Azumi Guide",
            description="Type-safe templates for Rust",
            image_url="https://example.com/azumi.jpg"
        )
    }))
}
