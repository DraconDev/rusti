//! Lesson 28: Error Handling
//!
//! Custom error pages for 404 and 500 errors

use azumi::html;

#[azumi::component]
pub fn error_page<'a>(
    code: u16,
    title: &'a str,
    message: &'a str
) -> impl azumi::Component + 'a {
    html! {
        <style src="/static/pages/lesson28.css" />
        <div class="error-container">
            <div class="error-content">
                <h1 class="error-code">{code}</h1>
                <h2 class="error-title">{title}</h2>
                <p class="error-message">{message}</p>
                <a href="/" class="home-button">"Return Home"</a>
            </div>
        </div>
    }
}

#[azumi::component]
pub fn not_found_page() -> impl azumi::Component {
    @error_page(
        code=404,
        title="Page Not Found",
        message="The page you are looking for does not exist or has been moved."
    )
}

#[azumi::component]
pub fn server_error_page() -> impl azumi::Component {
    @error_page(
        code=500,
        title="Server Error",
        message="Something went wrong on our end. Please try again later."
    )
}

#[azumi::component]
pub fn error_demo() -> impl azumi::Component {
    html! {
        <div class="demo-wrapper">
            <h1>"Lesson 28: Error Pages"</h1>
            <p>"Reusable error page component for consistent error handling"</p>
            
            <div class="preview-box">
                <h3>"404 Preview:"</h3>
                @not_found_page()
            </div>

            <div class="preview-box">
                <h3>"500 Preview:"</h3>
                @server_error_page()
            </div>
        </div>
    }
}

pub async fn lesson28_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @error_demo() }))
}
