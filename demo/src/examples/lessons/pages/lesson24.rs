//! Lesson 24: Boolean Attributes
//!
//! Using dynamic boolean attributes in forms

use azumi::html;

#[azumi::component]
pub fn form_states_demo() -> impl azumi::Component {
    let is_admin = true;
    let email_verified = false;
    let subscribe_checked = true;

    html! {
        <style src="/static/pages/lesson24.css" />
        <div class="container">
            <h1>"Lesson 24: Boolean Attributes"</h1>

            <form class="demo-form">
                <h2>"Dynamic States"</h2>

                // Disabled based on Rust variable
                <div class="field">
                    <label for="username">"Username"</label>
                    <input
                        type="text"
                        id="username"
                        value="admin"
                        disabled={!is_admin}  // Dynamic!
                    />
                </div>

                // Readonly from variable
                <div class="field">
                    <label for="email">"Email"</label>
                    <input
                        type="email"
                        id="email"
                        value="user@example.com"
                        readonly={email_verified}  // Dynamic!
                    />
                </div>

                // Checked from variable
                <div class="field">
                    <label>
                        <input
                            type="checkbox"
                            checked={subscribe_checked}  // Dynamic!
                        />
                        " Newsletter subscription"
                    </label>
                </div>

                <h2>"Static Booleans"</h2>

                <div class="field">
                    <label for="bio">"Bio"</label>
                    <textarea
                        id="bio"
                        rows="3"
                        disabled=true  // Always disabled
                    >"Read only content"</textarea>
                </div>

                <select>
                    <option>"Choose"</option>
                    <option selected=true>"Default"</option>
                    <option disabled=true>"Unavailable"</option>
                </select>

                <div class="buttons">
                    <button type="submit" disabled=false>"Submit"</button>
                    <button type="button" disabled=true>"Disabled"</button>
                </div>
            </form>

            <div class="state-display">
                <h3>"Current state:"</h3>
                <ul>
                    <li>"is_admin = " {is_admin.to_string()}</li>
                    <li>"email_verified = " {email_verified.to_string()}</li>
                    <li>"subscribe_checked = " {subscribe_checked.to_string()}</li>
                </ul>
            </div>
        </div>
    }
}

pub async fn lesson24_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @form_states_demo() }))
}
