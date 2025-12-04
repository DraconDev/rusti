use azumi::prelude::*;

/// Lesson 10: Live Components with Auto-Detection
///
/// How the component macro auto-detects live state

// Define a Like Button state
#[azumi::live]
pub struct LikeButton {
    pub liked: bool,
    pub count: i32,
}

#[azumi::live_impl]
impl LikeButton {
    pub fn toggle(&mut self) {
        self.liked = !self.liked;
        if self.liked {
            self.count += 1;
        } else {
            self.count -= 1;
        }
    }
}

/// Like button component - auto-detects live state from first parameter
#[azumi::component]
pub fn like_button_view<'a>(state: &'a LikeButton) -> impl Component + 'a {
    html! {
        <style>
            .like_container {
                display: "inline-flex";
                align-items: "center";
                gap: "0.5rem";
            }
            .like_btn {
                padding: "0.75rem 1.5rem";
                font-size: "1.5rem";
                border: "2px solid #e91e63";
                border-radius: "50px";
                background: "transparent";
                cursor: "pointer";
                transition: "all 0.2s ease";
            }
            .like_btn:hover {
                background: "#fce4ec";
                transform: "scale(1.1)";
            }
            .like_count {
                font-size: "1.2rem";
                font-weight: "bold";
                color: "#e91e63";
            }
        </style>
        <div class={like_container}>
            // on:click={state.toggle} auto-generates predictions for toggle
            <button class={like_btn} on:click={state.toggle}>
                {if state.liked { "❤️" } else { "🤍" }}
            </button>
            <span class={like_count} data-bind="count">{state.count}</span>
        </div>
    }
}

// Theme toggle state
#[azumi::live]
pub struct ThemeToggle {
    pub dark_mode: bool,
}

#[azumi::live_impl]
impl ThemeToggle {
    pub fn toggle(&mut self) {
        self.dark_mode = !self.dark_mode;
    }
}

/// Theme toggle component
#[azumi::component]
pub fn theme_toggle_view<'a>(state: &'a ThemeToggle) -> impl Component + 'a {
    html! {
        <style>
            .theme_toggle {
                padding: "1rem";
                border-radius: "8px";
                transition: "all 0.3s ease";
            }
            .theme_light {
                background: "#ffffff";
                color: "#333333";
                border: "1px solid #ddd";
            }
            .theme_dark {
                background: "#1e1e1e";
                color: "#ffffff";
                border: "1px solid #444";
            }
            .toggle_btn {
                padding: "0.5rem 1rem";
                border: "none";
                border-radius: "4px";
                cursor: "pointer";
                font-size: "1rem";
            }
        </style>
        <div class={theme_toggle, if state.dark_mode { "theme_dark" } else { "theme_light" }}>
            <p>"Current theme: " {if state.dark_mode { "🌙 Dark" } else { "☀️ Light" }}</p>
            <button class={toggle_btn} on:click={state.toggle}>
                "Toggle Theme"
            </button>
        </div>
    }
}

// Accordion state
#[azumi::live]
pub struct Accordion {
    pub open: bool,
}

#[azumi::live_impl]
impl Accordion {
    pub fn toggle(&mut self) {
        self.open = !self.open;
    }
}

/// Accordion component
#[azumi::component]
pub fn accordion_view<'a>(
    state: &'a Accordion,
    title: &'a str,
    content: &'a str,
) -> impl Component + 'a {
    html! {
        <style>
            .accordion {
                border: "1px solid #ddd";
                border-radius: "8px";
                overflow: "hidden";
            }
            .accordion_header {
                padding: "1rem";
                background: "#f5f5f5";
                cursor: "pointer";
                display: "flex";
                justify-content: "space-between";
                align-items: "center";
                border: "none";
                width: "100%";
                font-size: "1rem";
                text-align: "left";
            }
            .accordion_header:hover { background: "#eeeeee"; }
            .accordion_content {
                padding: "1rem";
                background: "white";
                border-top: "1px solid #ddd";
            }
            .accordion_icon {
                transition: "transform 0.2s";
            }
        </style>
        <div class={accordion}>
            <button class={accordion_header} on:click={state.toggle}>
                <span>{title}</span>
                <span class={accordion_icon}>{if state.open { "▼" } else { "▶" }}</span>
            </button>
            @if state.open {
                <div class={accordion_content}>
                    {content}
                </div>
            }
        </div>
    }
}

