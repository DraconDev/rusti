use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 15: Reusable Form Component
pub fn lesson15() -> impl azumi::Component {
    html! {
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>"Lesson 15: Reusable Form Component - Azumi"</title>
                </head>
                <body>
                    <div>
                        <a href="/">"← Back to Lessons"</a>
                        <h1>"Lesson 15: Reusable Form Component"</h1>

                        <section>
                            <h2>"Working Example"</h2>
                            <div>
                                <label>"Email"</label>
                                <input type="text" />
                            </div>
                        </section>

                        <section>
                            <h2>"Source Code: lesson15.rs"</h2>
                            <pre>{"// Building flexible form components"}
    {"#[derive(Clone)]"}
    {"pub struct FormFieldProps {"}
    {"    pub label: String,"}
    {"    pub value: String,"}
    {"    pub on_change: Box<dyn Fn(String)>,"}
    {"}"}
    {""}
    {"pub fn form_field(props: FormFieldProps) -> impl azumi::Component {"}
    {"    html! {"}
    {"        <div class=\"form-field\">"}
    {"            <label>{&props.label}</label>"}
    {"            <input "}
    {"                type=\"text\" "}
    {"                value={&props.value}"}
    {"                oninput={move |e| props.on_change(e.value())}"}
    {"            />"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>
                        </section>

                        <nav>
                            <a href="/lesson-14">"← Previous"</a>
                            <a href="/lesson-16">"Next: JavaScript Integration →"</a>
                        </nav>
                    </div>
                </body>
            </html>
        }
}

pub async fn lesson15_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson15()))
}
