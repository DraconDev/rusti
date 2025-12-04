
/// Lesson 11: Advanced CSS Features
///
/// CSS variables, pseudo-classes, and media queries
#[azumi::component]
pub fn advanced_css_example() -> impl azumi::Component {
    html! {
        <style>
            :root {
                --primary-color: #2196f3;
                --secondary-color: #ff4081;
            }
            .advanced { padding: "2rem"; }
            .button {
                padding: "0.75rem 1.5rem";
                background: "var(--primary-color)";
                color: "white";
                border: "none";
                cursor: "pointer";
                transition: "background 0.3s";
            }
            .button:hover { background: "var(--secondary-color)"; }
            .responsive { color: "blue"; }
            @media (max-width: 600px) {
                .responsive { color: "red"; }
            }
        </style>
        <div class={advanced}>
            <button class={button}>"Hover Me"</button>
            <p class={responsive}>"Responsive Text"</p>
        </div>
    }
}

/// Example: CSS variables and theming
#[azumi::component]
pub fn css_variables_example() -> impl azumi::Component {
    html! {
        <style>
            .variables_container { padding: "1.5rem"; background: "#f9f9f9"; }
            .theme_demo { display: "grid"; gap: "1rem"; }
            .theme_card { padding: "1rem"; border: "1px solid #ddd"; }
            .light_theme {
                --bg-color: white;
                --text-color: #333;
                --border-color: #ddd;
            }
            .dark_theme {
                --bg-color: #333;
                --text-color: white;
                --border-color: #666;
            }
            .theme_content {
                background: "var(--bg-color)";
                color: "var(--text-color)";
                border: "1px solid var(--border-color)";
                padding: "1rem";
            }
        </style>
        <div class={variables_container}>
        <div class={variables_container}>
            <h3>"CSS Variables Demo"</h3>

            <div class={theme_demo}>
                <div class={theme_card} class={light_theme}>
                    <h4>"Light Theme"</h4>
                    <div class={theme_content}>"Light theme using CSS variables"</div>
                </div>

                <div class={theme_card} class={dark_theme}>
                    <h4>"Dark Theme"</h4>
                    <div class={theme_content}>"Dark theme using CSS variables"</div>
                </div>
            </div>
        </div>
    }
}

/// Example: Responsive design patterns
#[azumi::component]
pub fn responsive_design_example() -> impl azumi::Component {
    html! {
        <style>
            .responsive_container { padding: "1.5rem"; }
            .responsive_grid {
                display: "grid";
                grid-template-columns: "repeat(3, 1fr)";
                gap: "1rem";
            }
            .grid_item { padding: "1rem"; background: "#f0f0f0"; border: "1px solid #eee"; }
            @media (max-width: 768px) {
                .responsive_grid {
                    grid-template-columns: "repeat(2, 1fr)";
                }
            }
            @media (max-width: 480px) {
                .responsive_grid {
                    grid-template-columns: "1fr";
                }
            }
        </style>
        <div class={responsive_container}>
            <h3>"Responsive Grid"</h3>

            <div class={responsive_grid}>
                <div class={grid_item}>"Item 1"</div>
                <div class={grid_item}>"Item 2"</div>
                <div class={grid_item}>"Item 3"</div>
                <div class={grid_item}>"Item 4"</div>
                <div class={grid_item}>"Item 5"</div>
                <div class={grid_item}>"Item 6"</div>
            </div>

            <p>"This grid adapts to different screen sizes"</p>
        </div>
    }
}

/// Main lesson demonstration component
#[azumi::component]
pub fn lesson11() -> impl azumi::Component {
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
                <h1 class={main_title}>"Lesson 11: Advanced CSS Features"</h1>
                <p class={subtitle}>"CSS variables, pseudo-classes, and media queries"</p>
            </header>

            <section class={key_points}>
                <h2 class={section_title}>"Key Concepts"</h2>
                <ul class={points_list}>
                    <li class={point}>"✅ CSS variables for theming"</li>
                    <li class={point}>"✅ Pseudo-classes for interactivity"</li>
                    <li class={point}>"✅ Media queries for responsiveness"</li>
                    <li class={point}>"✅ Advanced CSS features supported"</li>
                    <li class={point}>"✅ All validated at compile time"</li>
                </ul>
            </section>

            <section class={examples}>
                <div class={example_card}>
                    @advanced_css_example()
                </div>
                <div class={example_card}>
                    @css_variables_example()
                </div>
                <div class={example_card}>
                    @responsive_design_example()
                </div>
            </section>
        </div>
    }
}

// Handler for Axum
pub async fn lesson11_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson11()))
}
