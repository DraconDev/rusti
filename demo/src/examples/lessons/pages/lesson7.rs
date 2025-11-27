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
            <p>{"Original: $" product.price.to_string()}</p>
            <p class="discounted-price">{"Final: $" final_price.to_string()}</p>
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
pub async fn lesson7_handler() -> impl axum::response::IntoResponse { axum::response::Html("Lesson 7") }
