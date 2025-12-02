use azumi::html;

/// Test component to verify inline styles work
#[azumi::component]
pub fn test_inline_styles() -> impl azumi::Component {
    html! {
        <div>
            <h1>"Testing Inline Styles"</h1>

            {/* Test 1: Inline style attribute */}
            <div style="color: red; padding: 10px; background-color: #f0f0f0;">
                "This should have red text with padding and light gray background"
            </div>

            {/* Test 2: Inline style tag */}
            <style>
                .test_class {
                    font-weight: "bold";
                    color: "blue";
                }
            </style>

            <p class="test_class">"This should be bold and blue"</p>

            {/* Test 3: Dynamic inline style */}
            <div style="background-color: yellow; padding: 5px;">
                "Dynamic yellow background"
            </div>
        </div>
    }
}

pub async fn test_inline_styles_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&test_inline_styles()))
}
