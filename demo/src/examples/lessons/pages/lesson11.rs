
use azumi::html;

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
                background: var(--primary-color);
                color: white;
                border: none;
                cursor: pointer;
                transition: background 0.3s;
            }
            .button:hover { background: var(--secondary-color); }
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
                background: var(--bg-color);
                color: var(--text-color);
                border: "1px solid var(--border-color)";
                padding: "1rem";
            }
        </style>
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
