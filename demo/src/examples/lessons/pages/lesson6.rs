use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 6: Pattern Matching
pub fn lesson6() -> impl azumi::Component {
    html! {
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>"Lesson 6: Pattern Matching - Azumi"</title>
                </head>
                <body>
                    <div>
                        <a href="/">"← Back to Lessons"</a>
                        <h1>"Lesson 6: Pattern Matching"</h1>

                        <section>
                            <h2>"Working Example"</h2>
                            <div>
                                <span>"Admin"</span>
                            </div>
                        </section>

                        <section>
                            <h2>"Code"</h2>
                            <pre>{"// Using @match for complex conditions"}
    {"pub fn user_role_display(user: &User) -> impl azumi::Component {"}
    {"    html! {"}
    {"        <div>"}
    {"            @match user.role {"}
    {"                Role::Admin => <span>\"Admin\"</span>,"}
    {"                Role::User => <span>\"User\"</span>,"}
    {"                Role::Guest => <span>\"Guest\"</span>,"}
    {"            }"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>
                        </section>

                        <nav>
                            <a href="/lesson-5">"← Previous"</a>
                            <a href="/lesson-7">"Next: Local Variables →"</a>
                        </nav>
                    </div>
                </body>
            </html>
        }
}

pub async fn lesson6_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson6()))
}
