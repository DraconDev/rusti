//! Lesson 7: local_variables.rs
//!
//! Using @let for computed values
use azumi::html;

#[derive(Clone, Copy)]
struct Product {
    price: f64,
    discount: f64,
}

/// Formatted price with computed discount
pub fn formatted_price(product: Product) -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson7.css" />
        <div>
            @let final_price = product.price * (1.0 - product.discount);
            @let original_text = format!("Original: ${}", product.price);
            @let final_text = format!("Final: ${:.2}", final_price);
            <p>{original_text}</p>
            <p class="discounted-price">{final_text}</p>
        </div>
    }
}

/// Example product with 20% discount
pub fn example_product() -> impl azumi::Component {
    formatted_price(Product {
        price: 100.0,
        discount: 0.2,
    })
}

/// No discount example
pub fn full_price_product() -> impl azumi::Component {
    formatted_price(Product {
        price: 50.0,
        discount: 0.0,
    })
}
