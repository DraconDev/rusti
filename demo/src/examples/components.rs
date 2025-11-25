use axum::response::{Html, IntoResponse};
use azumi::html;
use azumi::Component;

/// Component composition example
pub fn components_demo() -> impl Component {
    let title = "Components Demo";
    let items = vec!["Type-safe", "Zero-cost", "Composable"];

    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <title>{title}</title>
                <style src="demo/static/components.css" />
            </head>
            <body>
                <div class="page">
                    @header(title)

                    <main class="content">
                        <h2>"Feature Cards"</h2>
                        <div class="cards">
                            @for item in &items {
                                @card(item, "✨")
                            }
                        </div>

                        <h2>"Dynamic Content"</h2>
                        <div class="dynamic">
                            <p>"Timestamp: " {std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs()
                                .to_string()}</p>
                        </div>
                    </main>

                    @footer()
                </div>
            </body>
        </html>
    }
}

fn header<'a>(title: &'a str) -> impl azumi::Component + 'a {
    html! {
        <header class="header">
            <h1>{title}</h1>
            <p>"Demonstrating component composition"</p>
        </header>
    }
}

fn card<'a>(text: &'a str, icon: &'a str) -> impl azumi::Component + 'a {
    html! {
        <div class="card">
            <span class="icon">{icon}</span>
            <span class="text">{text}</span>
        </div>
    }
}

fn footer() -> impl azumi::Component {
    html! {
        <footer class="footer">
            <p>"Built with azumi
    2.0"</p>
        </footer>
    }
}

pub async fn components_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&components_demo()))
}
