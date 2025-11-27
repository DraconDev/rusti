use azumi::html;



#[derive(Debug, Clone)]
pub struct Product {
    pub name: String,
    pub price: f64,
    pub discount: f64, // 0.0 to 1.0
}

pub fn lesson7() -> impl azumi::Component {
    let products = vec![
        Product {
            name: "Laptop".to_string(),
            price: 999.00,
            discount: 0.10,
        },
        Product {
            name: "Smartphone".to_string(),
            price: 699.00,
            discount: 0.0,
        },
        Product {
            name: "Headphones".to_string(),
            price: 199.00,
            discount: 0.25,
        },
    ];

    html! {
        <style src="/static/pages/lesson7.css" />
        <div class="lesson-container">
            <h1 class="lesson-title">"Lesson 7: Local Variables"</h1>
            <p class="lesson-description">
                "Use " <code>"@let"</code> " to define local variables within your template. This is useful for calculating values derived from props or state without cluttering your Rust code."
            </p>

            <div class="demo-section">
                <h2>"Product Pricing"</h2>
                <div class="product-grid">
                    @for product in &products {
                        <div class="product-card">
                            <h3>{&product.name}</h3>

                            // Calculate final price using @let
                            @let final_price = product.price * (1.0 - product.discount);
                            @let has_discount = product.discount > 0.0;

                            <div class="price-container">
                                @if has_discount {
                                    <span class="original-price">{"$" product.price.to_string()}</span>
                                    <span class="discount-badge">{format!("-{}%", (product.discount * 100.0) as u32)}</span>
                                }
                                <div class="final-price">
                                    {"$" format!("{:.2}", final_price)}
                                </div>
                            </div>
                        </div>
                    }
                </div>
            </div>

            <div class="code-preview">
                <h3>"Source Code"</h3>
                <pre><code>
"// Using @let for calculations
@let final_price = product.price * (1.0 - product.discount);
@let has_discount = product.discount > 0.0;

@if has_discount {
    <span class=\"original-price\">{product.price}</span>
}
<div class=\"final-price\">{final_price}</div>"
                </code></pre>
            </div>
        </div>
    }
}
