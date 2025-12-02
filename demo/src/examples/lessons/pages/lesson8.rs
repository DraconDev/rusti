use azumi::html;

/// Lesson 8: Conditional Rendering
///
/// Demonstrates @if and @else control flow
#[azumi::component]
pub fn conditional_example() -> impl azumi::Component {
    let is_logged_in = true;
    let user_name = "John";

    html! {
        <style>
            .auth_container { padding: "20px"; border: "1px solid #ddd"; }
            .welcome { color: "green"; }
            .login_prompt { color: "blue"; }
        </style>
        <div class={auth_container}>
            <h3>"Authentication Status"</h3>
            @if is_logged_in {
                <p class={welcome}>"Welcome back, " {user_name} "!"</p>
            }
            @if !is_logged_in {
                <p class={login_prompt}>"Please log in to continue"</p>
            }
        </div>
    }
}

/// Example: Multiple conditions
#[azumi::component]
pub fn multi_condition() -> impl azumi::Component {
    let user_role = "admin";
    let has_permission = true;

    html! {
        <style>
            .permissions { padding: "15px"; background: "#f5f5f5"; }
            .granted { color: "green"; }
            .denied { color: "red"; }
        </style>
        <div class={permissions}>
            <h3>"Permission Check"</h3>
            @if user_role == "admin" && has_permission {
                <p class={granted}>"Full access granted"</p>
            }
            @if user_role == "user" {
                <p>"Limited access"</p>
            }
            @if user_role != "admin" && user_role != "user" {
                <p class={denied}>"Access denied"</p>
            }
        </div>
    }
}

/// Example: Conditional attributes
#[azumi::component]
pub fn conditional_attributes() -> impl azumi::Component {
    let is_disabled = false;
    let button_text = "Submit";

    html! {
        <style>
            .button_container { padding: "20px"; }
            .btn { padding: "10px 20px"; background: "#2196f3"; color: "white"; border: "none"; }
            .disabled { opacity: "0.5"; cursor: "not-allowed"; }
        </style>
        <div class={button_container}>
            <button
                class={if is_disabled { "btn disabled" } else { "btn" }}
                disabled={is_disabled}
            >
                {button_text}
            </button>
        </div>
    }
}

/// Main lesson component
#[azumi::component]
pub fn lesson8() -> impl azumi::Component {
    html! {
        <style>
            .lesson_container { padding: "20px"; }
            .lesson_title { font-size: "24px"; color: "#333"; }
            .examples { display: "grid"; gap: "20px"; margin-top: "20px"; }
        </style>
        <div class={lesson_container}>
            <h1 class={lesson_title}>"Lesson 8: Conditional Rendering"</h1>
            <p>"Learn how to conditionally render content in Azumi"</p>

            <div class={examples}>
                @conditional_example()
                @multi_condition()
                @conditional_attributes()
            </div>
        </div>
    }
}

// Handler for Axum
pub async fn lesson8_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson8()))
}
