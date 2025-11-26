use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 14: Component with State
pub fn lesson14() -> impl azumi::Component {
    html! {
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>"Lesson 14: Component with State - Azumi"</title>
                </head>
                <body>
                    <div>
                        <a href="/">"← Back to Lessons"</a>
                        <h1>"Lesson 14: Component with State"</h1>

                        <section>
                            <h2>"Working Example"</h2>
                            <div>
                                <p>"Count: 3"</p>
                                <button>"Increment"</button>
                            </div>
                        </section>

                        <section>
                            <h2>"Code"</h2>
                            <pre>{"pub fn counter(initial: u32) -> impl azumi::Component {"}
    {"    let mut count = initial;"}
    {"    "}
    {"    html! {"}
    {"        <div>"}
    {"            <p>{\"Count: \" count.to_string()}</p>"}
    {"            <button onclick={move || count += 1}>\"Increment\"</button>"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>
                        </section>

                        <nav>
                            <a href="/lesson-13">"← Previous"</a>
                            <a href="/lesson-15">"Next: Reusable Form Component →"</a>
                        </nav>
                    </div>
                </body>
            </html>
        }
}

pub async fn lesson14_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson14()))
}