/// Main lesson page
#[azumi::component]
pub fn lesson10() -> impl Component {
    html! {
        <style>
            .container { max-width: "800px"; margin: "0 auto"; padding: "2rem"; }
            .header { text-align: "center"; margin-bottom: "2rem"; }
            .main_title { font-size: "2rem"; color: "#333"; }
            .subtitle { color: "#666"; }
            .demo_grid { display: "grid"; gap: "2rem"; margin: "2rem 0"; }
            .demo_card {
                padding: "1.5rem";
                border: "1px solid #eee";
                border-radius: "12px";
                background: "white";
            }
            .demo_title { color: "#2196f3"; margin-bottom: "1rem"; }
            .code_block {
                background: "#2d2d2d";
                color: "#f8f8f2";
                padding: "1rem";
                border-radius: "8px";
                font-family: "monospace";
                font-size: "0.85rem";
                overflow-x: "auto";
                margin: "1rem 0";
            }
        </style>
        <div class={container}>
            <header class={header}>
                <h1 class={main_title}>"Lesson 10: Auto-Detection"</h1>
                <p class={subtitle}>"Components automatically detect live state"</p>
            </header>

            <div class={demo_grid}>
                <div class={demo_card}>
                    <h3 class={demo_title}>"❤️ Like Button"</h3>
                    <p>"Click to toggle like state - instant UI update!"</p>
                    @like_button_view(state=&LikeButton { liked: false, count: 42 })
                </div>

                <div class={demo_card}>
                    <h3 class={demo_title}>"🎨 Theme Toggle"</h3>
                    <p>"Toggle between light and dark themes"</p>
                    @theme_toggle_view(state=&ThemeToggle { dark_mode: false })
                </div>

                <div class={demo_card}>
                    <h3 class={demo_title}>"📂 Accordion"</h3>
                    @accordion_view(
                        state=&Accordion { open: false },
                        title="Click to expand",
                        content="This content is revealed with an instant toggle animation!"
                    )
                </div>
            </div>

            <div class={code_block}>
                "// Auto-detection happens when first param is `state: &T`\n"
                "#[azumi::component]\n"
                "fn my_view<'a>(state: &'a MyLiveState) -> impl Component + 'a {\n"
                "    html! {\n"
                "        <button on:click={state.action}>\"Click me\"</button>\n"
                "    }\n"
                "}"
            </div>
        </div>
    }
}

// Handler for Axum
pub async fn lesson10_handler() -> axum::response::Html<String> {
    // Create initial states
    let like_state = LikeButton {
        liked: false,
        count: 42,
    };
    let theme_state = ThemeToggle { dark_mode: false };
    let accordion_state = Accordion { open: false };

    use accordion_view_component::Props as AccordionProps;
    use like_button_view_component::Props as LikeProps;
    use theme_toggle_view_component::Props as ThemeProps;

    let like_html = azumi::render_to_string(&like_button_view_component::render(
        LikeProps::builder()
            .state(&like_state)
            .build()
            .expect("props"),
    ));

    let theme_html = azumi::render_to_string(&theme_toggle_view_component::render(
        ThemeProps::builder()
            .state(&theme_state)
            .build()
            .expect("props"),
    ));

    let accordion_html = azumi::render_to_string(&accordion_view_component::render(
        AccordionProps::builder()
            .state(&accordion_state)
            .title("Click to expand")
            .content("This content is revealed with an instant toggle animation!")
            .build()
            .expect("props"),
    ));

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Lesson 10: Live Components Auto-Detection</title>
    <style>
        body {{ 
            font-family: system-ui, sans-serif; 
            margin: 0;
            padding: 2rem;
            background: #fafafa;
        }}
        .container {{ max-width: 800px; margin: 0 auto; }}
        .header {{ text-align: center; margin-bottom: 2rem; }}
        .main_title {{ font-size: 2rem; color: #333; }}
        .subtitle {{ color: #666; }}
        .demo_grid {{ display: grid; gap: 2rem; margin: 2rem 0; }}
        .demo_card {{ 
            padding: 1.5rem; 
            border: 1px solid #eee; 
            border-radius: 12px;
            background: white;
        }}
        .demo_title {{ color: #2196f3; margin-bottom: 1rem; }}
    </style>
</head>
<body>
    <div class="container">
        <header class="header">
            <h1 class="main_title">Lesson 10: Auto-Detection</h1>
            <p class="subtitle">Components automatically detect live state</p>
        </header>
        
        <div class="demo_grid">
            <div class="demo_card">
                <h3 class="demo_title">❤️ Like Button</h3>
                <p>Click to toggle like state - instant UI update!</p>
                {}
            </div>
            
            <div class="demo_card">
                <h3 class="demo_title">🎨 Theme Toggle</h3>
                <p>Toggle between light and dark themes</p>
                {}
            </div>
            
            <div class="demo_card">
                <h3 class="demo_title">📂 Accordion</h3>
                {}
            </div>
        </div>
    </div>
    <script src="/static/idiomorph.js"></script>
    <script src="/static/azumi.js"></script>
</body>
</html>"#,
        like_html, theme_html, accordion_html
    );

    axum::response::Html(html)
}
