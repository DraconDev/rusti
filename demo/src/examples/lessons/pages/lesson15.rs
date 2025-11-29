//! Lesson 15: Reusable Inputs
//!
//! Creating a reusable input component

use azumi::html;

#[azumi::component]
pub fn input_field<'a>(
    label: &'a str,
    name: &'a str,
    placeholder: &'a str,
) -> impl azumi::Component + 'a {
    html! {
        <style src="/static/pages/lesson15.css" />
        <div class="field">
            <label for={name}>{label}</label>
            <input
                type="text"
                id={name}
                name={name}
                placeholder={placeholder}
            />
        </div>
    }
}

#[azumi::component]
pub fn form_demo() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson15.css" />
        <div class="container">
            <h1>"Lesson 15: Reusable Inputs"</h1>

            <form class="simple-form">
                @input_field(
                    label="Username",
                    name="username",
                    placeholder="Enter username"
                )

                @input_field(
                    label="Email Address",
                    name="email",
                    placeholder="john@example.com"
                )

                <button type="submit">"Sign Up"</button>
            </form>
        </div>
    }
}

pub async fn lesson15_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @form_demo() }))
}
