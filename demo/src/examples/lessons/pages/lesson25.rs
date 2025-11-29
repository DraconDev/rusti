//! Lesson 25: Schema.org JSON-LD
//!
//! Using #[derive(Schema)] to generate structured data

use azumi::html;
use serde::Serialize;
use serde_json;

// Derive Schema for automatic JSON-LD generation!
#[derive(Serialize, azumi::Schema)]
#[schema(type = "Product")]
struct ProductData {
    name: String,
    description: String,
    image: String,
    #[schema(name = "offers")]
    price_info: PriceOffer,
}

#[derive(Serialize, azumi::Schema)]
#[schema(type = "Offer")]
struct PriceOffer {
    price: String,
    #[schema(name = "priceCurrency")]
    currency: String,
}

#[azumi::component]
pub fn schema_demo() -> impl azumi::Component {
    let product = ProductData {
        name: "Wireless Headphones".to_string(),
        description: "Premium noise-cancelling headphones".to_string(),
        image: "https://example.com/headphones.jpg".to_string(),
        price_info: PriceOffer {
            price: "299.99".to_string(),
            currency: "USD".to_string(),
        },
    };

    html! {
        <style src="/static/pages/lesson25.css" />

        // Automatically inject JSON-LD schema!
        // Automatically inject JSON-LD schema!
        <script type="application/ld+json">
            {serde_json::to_string_pretty(&product.to_schema_json_value()).unwrap()}
        </script>

        <div class="container">
            <h1>"Lesson 25: Schema.org JSON-LD"</h1>

            <div class="product-card">
                <img src={&product.image} alt={&product.name} class="product-image" />
                <h2 class="product-name">{&product.name}</h2>
                <p class="product-description">{&product.description}</p>
                <div class="product-price">
                    "$" {&product.price_info.price}
                </div>
            </div>

            <div class="explanation">
                <h2>"How it works:"</h2>
                <ol>
                    <li>"Define struct with " <code>"#[derive(Schema)]"</code></li>
                    <li>"Call " <code>".to_schema_script()"</code> " in template"</li>
                    <li>"Automatic JSON-LD script tag!"</li>
                </ol>

                <div class="code-box">
                    <pre>
    "#[derive(Serialize, azumi::Schema)]\n"
    "#[schema(type = \"Product\")]\n"
    "struct ProductData {\n"
    "    name: String,\n"
    "    // Use #[schema(name = \"...\")] for camelCase\n"
    "    #[schema(name = \"offers\")]\n"
    "    price_info: PriceOffer,\n"
    "}\n\n"
    "// In template:\n"
    "{@product.to_schema_script()}"
                    </pre>
                </div>
            </div>
        </div>
    }
}

pub async fn lesson25_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @schema_demo() }))
}
