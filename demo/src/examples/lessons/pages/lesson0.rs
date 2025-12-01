use azumi::html;

/// Lesson 0: Understanding Fragments
///
/// Fragments allow you to return multiple elements without a wrapper.
/// Azumi supports BOTH automatic and explicit fragments!

/// Example 1: Automatic Fragment (No explicit <> needed)
#[azumi::component]
pub fn automatic_fragment_example() -> impl azumi::Component {
    style! {
        .lesson0-container {
            padding: "20px";
            border: "1px solid #ccc";
        }
        .lesson0-title {
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
    }

    html! {
        <div class={lesson0_container}>
            <h1 class={lesson0_title}>"Automatic Fragments"</h1>
            <div class={content}>
                "This is some content."
            </div>
            <div class={footer}>
                "Footer content"
            </div>
        </div>
    }
}

/// Example 2: Explicit Fragment with <>
#[azumi::component]
pub fn explicit_fragment_example() -> impl azumi::Component {
    style! {
        .lesson0-container {
            padding: "20px";
            border: "1px solid #ccc";
        }
        .lesson0-title {
            color: "blue";
            font-size: "24px";
        }
        .lesson0-text {
            font-size: "16px";
        }
    }
    html! {
        <div class={lesson0_container}>
            <h1 class={lesson0_title}>"Explicit Fragments"</h1>
            <>
                <p class={lesson0_text}>"You can use explicit &lt;&gt; for clarity."</p>
                <p class={lesson0_text}>"Functionally identical to automatic!"</p>
            </>
        </div>
    }
}

/// Example 3: Fragments in Control Flow
#[azumi::component]
pub fn control_flow_fragments() -> impl azumi::Component {
    let show_content = true;
    style! {
        .lesson0-container { padding: "20px"; }
        .lesson0-title { color: "blue"; }
        .lesson0-box { margin: "10px"; border: "1px solid #eee"; padding: "10px"; }
        .lesson0-subtitle { font-weight: "bold"; }
        .lesson0-text { color: "#333"; }
    }
    html! {
        <div class={lesson0_container}>
            <h1 class={lesson0_title}>"Fragments in Control Flow"</h1>
            <div class={lesson0_box}>
                <h3 class={lesson0_subtitle}>"Automatic (No &lt;&gt;):"</h3>
                @if show_content {
                    <p class={lesson0_text}>"First element"</p>
                    <p class={lesson0_text}>"Second element"</p>
                }
            </div>
            <div class={lesson0_box}>
                <h3 class={lesson0_subtitle}>"Explicit (With &lt;&gt;):"</h3>
                @if show_content {
                    <>
                        <p class={lesson0_text}>"First element"</p>
                        <p class={lesson0_text}>"Second element"</p>
                    </>
                }
            </div>
        </div>
    }
}

/// Example 4: When Fragments Are Useful
#[azumi::component]
pub fn fragment_use_cases() -> impl azumi::Component {
    let items = ["Apple", "Banana", "Cherry"];

    style! {
        .lesson0-container { padding: "20px"; }
        .lesson0-title { font-size: "24px"; }
        .lesson0-section { margin-bottom: "20px"; }
        .lesson0-subtitle { font-size: "18px"; }
        .lesson0-comparison { display: "flex"; gap: "20px"; }
        .lesson0-before { flex: "1"; border: "1px solid red"; padding: "10px"; }
        .lesson0-after { flex: "1"; border: "1px solid green"; padding: "10px"; }
        .lesson0-code { background: "#eee"; padding: "5px"; display: "block"; }
        .lesson0-list { list_style: "none"; }
        .lesson0-item { font-weight: "bold"; }
        .lesson0-item-note { font-style: "italic"; color: "#666"; }
        .lesson0-text { margin-top: "5px"; }
    }

    html! {
        <div class={lesson0_container}>
            <h1 class={lesson0_title}>"When to Use Fragments"</h1>

            <section class={lesson0_section}>
                <h3 class={lesson0_subtitle}>"1. Avoiding Unnecessary Wrappers"</h3>
                <div class={lesson0_comparison}>
                    <div class={lesson0_before}>
                        <h4>"❌ Without Fragment (Extra div)"</h4>
                        <code class={lesson0_code}>"&lt;div&gt;\n  &lt;h2&gt;Title&lt;/h2&gt;\n  &lt;p&gt;Text&lt;/p&gt;\n&lt;/div&gt;"</code>
                    </div>
                    <div class={lesson0_after}>
                        <h4>"✅ With Fragment (Clean)"</h4>
                        <code class={lesson0_code}>"&lt;&gt;\n  &lt;h2&gt;Title&lt;/h2&gt;\n  &lt;p&gt;Text&lt;/p&gt;\n&lt;/&gt;"</code>
                    </div>
                </div>
            </section>

            <section class={lesson0_section}>
                <h3 class={lesson0_subtitle}>"2. In Loops"</h3>
                <ul class={lesson0_list}>
                    @for item in &items {
                        <>
                            <li class={lesson0_item}>{item}</li>
                            <li class={lesson0_item_note}>"(Note: " {item} ")"</li>
                        </>
                    }
                </ul>
            </section>

            <section class={lesson0_section}>
                <h3 class={lesson0_subtitle}>"3. Component Returns"</h3>
                <p class={lesson0_text}>"Components can return multiple elements without wrappers using fragments!"</p>
            </section>
        </div>
    }
}

/// Main demonstration component
#[azumi::component]
pub fn lesson0() -> impl azumi::Component {
    style! {
        .lesson0-container { padding: "20px"; }
        .lesson0-header { text-align: "center"; margin-bottom: "30px"; }
        .lesson0-main-title { font-size: "32px"; color: "#333"; }
        .lesson0-subtitle { font-size: "18px"; color: "#666"; }
        .lesson0-key-points { background: "#f9f9f9"; padding: "20px"; border-radius: "8px"; margin-bottom: "30px"; }
        .lesson0-section-title { font-size: "20px"; margin-bottom: "15px"; }
        .lesson0-points-list { list_style: "none"; padding: "0"; }
        .lesson0-point { margin-bottom: "10px"; }
        .lesson0-examples { display: "grid"; gap: "20px"; }
        .lesson0-example-card { border: "1px solid #ddd"; padding: "20px"; border-radius: "8px"; }
    }

    html! {
        <div class={lesson0_container}>
            <header class={lesson0_header}>
                <h1 class={lesson0_main_title}>"Lesson 0: Fragments"</h1>
                <p class={lesson0_subtitle}>"Multiple Elements Without Wrapper Divs"</p>
            </header>

            <section class={lesson0_key_points}>
                <h2 class={lesson0_section_title}>"Key Points"</h2>
                <ul class={lesson0_points_list}>
                    <li class={lesson0_point}>"✅ Automatic: Multiple elements work without explicit &lt;&gt;"</li>
                    <li class={lesson0_point}>"✅ Explicit: Use &lt;&gt; &lt;/&gt; for semantic clarity"</li>
                    <li class={lesson0_point}>"✅ Both generate identical output"</li>
                    <li class={lesson0_point}>"✅ Works in control flow (@if, @for, @match)"</li>
                    <li class={lesson0_point}>"✅ Avoids unnecessary DOM nesting"</li>
                </ul>
            </section>

            <section class={lesson0_examples}>
                <div class={lesson0_example_card}>
                    @automatic_fragment_example()
                </div>
                <div class={lesson0_example_card}>
                    @explicit_fragment_example()
                </div>
                <div class={lesson0_example_card}>
                    @control_flow_fragments()
                </div>
                <div class={lesson0_example_card}>
                    @fragment_use_cases()
                </div>
            </section>
        </div>
    }
}

// Handler for Axum
pub async fn lesson0_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson0()))
}
