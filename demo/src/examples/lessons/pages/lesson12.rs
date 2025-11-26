use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 12: Component with Children
pub fn lesson12() -> impl azumi::Component {
    html! {
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>"Lesson 12: Component with Children - Azumi"</title>
                </head>
                <body>
                    <div>
                        <a href="/">"← Back to Lessons"</a>
                        <h1>"Lesson 12: Component with Children"</h1>

                        <section>
                            <h2>"Working Example"</h2>
                            <div>
                                <h3>"Profile"</h3>
                                <p>"Name: Alice"</p>
                            </div>
                        </section>

                        <section>
                            <h2>"Code"</h2>
                            <pre>{"#[derive(Clone)]"}
    {"pub struct CardProps {"}
    {"    pub title: String,"}
    {"    pub children: impl azumi::Component,"}
    {"}"}
    {""}
    {"pub fn card(props: CardProps) -> impl azumi::Component {"}
    {"    html! {"}
    {"        <div class=\"card\">"}
    {"            <h3>{&props.title}</h3>"}
    {"            {props.children}"}
    {"        </div>"}
    {"    }"}
    {"}"}
    {""}
    {"pub fn user_profile(user: &User) -> impl azumi::Component {"}
    {"    html! {"}
    {"        <div>"}
    {"            {card(CardProps {"}
    {"                title: \"Profile\".to_string(),"}
    {"                children: html! {"}
    {"                    <div>"}
    {"                        <p>{\"Name: \" user.name}</p>"}
    {"                    </div>"}
    {"                }"}
    {"            })}"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>
                        </section>

                        <nav>
                            <a href="/lesson-11">"← Previous"</a>
                            <a href="/lesson-13">"Next: Component Composition →"</a>
                        </nav>
                    </div>
                </body>
            </html>
        }
}

pub async fn lesson12_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson12()))
}
