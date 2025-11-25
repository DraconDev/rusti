use axum::response::{Html, IntoResponse};
use rusti::rusti;

/// Component composition example
pub fn components_demo() -> impl rusti::Component {
    let title = "Components Demo";
    let items = vec!["Type-safe", "Zero-cost", "Composable"];

    rusti! {
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

fn header(title: &str) -> impl rusti::Component {
    rusti! {
        <header class="header">
            <h1>{title}</h1>
            <p>"Demonstrating component composition"</p>
        </header>
    }
}

fn card(text: &str, icon: &str) -> impl rusti::Component {
    rusti! {
        <div class="card">
            <span class="icon">{icon}</span>
            <span class="text">{text}</span>
        </div>
    }
}

fn footer() -> impl rusti::Component {
    rusti! {
        <footer class="footer">
            <p>"Built with Rusti 2.0"</p>
        </footer>
    }
}

pub async fn components_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&components_demo()))
}
