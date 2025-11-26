use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 11: Simple Component
pub fn lesson11() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Lesson 11: Simple Component - Azumi"</title>
            </head>
            <body>
                <div>
                    <a href="/">"← Back to Lessons"</a>
                    <h1>"Lesson 11: Simple Component"</h1>
                    
                    <section>
                        <h2>"Working Example"</h2>
                        <button>"Click Me"</button>
                    </section>

                    <section>
                        <h2>"Code"</h2>
                        <pre>{"// Creating reusable components"}
{"#[derive(Clone)]"}
{"pub struct ButtonProps {"}
{"    pub text: String,"}
{"    pub variant: ButtonVariant,"}
{"}"}
{""}
{"pub fn button(props: ButtonProps) -> impl azumi::Component {"}
{"    html! {"}
{"        <button class={format!(\"btn btn-{}\", match props.variant {"}
{"            ButtonVariant::Primary => \"primary\","}
{"            ButtonVariant::Secondary => \"secondary\","}
{"        })}>"
{"            {&props.text}"}
{"        </button>"}
{"    }"}
{"}"}</pre>
                    </section>

                    <nav>
                        <a href="/lesson-10">"← Previous"</a>
                        <a href="/lesson-12">"Next: Component with Children →"</a>
                    </nav>
                </div>
            </body>
        </html>
    }
}

pub async fn lesson11_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson11()))
}