use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 8: Nested Control Flow
pub fn lesson8() -> impl azumi::Component {
    html! {
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>"Lesson 8: Nested Control Flow - Azumi"</title>
                </head>
                <body>
                    <div>
                        <a href="/">"← Back to Lessons"</a>
                        <h1>"Lesson 8: Nested Control Flow"</h1>

                        <section>
                            <h2>"Working Example"</h2>
                            <div>
                                <h2>"Admin Dashboard"</h2>
                            </div>
                        </section>

                        <section>
                            <h2>"Code"</h2>
                            <pre>{"// Combining @if, @for, @match"}
    {"pub fn dashboard(user: &User) -> impl azumi::Component {"}
    {"    html! {"}
    {"        <div>"}
    {"            @if user.is_admin {"}
    {"                <h2>\"Admin Dashboard\"</h2>"}
    {"                @for widget in user.admin_widgets {"}
    {"                    @match widget {"}
    {"                        Widget::Chart(data) => render_chart(data),"}
    {"                        Widget::Table(data) => render_table(data),"}
    {"                    }"}
    {"                }"}
    {"            } else {"}
    {"                <h2>\"User Dashboard\"</h2>"}
    {"            }"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>
                        </section>

                        <nav>
                            <a href="/lesson-7">"← Previous"</a>
                            <a href="/lesson-9">"Next: Advanced List Processing →"</a>
                        </nav>
                    </div>
                </body>
            </html>
        }
}

pub async fn lesson8_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson8()))
}
