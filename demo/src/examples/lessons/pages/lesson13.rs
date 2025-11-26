use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 13: Component Composition
pub fn lesson13() -> impl azumi::Component {
    html! {
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>"Lesson 13: Component Composition - Azumi"</title>
                </head>
                <body>
                    <div>
                        <a href="/">"← Back to Lessons"</a>
                        <h1>"Lesson 13: Component Composition"</h1>

                        <section>
                            <h2>"Working Example"</h2>
                            <div>
                                <h3>"Profile"</h3>
                                <p>"Name: Alice"</p>
                                <h3>"Recent Activity"</h3>
                                <ul>
                                    <li>"Posted a comment"</li>
                                </ul>
                            </div>
                        </section>

                        <section>
                            <h2>"Code"</h2>
                            <pre>{"pub fn user_profile(user: &User) -> impl azumi::Component {"}
    {"    html! {"}
    {"        <div>"}
    {"            {card(CardProps {"}
    {"                title: \"Profile\".to_string(),"}
    {"                children: html! {"}
    {"                    <div>"}
    {"                        <p>{\"Name: \" user.name}</p>"}
    {"                        <p>{\"Email: \" user.email}</p>"}
    {"                    </div>"}
    {"                }"}
    {"            })}"}
    {"            "}
    {"            {card(CardProps {"}
    {"                title: \"Recent Activity\".to_string(),"}
    {"                children: html! {"}
    {"                    <ul>"}
    {"                        @for activity in &user.activities {"}
    {"                            <li>{&activity.description}</li>"}
    {"                        }"}
    {"                    </ul>"}
    {"                }"}
    {"            })}"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>
                        </section>

                        <nav>
                            <a href="/lesson-12">"← Previous"</a>
                            <a href="/lesson-14">"Next: Component with State →"</a>
                        </nav>
                    </div>
                </body>
            </html>
        }
}

pub async fn lesson13_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson13()))
}
