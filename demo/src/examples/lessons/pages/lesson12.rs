use azumi::prelude::*;

/// Lesson 12: How Optimistic UI Works
///
/// Understanding the prediction → confirm flow

// Demo state for showing the flow
#[azumi::live]
pub struct FlowDemo {
    pub count: i32,
    pub status: String,
}

#[azumi::live_impl]
impl FlowDemo {
    pub fn increment(&mut self) {
        self.count += 1;
        self.status = "Updated!".to_string();
    }
}

/// Flow visualization component
#[azumi::component]
pub fn flow_demo<'a>(state: &'a FlowDemo) -> impl Component + 'a {
    html! {
        <style>
            .flow_demo {
                padding: "2rem";
                background: "white";
                border-radius: "12px";
                border: "1px solid #e0e0e0";
            }
            .flow_value {
                font-size: "3rem";
                font-weight: "bold";
                color: "#667eea";
                text-align: "center";
                margin: "1rem 0";
            }
            .flow_btn {
                display: "block";
                width: "100%";
                padding: "1rem";
                font-size: "1.2rem";
                background: "#667eea";
                color: "white";
                border: "none";
                border-radius: "8px";
                cursor: "pointer";
            }
            .flow_btn:hover { background: "#5a67d8"; }
        </style>
        <div class={flow_demo}>
            <h3>"Interactive Demo"</h3>
            <div class={flow_value} data-bind="count">{state.count}</div>
            <button class={flow_btn} on:click={state.increment}>
                "Click to Increment"
            </button>
            <p>"Status: " <span data-bind="status">{&state.status}</span></p>
        </div>
    }
}

/// Flow diagram component
#[azumi::component]
pub fn flow_diagram() -> impl Component {
    html! {
        <style>
            .diagram {
                background: "#1e1e2e";
                color: "#cdd6f4";
                padding: "1.5rem";
                border-radius: "12px";
                font-family: "monospace";
                font-size: "0.9rem";
                overflow-x: "auto";
            }
            .diagram_title {
                color: "#89b4fa";
                margin-bottom: "1rem";
                font-size: "1.1rem";
            }
            .step {
                padding: "0.75rem";
                margin: "0.5rem 0";
                border-radius: "6px";
            }
            .step_user { background: "#45475a"; border-left: "4px solid #89b4fa"; }
            .step_client { background: "#45475a"; border-left: "4px solid #a6e3a1"; }
            .step_server { background: "#45475a"; border-left: "4px solid #f9e2af"; }
            .step_reconcile { background: "#45475a"; border-left: "4px solid #f38ba8"; }
            .step_num {
                display: "inline-block";
                width: "24px";
                height: "24px";
                line-height: "24px";
                text-align: "center";
                background: "#585b70";
                border-radius: "50%";
                margin-right: "0.5rem";
                font-size: "0.8rem";
            }
        </style>
        <div class={diagram}>
            <div class={diagram_title}>"🔄 Optimistic UI Flow"</div>

            <div class={step, step_user}>
                <span class={step_num}>"1"</span>
                "User clicks button with on:click={state.increment}"
            </div>

            <div class={step, step_client}>
                <span class={step_num}>"2"</span>
                "INSTANT: Execute data-predict=\"count = count + 1\" locally (0ms!)"
            </div>

            <div class={step, step_server}>
                <span class={step_num}>"3"</span>
                "ASYNC: POST to /_azumi/action/FlowDemo/increment"
            </div>

            <div class={step, step_reconcile}>
                <span class={step_num}>"4"</span>
                "RECONCILE: Server HTML morphed into DOM (prediction verified)"
            </div>
        </div>
    }
}

/// Prediction table
#[azumi::component]
pub fn prediction_table() -> impl Component {
    html! {
        <style>
            .pred_table {
                width: "100%";
                border-collapse: "collapse";
                margin: "1rem 0";
                font-size: "0.9rem";
            }
            .pred_header {
                background: "#f8f9fa";
                text-align: "left";
                padding: "0.75rem";
                border-bottom: "2px solid #dee2e6";
            }
            .pred_cell {
                padding: "0.75rem";
                border-bottom: "1px solid #dee2e6";
                font-family: "monospace";
            }
            .rust_code { color: "#e91e63"; }
            .pred_code { color: "#4caf50"; }
        </style>
        <table class={pred_table}>
            <thead>
                <tr>
                    <th class={pred_header}>"Rust Pattern"</th>
                    <th class={pred_header}>"Generated Prediction"</th>
                    <th class={pred_header}>"Result"</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td class={pred_cell}>
                        <span class={rust_code}>"self.x = !self.x"</span>
                    </td>
                    <td class={pred_cell}>
                        <span class={pred_code}>"x = !x"</span>
                    </td>
                    <td class={pred_cell}>"Toggle boolean"</td>
                </tr>
                <tr>
                    <td class={pred_cell}>
                        <span class={rust_code}>"self.x = true"</span>
                    </td>
                    <td class={pred_cell}>
                        <span class={pred_code}>"x = true"</span>
                    </td>
                    <td class={pred_cell}>"Literal assignment"</td>
                </tr>
                <tr>
                    <td class={pred_cell}>
                        <span class={rust_code}>"self.x += 1"</span>
                    </td>
                    <td class={pred_cell}>
                        <span class={pred_code}>"x = x + 1"</span>
                    </td>
                    <td class={pred_cell}>"Increment"</td>
                </tr>
                <tr>
                    <td class={pred_cell}>
                        <span class={rust_code}>"self.x -= 1"</span>
                    </td>
                    <td class={pred_cell}>
                        <span class={pred_code}>"x = x - 1"</span>
                    </td>
                    <td class={pred_cell}>"Decrement"</td>
                </tr>
            </tbody>
        </table>
    }
}

