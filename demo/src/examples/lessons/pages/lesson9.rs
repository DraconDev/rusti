use azumi::prelude::*;

/// Lesson 9: Introducing Azumi Live
///
/// Compiler-driven optimistic UI - write Rust, get instant updates!

// #[azumi::live] auto-derives Serialize/Deserialize + LiveState trait
#[azumi::live]
pub struct Counter {
    pub count: i32,
    pub active: bool,
}

// #[azumi::live_impl] analyzes mutations and generates predictions
#[azumi::live_impl]
impl Counter {
    // Compiler detects: self.count += 1 → Prediction: "count = count + 1"
    pub fn increment(&mut self) {
        self.count += 1;
    }

    // Compiler detects: self.active = !self.active → Prediction: "active = !active"
    pub fn toggle(&mut self) {
        self.active = !self.active;
    }

    pub fn decrement(&mut self) {
        self.count -= 1;
    }

    pub fn reset(&mut self) {
        self.count = 0;
        self.active = true;
    }
}

/// Live component view - using `on:event` syntax
#[azumi::component]
pub fn counter_view<'a>(state: &'a Counter) -> impl Component + 'a {
    html! {
        <style>
            .counter_box {
                padding: "2rem";
                border: "2px solid #e0e0e0";
                border-radius: "12px";
                background: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)";
                color: "white";
                text-align: "center";
                max-width: "400px";
            }
            .value {
                font-size: "4rem";
                font-weight: "bold";
                margin: "1rem 0";
                text-shadow: "2px 2px 4px rgba(0,0,0,0.3)";
            }
            .status {
                font-size: "1.2rem";
                opacity: "0.9";
                margin-bottom: "1.5rem";
            }
            .btn_row {
                display: "flex";
                gap: "0.5rem";
                justify-content: "center";
                flex-wrap: "wrap";
            }
            .btn {
                padding: "0.75rem 1.5rem";
                font-size: "1rem";
                border: "none";
                border-radius: "8px";
                cursor: "pointer";
                transition: "transform 0.1s, opacity 0.1s";
                font-weight: "bold";
            }
            .btn:hover { transform: "scale(1.05)"; }
            .btn:active { transform: "scale(0.95)"; }
            .btn_primary { background: "#4caf50"; color: "white"; }
            .btn_secondary { background: "#ff9800"; color: "white"; }
            .btn_danger { background: "#f44336"; color: "white"; }
        </style>
        <div class={counter_box}>
            <h2>"🚀 Azumi Live Counter"</h2>

            // Display current state values
            <div class={value} data-bind="count">{state.count}</div>
            <div class={status}>
                "Status: "
                <span data-bind="active">{if state.active { "Active ✓" } else { "Inactive ✗" }}</span>
            </div>

            // on:click={state.method} auto-generates:
            // - az-on="click call Counter/increment"
            // - data-predict="count = count + 1"
            <div class={btn_row}>
                <button class={btn, btn_primary} on:click={state.increment}>
                    "+ Increment"
                </button>
                <button class={btn, btn_secondary} on:click={state.decrement}>
                    "- Decrement"
                </button>
                <button class={btn, btn_danger} on:click={state.toggle}>
                    "Toggle Status"
                </button>
            </div>
        </div>
    }
}

/// Key concepts explanation
#[azumi::component]
pub fn key_concepts() -> impl Component {
    html! {
        <style>
            .concepts {
                background: "#f8f9fa";
                padding: "1.5rem";
                border-radius: "8px";
                margin-top: "2rem";
            }
            .concept_title { color: "#333"; margin-bottom: "1rem"; }
            .concept_list { list-style: "none"; padding: "0"; }
            .concept_item {
                padding: "0.75rem";
                margin: "0.5rem 0";
                background: "white";
                border-left: "4px solid #667eea";
                border-radius: "0 4px 4px 0";
            }
            .code_inline {
                background: "#e8e8e8";
                padding: "0.2rem 0.5rem";
                border-radius: "4px";
                font-family: "monospace";
            }
        </style>
        <div class={concepts}>
            <h3 class={concept_title}>"🎯 Key Concepts"</h3>
            <ul class={concept_list}>
                <li class={concept_item}>
                    <strong>"#[azumi::live]"</strong>
                    " - Marks struct as reactive state (auto-derives Serialize/Deserialize)"
                </li>
                <li class={concept_item}>
                    <strong>"#[azumi::live_impl]"</strong>
                    " - Analyzes mutations at compile time, generates prediction DSL"
                </li>
                <li class={concept_item}>
                    <strong>"on:click={state.method}"</strong>
                    " - Declarative event binding with auto-generated optimistic updates"
                </li>
                <li class={concept_item}>
                    <strong>"Zero JS Required"</strong>
                    " - Compiler generates client-side predictions from Rust code"
                </li>
            </ul>
        </div>
    }
}

/// Main lesson page
#[azumi::component]
pub fn lesson9() -> impl Component {
    html! {
        <style>
            .container {
                max-width: "800px";
                margin: "0 auto";
                padding: "2rem";
                font-family: "system-ui, sans-serif";
            }
            .header { text-align: "center"; margin-bottom: "2rem"; }
            .main_title {
                font-size: "2.5rem";
                color: "#333";
                margin-bottom: "0.5rem";
            }
            .subtitle { font-size: "1.2rem"; color: "#666"; }
            .demo_section { margin: "2rem 0"; }
        </style>
        <div class={container}>
            <header class={header}>
                <h1 class={main_title}>"Lesson 9: Introducing Azumi Live"</h1>
                <p class={subtitle}>"Compiler-driven optimistic UI"</p>
            </header>

            <section class={demo_section}>
                // Render the live counter component
                @counter_view(state=&Counter { count: 0, active: true })
            </section>

            @key_concepts()
        </div>
    }
}

// Handler for Axum - includes azumi.js runtime
pub async fn lesson9_handler() -> axum::response::Html<String> {
    let state = Counter {
        count: 0,
        active: true,
    };

    use counter_view_component::*;
    let component_html = azumi::render_to_string(&render(
        Props::builder().state(&state).build().expect("props"),
    ));

    let concepts_html = azumi::render_to_string(&key_concepts());

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Lesson 9: Introducing Azumi Live</title>
    <style>
        body {{ 
            font-family: system-ui, sans-serif; 
            margin: 0;
            padding: 2rem;
            background: #fafafa;
        }}
        .container {{
            max-width: 800px;
            margin: 0 auto;
        }}
        .header {{ text-align: center; margin-bottom: 2rem; }}
        .main_title {{ font-size: 2.5rem; color: #333; margin-bottom: 0.5rem; }}
        .subtitle {{ font-size: 1.2rem; color: #666; }}
    </style>
</head>
<body>
    <div class="container">
        <header class="header">
            <h1 class="main_title">Lesson 9: Introducing Azumi Live</h1>
            <p class="subtitle">Compiler-driven optimistic UI</p>
        </header>
        {}
        {}
    </div>
    <script src="/static/idiomorph.js"></script>
    <script src="/static/azumi.js"></script>
</body>
</html>"#,
        component_html, concepts_html
    );

    axum::response::Html(html)
}
