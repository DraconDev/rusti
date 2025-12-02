use azumi::html;

/// Lesson 3: Global vs Component CSS
///
/// Understanding style scoping options
#[azumi::component]
pub fn global_css_example() -> impl azumi::Component {
    html! {
        // Global styles - not scoped to component
        <style global>
            body { font-family: "Arial, sans-serif"; }
            .global_class { color: "purple"; }
        </style>

        // Component-scoped styles - automatically scoped
        <style>
            .component_class { background: "#f5f5f5"; padding: "1rem"; }
            .local_class { color: "blue"; }
        </style>

        <div class={component_class}>
            <h2 class={local_class}>"Scoped Style"</h2>
            <p class="global_class">"Global Style"</p>
            <p>"This component demonstrates both global and scoped CSS"</p>
        </div>
    }
}

/// Example: Multiple components with global and scoped styles
#[azumi::component]
pub fn mixed_scoping_example() -> impl azumi::Component {
    html! {
        <style global>
            .shared_global { font-size: "1.1rem"; }
        </style>

        <style>
            .container { padding: "1rem"; border: "1px solid #ddd"; }
            .scoped_title { color: "#2196f3"; }
        </style>

        <div class={container}>
            <h3 class={scoped_title}>"Mixed Scoping Example"</h3>
            <p class="shared_global">"This text uses global styling"</p>
            <p class={scoped_title}>"This text uses scoped styling"</p>
        </div>
    }
}

/// Example: CSS scoping best practices
#[azumi::component]
pub fn scoping_best_practices() -> impl azumi::Component {
    html! {
        <style>
            .best_practices { padding: "1.5rem"; background: "#f9f9f9"; }
            .practice_item { margin: "0.5rem 0"; padding: "0.5rem"; background: "white"; }
            .do { color: "green"; font-weight: "bold"; }
            .dont { color: "red"; font-weight: "bold"; }
        </style>

        <div class={best_practices}>
            <h3>"CSS Scoping Best Practices"</h3>

            <div class={practice_item}>
                <span class={do}>"DO:"</span> " Use component-scoped styles for most cases"
            </div>

            <div class={practice_item}>
                <span class={do}>"DO:"</span> " Use global styles only for truly global elements"
            </div>

            <div class={practice_item}>
                <span class={dont}>"DON'T:"</span> " Overuse global styles - they can cause conflicts"
            </div>

            <div class={practice_item}>
                <span class={do}>"DO:"</span> " Let Azumi handle scoping automatically"
            </div>
        </div>
    }
}

/// Main lesson demonstration component
#[azumi::component]
pub fn lesson2() -> impl azumi::Component {
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
                <h1 class={main_title}>"Lesson 3: Global vs Component CSS"</h1>
                <p class={subtitle}>"Understanding style scoping options"</p>
            </header>

            <section class={key_points}>
                <h2 class={section_title}>"Key Concepts"</h2>
                <ul class={points_list}>
                    <li class={point}>"✅ Global styles use <style global> tag"</li>
                    <li class={point}>"✅ Component styles use <style> tag (automatically scoped)"</li>
                    <li class={point}>"✅ Global styles affect the entire application"</li>
                    <li class={point}>"✅ Component styles are scoped to prevent conflicts"</li>
                    <li class={point}>"✅ Azumi handles scoping automatically for component styles"</li>
                </ul>
            </section>

            <section class={examples}>
                <div class={example_card}>
                    @global_css_example()
                </div>
                <div class={example_card}>
                    @mixed_scoping_example()
                </div>
                <div class={example_card}>
                    @scoping_best_practices()
                </div>
            </section>
        </div>
    }
}

// Handler for Axum
pub async fn lesson2_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson2()))
}
