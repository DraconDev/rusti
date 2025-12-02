use azumi::html;

/// Lesson 7: Props and Parameters
///
/// Shows how to pass props to components
#[azumi::component]
pub fn greeting_component<'a>(name: &'a str) -> impl azumi::Component + 'a {
    html! {
        <style>
            .greeting { padding: "15px"; background: "#e3f2fd"; border: "1px solid #90caf9"; }
            .name { color: "#1976d2"; font-weight: "bold"; }
        </style>
        <div class={greeting}>
            <p>"Hello, " <span class={name}>{name}</span> "!"</p>
        </div>
    }
}

/// Example: Component with multiple parameters
#[azumi::component]
pub fn user_card<'a>(name: &'a str, age: i32, is_active: bool) -> impl azumi::Component + 'a {
    html! {
        <style>
            .card { padding: "20px"; border: "1px solid #ddd"; margin: "10px 0"; }
            .active { background: "#e8f5e9"; }
            .inactive { background: "#ffebee"; }
        </style>
        <div class={if is_active { "card active" } else { "card inactive" }}>
            <h3>{name}</h3>
            <p>"Age: " {age}</p>
            <p>"Status: " {if is_active { "Active" } else { "Inactive" }}</p>
        </div>
    }
}

/// Example: Reusable button component
#[azumi::component]
pub fn custom_button<'a>(text: &'a str, color: &'a str) -> impl azumi::Component + 'a {
    html! {
        <style>
            .btn { padding: "10px 20px"; border: "none"; border-radius: "4px"; cursor: "pointer"; }
            .color-2196f3 { background-color: "#2196f3"; }
            .color-4caf50 { background-color: "#4caf50"; }
            .color-f44336 { background-color: "#f44336"; }
        </style>
        <button class={btn} class={format!("color-{}", color.replace("#", ""))}>
            {text}
        </button>
    }
}

/// Main lesson component
#[azumi::component]
pub fn lesson7() -> impl azumi::Component {
    html! {
        <style>
            .lesson_container { padding: "20px"; }
            .lesson_title { font-size: "24px"; color: "#333"; }
            .demo_section { margin: "20px 0"; }
        </style>
        <div class={lesson_container}>
            <h1 class={lesson_title}>"Lesson 7: Props and Parameters"</h1>
            <p>"Learn how to create reusable components with parameters"</p>

            <div class={demo_section}>
                <h3>"Greeting Component"</h3>
                @greeting_component(name="Alice")
                @greeting_component(name="Bob")
            </div>

            <div class={demo_section}>
                <h3>"User Cards"</h3>
                @user_card(name="John Doe", age=30, is_active=true)
                @user_card(name="Jane Smith", age=25, is_active=false)
            </div>

            <div class={demo_section}>
                <h3>"Custom Buttons"</h3>
                @custom_button(text="Primary", color="#2196f3")
                @custom_button(text="Success", color="#4caf50")
                @custom_button(text="Danger", color="#f44336")
            </div>
        </div>
    }
}

// Handler for Axum
pub async fn lesson7_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson7()))
}
