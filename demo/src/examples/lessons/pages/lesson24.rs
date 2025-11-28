//! Lesson 24: Boolean Attributes
//!
//! HTML boolean attributes without values

use azumi::html;

#[azumi::component]
pub fn boolean_attributes_demo() -> impl azumi::Component {
    let is_readonly = true;
    let is_checked = false;

    html! {
        <style src="/static/pages/lesson24.css" />
        <div class="container">
            <h1>"Lesson 24: Boolean Attributes"</h1>
            <p class="description">
                "Boolean attributes don't need values - just their presence = true"
            </p>

            <h2>"Form Inputs"</h2>
            <div class="form-group">
                <label for="username">"Username"</label>
                <input
                    type="text"
                    id="username"
                    required=true
                    disabled=false
                    value="admin"
                />
            </div>

            <div class="form-group">
                <label for="email">"Email (readonly)"</label>
                <input
                    type="email"
                    id="email"
                    value="user@example.com"
                    readonly={is_readonly}
                />
            </div>

            <div class="form-group">
                <label>
                    <input
                        type="checkbox"
                        checked={is_checked}
                    />
                    " Subscribe to newsletter"
                </label>
            </div>

            <div class="form-group">
                <label for="bio">"Bio"</label>
                <textarea
                    id="bio"
                    rows="3"
                    disabled=true
                >"This textarea is disabled"</textarea>
            </div>

            <h2>"Buttons"</h2>
            <button disabled=false>"Enabled Button"</button>
            <button disabled=true>"Disabled Button"</button>

            <h2>"Select"</h2>
            <select>
                <option>"Option 1"</option>
                <option selected=true>"Option 2 (selected)"</option>
                <option disabled=true>"Option 3 (disabled)"</option>
            </select>

            <div class="note">
                <strong>"Common boolean attributes:"</strong>
                <code>"disabled"</code> ", "
                <code>"readonly"</code> ", "
                <code>"required"</code> ", "
                <code>"checked"</code> ", "
                <code>"selected"</code> ", "
                <code>"autofocus"</code> ", "
                <code>"multiple"</code>
            </div>
        </div>
    }
}

pub async fn lesson24_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(
        &html! { @boolean_attributes_demo() },
    ))
}
