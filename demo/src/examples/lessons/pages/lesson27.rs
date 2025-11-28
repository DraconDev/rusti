//! Lesson 27: SEO Meta Tags
//!
//! Building complete HTML documents with meta tags

use azumi::html;

#[azumi::component]
pub fn seo_page_demo<'a>(
    title: &'a str,
    description: &'a str,
    image_url: &'a str
) -> impl azumi::Component + 'a {
    html! {
        <html lang="en">
            <head>
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
            </head>
            <body>
                <div class="page-container">
                    <h1>"Lesson 27: SEO Meta Tags"</h1>
                    
                    <div class="preview-card">
                        <img src={image_url} alt="Preview" class="preview-image" />
                        <h2>{title}</h2>
                        <p>{description}</p>
                    </div>

                    <div class="code-sample">
                        <h3>"Generated meta tags:"</h3>
                        <pre>
"<title>" {title} "</title>\n"
"<meta property=\"og:title\" content=\"" {title} "\" />\n"
"<meta property=\"og:image\" content=\"" {image_url} "\" />"
                        </pre>
                    </div>
                </div>
            </body>
        </html>
    }
}

#[azumi::component]
pub fn meta_demo() -> impl azumi::Component {
    @seo_page_demo(
        title="Ultimate Rust Guide",
        description="Learn Rust with type-safe templates",
        image_url="https://example.com/rust-guide.jpg"
    )
}

pub async fn lesson27_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @meta_demo() }))
}
