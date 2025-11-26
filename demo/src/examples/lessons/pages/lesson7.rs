use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 7: Local Variables
pub fn lesson7() -> impl azumi::Component {
    html! {
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>"Lesson 7: Local Variables - Azumi"</title>
                </head>
                <body>
                    <div>
                        <a href="/">"← Back to Lessons"</a>
                        <h1>"Lesson 7: Local Variables"</h1>

                        <section>
                            <h2>"Working Example"</h2>
                            <p>"Price: $19.99"</p>
                        </section>

                        <section>
                            <h2>"Code"</h2>
                            <pre>{"// Using @let for computed values"}
    {"pub fn formatted_price(product: &Product) -> impl azumi::Component {"}
    {"    html! {"}
    {"        <div>"}
    {"            @let final_price = product.price * (1.0 - product.discount);"}
    {"            <p>{\"Price: $\" final_price.to_string()}</p>"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>
                        </section>

                        <nav>
                            <a href="/lesson-6">"← Previous"</a>
                            <a href="/lesson-8">"Next: Nested Control Flow →"</a>
                        </nav>
                    </div>
                </body>
            </html>
        }
}

pub async fn lesson7_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson7()))
}
