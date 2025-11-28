//! Lesson 25: JSON-LD & Structured Data
//!
//! Embedding JSON for SEO and structured data

use azumi::html;

#[azumi::component]
pub fn json_ld_demo() -> impl azumi::Component {
    // Product data
    let product_json = r#"{
  "@context": "https://schema.org",
  "@type": "Product",
  "name": "Wireless Headphones",
  "description": "Premium noise-cancelling headphones",
  "image": "https://example.com/headphones.jpg",
  "offers": {
    "@type": "Offer",
    "price": "299.99",
    "priceCurrency": "USD"
  }
}"#;

    // Article data
    let article_json = r#"{
  "@context": "https://schema.org",
  "@type": "Article",
  "headline": "Getting Started with Azumi",
  "author": {
    "@type": "Person",
    "name": "Jane Developer"
  },
  "datePublished": "2024-01-15"
}"#;

    html! {
        <style src="/static/pages/lesson25.css" />

        // JSON-LD scripts are allowed
        <script type="application/ld+json">
            {product_json}
        </script>

        <script type="application/ld+json">
            {article_json}
        </script>

        <div class="container">
            <h1>"Lesson 25: JSON-LD & Structured Data"</h1>
            <p class="description">
                "Embed JSON-LD for SEO, social media, and search engines"
            </p>

            <h2>"Product Schema"</h2>
            <div class="schema-example">
                <h3>"Wireless Headphones"</h3>
                <p>"Premium noise-cancelling headphones"</p>
                <div class="price">"$299.99"</div>
                <p class="help-text">
                    "This page includes Product schema for rich search results"
                </p>
            </div>

            <h2>"Article Schema"</h2>
            <div class="schema-example">
                <h3>"Getting Started with Azumi"</h3>
                <p class="meta">"By Jane Developer • Jan 15, 2024"</p>
                <p class="help-text">
                    "This page includes Article schema for better social sharing"
                </p>
            </div>

            <div class="code-example">
                <h3>"How it works:"</h3>
                <pre>
    "let product_json = r#\"{\n"
    "  \"@context\": \"https://schema.org\",\n"
    "  \"@type\": \"Product\",\n"
    "  \"name\": \"Wireless Headphones\"\n"
    "}\"#;\n\n"
    "<script type=\"application/ld+json\">\n"
    "  {product_json}\n"
    "</script>"
                </pre>
            </div>

            <div class="note">
                <strong>"Use cases:"</strong>
                <ul>
                    <li>"Products (ecommerce rich results)"</li>
                    <li>"Articles (social media cards)"</li>
                    <li>"Events (calendar integration)"</li>
                    <li>"Organizations (knowledge panels)"</li>
                    <li>"Breadcrumbs (search navigation)"</li>
                </ul>
            </div>
        </div>
    }
}

pub async fn lesson25_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @json_ld_demo() }))
}
