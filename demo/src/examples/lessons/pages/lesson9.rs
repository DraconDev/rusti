use azumi::html;

/// Lesson 9: Feature Composition
///
/// Combining multiple Azumi features
#[azumi::component]
pub fn feature_showcase() -> impl azumi::Component {
    html! {
        <style>
            .showcase { display: "grid"; gap: "2rem"; }
            .section { padding: "1rem"; border: "1px solid #eee"; }
            .feature_list { display: "grid"; gap: "0.5rem"; }
            .feature_item { padding: "0.5rem"; background: "#f0f0f0"; }
            .active_badge { background: "#4caf50"; color: "white"; padding: "0.25rem 0.5rem"; border-radius: "4px"; }
        </style>
        <div class={showcase}>
            <div class={section}>
                <h2>"Feature Composition"</h2>
                <span class={active_badge}>"ACTIVE"</span>
                <div class={feature_list}>
                    <div class={feature_item}>"Feature 1"</div>
                    <div class={feature_item}>"Feature 2"</div>
                    <div class={feature_item}>"Feature 3"</div>
                </div>
            </div>
        </div>
    }
}

/// Example: Complex feature integration
#[azumi::component]
pub fn complex_feature_integration() -> impl azumi::Component {
    html! {
        <style>
            .integration_container { padding: "1.5rem"; background: "#f9f9f9"; }
            .feature_grid { display: "grid"; grid-template-columns: "repeat(auto-fit, minmax(200px, 1fr))"; gap: "1rem"; }
            .feature_card { padding: "1rem"; background: "white"; border: "1px solid #eee"; }
            .feature_name { font-weight: "bold"; color: "#2196f3"; }
            .feature_desc { font-size: "0.9rem"; color: "#666"; }
        </style>
        <div class={integration_container}>
            <h3>"Complex Feature Integration"</h3>

            <div class={feature_grid}>
                <div class={feature_card}>
                    <div class={feature_name}>"CSS Scoping"</div>
                    <div class={feature_desc}>"Automatic component scoping"</div>
                </div>
                <div class={feature_card}>
                    <div class={feature_name}>"Type Safety"</div>
                    <div class={feature_desc}>"Compile-time type checking"</div>
                </div>
                <div class={feature_card}>
                    <div class={feature_name}>"Validation"</div>
                    <div class={feature_desc}>"Strict HTML/CSS validation"</div>
                </div>
                <div class={feature_card}>
                    <div class={feature_name}>"Performance"</div>
                    <div class={feature_desc}>"Optimized rendering"</div>
                </div>
            </div>
        </div>
    }
}

/// Main lesson demonstration component
#[azumi::component]
pub fn lesson9() -> impl azumi::Component {
    html! {
        <style>
            .container { padding: "20px"; }
            .header { text-align: "center"; margin-bottom: "30px"; }
            .main_title { font-size: "32px"; color: "#333"; }
            .subtitle { font-size: "18px"; color: "#666"; }
            .key_points { background: "#f9f9f9"; padding: "20px"; border-radius: "8px"; margin-bottom: "30px"; }
            .section_title { font-size: "20px"; margin-bottom: "15px"; }
            .points_list { list-style: "none"; padding: "0"; }
            .point { margin-bottom: "10px"; }
            .examples { display: "grid"; gap: "20px"; }
            .example_card { border: "1px solid #ddd"; padding: "20px"; border-radius: "8px"; }
        </style>
        <div class={container}>
            <header class={header}>
                <h1 class={main_title}>"Lesson 9: Feature Composition"</h1>
                <p class={subtitle}>"Combining multiple Azumi features"</p>
            </header>

            <section class={key_points}>
                <h2 class={section_title}>"Key Concepts"</h2>
                <ul class={points_list}>
                    <li class={point}>"Combine multiple Azumi features"</li>
                    <li class={point}>"Composition over inheritance"</li>
                    <li class={point}>"Reusable feature patterns"</li>
                    <li class={point}>"Type-safe feature integration"</li>
                    <li class={point}>"Clean separation of concerns"</li>
                </ul>
            </section>

            <section class={examples}>
                <div class={example_card}>
                    @feature_showcase()
                </div>
                <div class={example_card}>
                    @complex_feature_integration()
                </div>
            </section>
        </div>
    }
}

// Handler for Axum
pub async fn lesson9_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson9()))
}
