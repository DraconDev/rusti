//! Lesson 29: Advanced Composition (Slots)
//!
//! Simulating slots/render props by passing components as arguments

use azumi::html;

// A "Modal" component that takes header and body content
// We use generics to accept any component type for slots
#[azumi::component]
pub fn modal<H, B>(header: H, body: B, is_open: bool) -> impl azumi::Component
where
    H: azumi::Component,
    B: azumi::Component,
{
    html! {
        <style src="/static/pages/lesson29.css" />
        <div class={format!("modal-overlay {}", if is_open { "open" } else { "" })}>
            <div class="modal-content">
                <div class="modal-header">
                    @header
                    <button class="close-btn">"×"</button>
                </div>
                <div class="modal-body">
                    @body
                </div>
            </div>
        </div>
    }
}

#[azumi::component]
pub fn slots_demo() -> impl azumi::Component {
    let show_modal = true;

    // Define content for "slots"
    let header_content = html! { <h3>"Confirmation Required"</h3> };

    let body_content = html! {
        <style src="/static/pages/lesson29.css" />
        <p>"Are you sure you want to delete this item?"</p>
        <div class="actions">
            <button class="btn cancel">"Cancel"</button>
            <button class="btn confirm">"Delete"</button>
        </div>
    };

    html! {
        <style src="/static/pages/lesson29.css" />
        <div class="container">
            <h1>"Lesson 29: Slots Pattern"</h1>
            <p>"Pass components as props to create flexible layouts (like slots)"</p>

            <button class="btn">"Open Modal"</button>

            // Pass content fragments as props!
            @modal(
                header=header_content,
                body=body_content,
                is_open=show_modal
            )

            <div class="code-example">
                <h3>"How it works:"</h3>
                <pre>
    "// Define content fragments\n"
    "let header = html! { <h3>\"Title\"</h3> };\n"
    "let body = html! { <p>\"Content\"</p> };\n\n"
    "// Pass as props\n"
    "@modal(header=header, body=body, ...)"
                </pre>
            </div>
        </div>
    }
}

pub async fn lesson29_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @slots_demo() }))
}