/// Benefits section
#[azumi::component]
pub fn benefits() -> impl Component {
    html! {
        <style>
            .benefits {
                display: "grid";
                grid-template-columns: "repeat(2, 1fr)";
                gap: "1rem";
                margin: "2rem 0";
            }
            .benefit_card {
                padding: "1.5rem";
                background: "white";
                border-radius: "8px";
                border: "1px solid #e0e0e0";
            }
            .benefit_icon { font-size: "2rem"; margin-bottom: "0.5rem"; }
            .benefit_title { font-weight: "bold"; color: "#333"; margin-bottom: "0.5rem"; }
            .benefit_desc { color: "#666"; font-size: "0.9rem"; }
        </style>
        <div class={benefits}>
            <div class={benefit_card}>
                <div class={benefit_icon}>"⚡"</div>
                <div class={benefit_title}>"0ms Perceived Latency"</div>
                <div class={benefit_desc}>"UI updates instantly before server confirms"</div>
            </div>
            <div class={benefit_card}>
                <div class={benefit_icon}>"🔒"</div>
                <div class={benefit_title}>"Server is Truth"</div>
                <div class={benefit_desc}>"Server always wins - can't trust client"</div>
            </div>
            <div class={benefit_card}>
                <div class={benefit_icon}>"🚫"</div>
                <div class={benefit_title}>"No JavaScript"</div>
                <div class={benefit_desc}>"Compiler generates all client logic"</div>
            </div>
            <div class={benefit_card}>
                <div class={benefit_icon}>"🛡️"</div>
                <div class={benefit_title}>"Compile-Time Safety"</div>
                <div class={benefit_desc}>"Bugs caught at build time, not runtime"</div>
            </div>
        </div>
    }
}

/// Main lesson page
#[azumi::component]
pub fn lesson12() -> impl Component {
    html! {
        <style>
            .container { max-width: "900px"; margin: "0 auto"; padding: "2rem"; }
            .header { text-align: "center"; margin-bottom: "2rem"; }
            .main_title { font-size: "2rem"; color: "#333"; }
            .subtitle { color: "#666"; }
            .two_col { display: "grid"; grid-template-columns: "1fr 1fr"; gap: "2rem"; margin: "2rem 0"; }
            .section { margin: "2rem 0"; }
            .section_title { color: "#2196f3"; margin-bottom: "1rem"; }
        </style>
        <div class={container}>
            <header class={header}>
                <h1 class={main_title}>"Lesson 12: Optimistic UI Flow"</h1>
                <p class={subtitle}>"Understanding prediction → confirm"</p>
            </header>

            <div class={two_col}>
                @flow_diagram()
                @flow_demo(state=&FlowDemo { count: 0, status: "Ready".to_string() })
            </div>

            <section class={section}>
                <h2 class={section_title}>"📊 Supported Predictions"</h2>
                @prediction_table()
            </section>

            <section class={section}>
                <h2 class={section_title}>"🎯 Why This Matters"</h2>
                @benefits()
            </section>
        </div>
    }
}

// Handler for Axum
pub async fn lesson12_handler() -> axum::response::Html<String> {
    let flow_state = FlowDemo {
        count: 0,
        status: "Ready".to_string(),
    };

    use flow_demo_component::Props as FlowProps;
    let flow_html = azumi::render_to_string(&flow_demo_component::render(
        FlowProps::builder()
            .state(&flow_state)
            .build()
            .expect("props"),
    ));

    let diagram_html = azumi::render_to_string(&flow_diagram());
    let table_html = azumi::render_to_string(&prediction_table());
    let benefits_html = azumi::render_to_string(&benefits());

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Lesson 12: Optimistic UI Flow</title>
    <style>
        body {{ 
            font-family: system-ui, sans-serif; 
            margin: 0;
            padding: 2rem;
            background: #fafafa;
        }}
        .container {{ max-width: 900px; margin: 0 auto; }}
        .header {{ text-align: center; margin-bottom: 2rem; }}
        .main_title {{ font-size: 2rem; color: #333; }}
        .subtitle {{ color: #666; }}
        .two_col {{ display: grid; grid-template-columns: 1fr 1fr; gap: 2rem; margin: 2rem 0; }}
        .section {{ margin: 2rem 0; }}
        .section_title {{ color: #2196f3; margin-bottom: 1rem; }}
        @media (max-width: 768px) {{
            .two_col {{ grid-template-columns: 1fr; }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <header class="header">
            <h1 class="main_title">Lesson 12: Optimistic UI Flow</h1>
            <p class="subtitle">Understanding prediction → confirm</p>
        </header>
        
        <div class="two_col">
            {}
            {}
        </div>
        
        <section class="section">
            <h2 class="section_title">📊 Supported Predictions</h2>
            {}
        </section>
        
        <section class="section">
            <h2 class="section_title">🎯 Why This Matters</h2>
            {}
        </section>
    </div>
    <script src="/static/idiomorph.js"></script>
    <script src="/static/azumi.js"></script>
</body>
</html>"#,
        diagram_html, flow_html, table_html, benefits_html
    );

    axum::response::Html(html)
}
