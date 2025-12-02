use azumi::html;

/// Lesson 15: E-commerce Product Catalog
///
/// Advanced e-commerce patterns
mod ecommerce {
    #[derive(Debug)]
    struct Product {
        id: i32,
        name: String,
        price: f64,
        description: String,
        in_stock: bool,
    }

    #[azumi::component]
    pub fn product_card(product: Product) -> impl azumi::Component {
        html! {
            <style>
                .product_card { border: "1px solid #eee"; padding: "1rem"; display: "grid"; gap: "0.5rem"; }
                .product_name { font-weight: "bold"; }
                .product_price { color: "#2196f3"; font-weight: "bold"; }
                .product_description { color: "#666"; font-size: "0.9rem"; }
                .add_button { padding: "0.5rem"; background: "#4caf50"; color: "white"; border: "none"; cursor: "pointer"; }
                .out_of_stock { opacity: "0.5"; }
            </style>
            <div class={if product.in_stock { "product_card" } else { "product_card out_of_stock" }}>
                <h3 class={product_name}>{product.name}</h3>
                <div class={product_price}>${product.price}</div>
                <p class={product_description}>{product.description}</p>
                @if product.in_stock {
                    <button class={add_button}>"Add to Cart"</button>
                }
                @if !product.in_stock {
                    <p>"Out of Stock"</p>
                }
            </div>
        }
    }

    #[azumi::component]
    pub fn product_grid() -> impl azumi::Component {
        let products = vec![
            Product {
                id: 1,
                name: "Premium Headphones",
                price: 199.99,
                description: "High-quality noise-cancelling headphones with 30-hour battery life",
                in_stock: true,
            },
            Product {
                id: 2,
                name: "Wireless Mouse",
                price: 49.99,
                description: "Ergonomic wireless mouse with 5 programmable buttons",
                in_stock: true,
            },
            Product {
                id: 3,
                name: "Mechanical Keyboard",
                price: 129.99,
                description: "RGB backlit mechanical keyboard with Cherry MX switches",
                in_stock: false,
            },
            Product {
                id: 4,
                name: "4K Monitor",
                price: 399.99,
                description: "27-inch 4K UHD monitor with HDR and 144Hz refresh rate",
                in_stock: true,
            },
        ];

        html! {
            <style>
                .grid { display: "grid"; grid-template-columns: "repeat(auto-fill, minmax(250px, 1fr))"; gap: "1rem"; }
            </style>
            <div class={grid}>
                @for product in &products {
                    @product_card(product=product)
                }
            </div>
        }
    }
}

/// Main lesson demonstration component
#[azumi::component]
pub fn lesson15() -> impl azumi::Component {
    html! {
        <style>
            .container { padding: "20px"; }
            .header { text-align: "center"; margin-bottom: "30px"; }
            .main_title { font-size: "32px"; color: "#333"; }
            .subtitle { font-size: "18px"; color: "#666"; }
            .key_points { background: "#f9f9f9"; padding: "20px"; border-radius: "8px"; margin-bottom: "30px"; }
            .section_title { font-size: "20px"; margin-bottom: "15px"; }
            .points_list { list-style: "none"; padding: "0"; }
            .point { margin-bottom: "10px"; }
            .examples { display: "grid"; gap: "20px"; }
            .example_card { border: "1px solid #ddd"; padding: "20px"; border-radius: "8px"; }
        </style>
        <div class={container}>
            <header class={header}>
                <h1 class={main_title}>"Lesson 15: E-commerce Product Catalog"</h1>
                <p class={subtitle}>"Advanced e-commerce patterns"</p>
            </header>

            <section class={key_points}>
                <h2 class={section_title}>"Key Concepts"</h2>
                <ul class={points_list}>
                    <li class={point}>"✅ Product data modeling"</li>
                    <li class={point}>"✅ Responsive product grid"</li>
                    <li class={point}>"✅ Inventory management"</li>
                    <li class={point}>"✅ Conditional rendering"</li>
                    <li class={point}>"✅ E-commerce component patterns"</li>
                </ul>
            </section>

            <section class={examples}>
                <div class={example_card}>
                    @ecommerce::product_grid()
                </div>
            </section>
        </div>
    }
}

// Handler for Axum
pub async fn lesson15_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson15()))
}