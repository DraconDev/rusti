use azumi::html;

/// Lesson 0: Understanding Fragments
///
/// Fragments allow you to return multiple elements without a wrapper.
/// Azumi supports BOTH automatic and explicit fragments!

/// Example 1: Automatic Fragment (No explicit <> needed)
#[azumi::component]
pub fn automatic_fragment_example() -> impl azumi::Component {
    html! {
        <style>
            .container {
                padding: "20px";
                border: "1px solid #ccc";
            }
            .title {
                color: "blue";
                font-size: "24px";
            }
            .content {
                margin-top: "10px";
            }
            .footer {
                margin-top: "15px";
                font-size: "12px";
                color: "#666";
            }
        </style>
        <div class={container}>
            <h1 class={title}>"Automatic Fragments"</h1>
            <div class={content}>
                "This is some content."
            </div>
            <div class={footer}>
                "Footer content"
            </div>
        </div>
    }
}

#[azumi::component]
fn ControlFlowFragments() -> impl azumi::Component {
    html! {
        <div>"Control Flow Fragments Placeholder"</div>
    }
}

/// Example 2: Explicit Fragment with <>
#[azumi::component]
pub fn explicit_fragment_example() -> impl azumi::Component {
    html! {
        <style>
            .container {
                padding: "20px";
                border: "1px solid #ccc";
            }
            .title {
                color: "blue";
                font-size: "24px";
            }
            .text {
                font-size: "16px";
            }
        </style>
        <div class={container}>
            <h1 class={title}>"Explicit Fragments"</h1>
            <>
                <p class={text}>"You can use explicit &lt;&gt; for clarity."</p>
                <p class={text}>"Functionally identical to automatic!"</p>
            </>
        </div>
    }
}

/// Example 3: Fragments in Control Flow
#[azumi::component]
pub fn control_flow_fragments() -> impl azumi::Component {
    let show_content = true;
    html! {
        <style>
            .container { padding: "20px"; }
            .title { color: "blue"; }
            .box { margin: "10px"; border: "1px solid #eee"; padding: "10px"; }
            .subtitle { font-weight: "bold"; }
            .text { color: "#333"; }
        </style>
        <div class={container}>
            <h1 class={title}>"Fragments in Control Flow"</h1>
            <div class={box}>
                <h3 class={subtitle}>"Automatic (No):"</h3>
                @if show_content {
                    <p class={text}>"First element"</p>
                    <p class={text}>"Second element"</p>
                }
            </div>
            <div class={box}>
                <h3 class={subtitle}>"Explicit (With):"</h3>
                @if show_content {
                    <>
                        <p class={text}>"First element"</p>
                        <p class={text}>"Second element"</p>
                    </>
                }
            </div>
        </div>
    }
}

/// Example 4: When Fragments Are Useful
#[azumi::component]
pub fn fragment_use_cases() -> impl azumi::Component {
    

    html! {
        <style>
            .container { padding: "20px"; }
            .title { font-size: "24px"; }
            .section { margin-bottom: "20px"; }
            .subtitle { font-size: "18px"; }
            .comparison { display: "flex"; gap: "20px"; }
            .before { flex: "1"; border: "1px solid red"; padding: "10px"; }
            .after { flex: "1"; border: "1px solid green"; padding: "10px"; }
            .code { background: "#eee"; padding: "5px"; display: "block"; }
            .list { list-style: "none"; }
            .item { font-weight: "bold"; }
            .item_note { font-style: "italic"; color: "#666"; }
            .text { margin-top: "5px"; }
        </style>
        <div class={container}>
            <h1 class={title}>"When to Use Fragments"</h1>

            <section class={section}>
                <h3 class={subtitle}>"1. Avoiding Unnecessary Wrappers"</h3>
                <div class={comparison}>
                    <div class={before}>
                        <h4>"❌ Without Fragment (Extra div)"</h4>
                        <code class={code}>"Title"</code>
                    </div>
                    <div class={after}>
                        <h4>"✅ With Fragment (Clean)"</h4>
                        <code class={code}>"Title"</code>
                    </div>
                </div>
            </section>

            <section class={section}>
                <h3 class={subtitle}>"2. In Loops"</h3>
                <ul class={list}>
                    @let items = ["Apple", "Banana", "Cherry"];
                    @for item in &items {
                        <>
                            <li class={item}>{item}</li>
                            <li class={item_note}>"(Note: " {item} ")"</li>
                        </>
                    }
                </ul>
            </section>

            <section class={section}>``
                <h3 class={subtitle}>"3. Component Returns"</h3>
                <p class={text}>"Components can return multiple elements without wrappers using fragments!"</p>
            </section>
        </div>
    }
}

/// Main demonstration component
#[azumi::component]
pub fn lesson0() -> impl azumi::Component {
    
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
                <h1 class={main_title}>"Lesson 0: Fragments"</h1>
                <p class={subtitle}>"Multiple Elements Without Wrapper Divs"</p>
            </header>

            <section class={key_points}>
                <h2 class={section_title}>"Key Points"</h2>
                <ul class={points_list}>
                    <li class={point}>"✅ Automatic: Multiple elements work without explicit &lt;&gt;"</li>
                    <li class={point}>"✅ Explicit: Use for semantic clarity"</li>
                    <li class={point}>"✅ Both generate identical output"</li>
                    <li class={point}>"✅ Works in control flow (@if, @for, @match)"</li>
                    <li class={point}>"✅ Avoids unnecessary DOM nesting"</li>
                </ul>
            </section>

            <section class={examples}>
                <div class={example_card}>
                    @automatic_fragment_example()
                </div>
                <div class={example_card}>
                    @explicit_fragment_example()
                </div>
                <div class={example_card}>
                    @control_flow_fragments()
                </div>
                <div class={example_card}>
                    @fragment_use_cases()
                </div>
            </section>
        </div>
    }
}

// Handler for Axum
pub async fn handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson0()))
}
