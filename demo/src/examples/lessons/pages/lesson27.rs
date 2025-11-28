//! Lesson 27: Meta Tags & SEO
//!
//! Essential meta tags for search engines and social media

use azumi::html;

#[azumi::component]
pub fn meta_tags_demo() -> impl azumi::Component {
    let page_title = "Ultimate Guide to Azumi";
    let description = "Learn how to build type-safe web applications with Azumi";
    let og_image = "https://example.com/azumi-guide.jpg";

    html! {
       <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />

                // SEO meta tags
                <title>{page_title}</title>
                <meta name="description" content={description} />
                <meta name="keywords" content="rust, web framework, type-safe, templates" />

                // Open Graph (Facebook, LinkedIn)
                <meta property="og:title" content={page_title} />
                <meta property="og:description" content={description} />
                <meta property="og:image" content={og_image} />
                <meta property="og:type" content="article" />

                // Twitter Card
                <meta name="twitter:card" content="summary_large_image" />
                <meta name="twitter:title" content={page_title} />
                <meta name="twitter:description" content={description} />
                <meta name="twitter:image" content={og_image} />

                <style src="/static/pages/lesson27.css" />
            </head>
            <body>
                <div class="container">
                    <h1>"Lesson 27: Meta Tags & SEO"</h1>
                    <p class="description">
                        "Essential meta tags for search engines and social media"
                    </p>

                    <div class="meta-group">
                        <h2>"Basic SEO"</h2>
                        <div class="meta-example">
                            <code>"<title>" {page_title} "</title>"</code>
                            <code>"<meta name=\"description\" content=\"...\" />"</code>
                        </div>
                    </div>

                    <div class="meta-group">
                        <h2>"Open Graph (Social Sharing)"</h2>
                        <div class="meta-example">
                            <code>"<meta property=\"og:title\" content=\"...\" />"</code>
                            <code>"<meta property=\"og:image\" content=\"...\" />"</code>
                        </div>
                        <p class="help-text">"Used by Facebook, LinkedIn when sharing"</p>
                    </div>

                    <div class="meta-group">
                        <h2>"Twitter Card"</h2>
                        <div class="meta-example">
                            <code>"<meta name=\"twitter:card\" content=\"summary_large_image\" />"</code>
                        </div>
                        <p class="help-text">"Rich previews on Twitter/X"</p>
                    </div>

                    <div class="note">
                        <strong>"All meta tags are type-safe!"</strong>
                        <p>"Build from Rust variables, validated at compile time"</p>
                    </div>
                </div>
            </body>
        </html>
    }
}

pub async fn lesson27_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @meta_tags_demo() }))
}
