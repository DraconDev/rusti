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
            {azumi::head! {
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />

                <title>{title}</title>
                <meta name="description" content={description} />

                // Open Graph tags
                <meta property="og:title" content={title} />
                <meta property="og:description" content={description} />
                <meta property="og:image" content={image_url} />
                <meta property="og:type" content="article" />

                // Twitter Card
                <meta name="twitter:card" content="summary_large_image" />
                <meta name="twitter:title" content={title} />
                <meta name="twitter:image" content={image_url} />

                <style src="/static/pages/lesson27.css" />
            }}

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
    "      <title>{{title}}</title>\n"
    "      <meta property=\"og:title\" content={{title}} />\n"
    "      <style src=\"/static/style.css\" />\n"
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
