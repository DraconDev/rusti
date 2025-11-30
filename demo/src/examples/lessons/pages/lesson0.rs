use azumi::html;

/// Lesson 0: Understanding Fragments
///
/// Fragments allow you to return multiple elements without a wrapper.
/// Azumi supports BOTH automatic and explicit fragments!

/// Example 1: Automatic Fragment (No explicit <> needed)
#[azumi::component]
pub fn automatic_fragment_example() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson0.css" />
        <div class="lesson0-container">
            <h1 class="lesson0-title">"Automatic Fragments"</h1>
            <p class="lesson0-text">"Multiple elements work automatically!"</p>
            <p class="lesson0-text">"No wrapper needed at the top level."</p>
        </div>
    }
}

/// Example 2: Explicit Fragment with <>
#[azumi::component]
pub fn explicit_fragment_example() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson0.css" />
        <div class="lesson0-container">
            <h1 class="lesson0-title">"Explicit Fragments"</h1>
            <>
                <p class="lesson0-text">"You can use explicit &lt;&gt; for clarity."</p>
                <p class="lesson0-text">"Functionally identical to automatic!"</p>
            </>
        </div>
    }
}

/// Example 3: Fragments in Control Flow
#[azumi::component]
pub fn control_flow_fragments() -> impl azumi::Component {
    let show_content = true;
    html! {
        <style src="/static/pages/lesson0.css" />
        <div class="lesson0-container">
            <h1 class="lesson0-title">"Fragments in Control Flow"</h1>
            <div class="lesson0-box">
                <h3 class="lesson0-subtitle">"Automatic (No &lt;&gt;):"</h3>
                @if show_content {
                    <p class="lesson0-text">"First element"</p>
                    <p class="lesson0-text">"Second element"</p>
                }
            </div>
            <div class="lesson0-box">
                <h3 class="lesson0-subtitle">"Explicit (With &lt;&gt;):"</h3>
                @if show_content {
                    <>
                        <p class="lesson0-text">"First element"</p>
                        <p class="lesson0-text">"Second element"</p>
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

    html! {
        <style src="/static/pages/lesson0.css" />

        // <style>
        //     body {
        //         background-color: "#f5f5f5";
        //     }
        //     .lesson0-container {
        //         background-color: "#fff";
        //     }
        // </style>
        <div class="lesson0-container">
            <h1 class="lesson0-title">"When to Use Fragments"</h1>

            <section class="lesson0-section">
                <h3 class="lesson0-subtitle">"1. Avoiding Unnecessary Wrappers"</h3>
                <div class="lesson0-comparison">
                    <div class="lesson0-before">
                        <h4>"❌ Without Fragment (Extra div)"</h4>
                        <code class="lesson0-code">"&lt;div&gt;\n  &lt;h2&gt;Title&lt;/h2&gt;\n  &lt;p&gt;Text&lt;/p&gt;\n&lt;/div&gt;"</code>
                    </div>
                    <div class="lesson0-after">
                        <h4>"✅ With Fragment (Clean)"</h4>
                        <code class="lesson0-code">"&lt;&gt;\n  &lt;h2&gt;Title&lt;/h2&gt;\n  &lt;p&gt;Text&lt;/p&gt;\n&lt;/&gt;"</code>
                    </div>
                </div>
            </section>

            <section class="lesson0-section">
                <h3 class="lesson0-subtitle">"2. In Loops"</h3>
                <ul class="lesson0-list">
                    @for item in &items {
                        <>
                            <li class="lesson0-item">{item}</li>
                            <li class="lesson0-item-note">"(Note: " {item} ")"</li>
                        </>
                    }
                </ul>
            </section>

            <section class="lesson0-section">
                <h3 class="lesson0-subtitle">"3. Component Returns"</h3>
                <p class="lesson0-text">"Components can return multiple elements without wrappers using fragments!"</p>
            </section>
        </div>
    }
}

/// Main demonstration component
#[azumi::component]
pub fn lesson0() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson0.css" />
        <div class="lesson0-container">
            <header class="lesson0-header">
                <h1 class="lesson0-main-title">"Lesson 0: Fragments"</h1>
                <p class="lesson0-subtitle">"Multiple Elements Without Wrapper Divs"</p>
            </header>

            <section class="lesson0-key-points">
                <h2 class="lesson0-section-title">"Key Points"</h2>
                <ul class="lesson0-points-list">
                    <li class="lesson0-point">"✅ Automatic: Multiple elements work without explicit &lt;&gt;"</li>
                    <li class="lesson0-point">"✅ Explicit: Use &lt;&gt; &lt;/&gt; for semantic clarity"</li>
                    <li class="lesson0-point">"✅ Both generate identical output"</li>
                    <li class="lesson0-point">"✅ Works in control flow (@if, @for, @match)"</li>
                    <li class="lesson0-point">"✅ Avoids unnecessary DOM nesting"</li>
                </ul>
            </section>

            <section class="lesson0-examples">
                <div class="lesson0-example-card">
                    @automatic_fragment_example()
                </div>
                <div class="lesson0-example-card">
                    @explicit_fragment_example()
                </div>
                <div class="lesson0-example-card">
                    @control_flow_fragments()
                </div>
                <div class="lesson0-example-card">
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
