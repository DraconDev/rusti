//! Lesson 31: Type-Safe Forms (Form Binding)
//!
//! Demonstrating compile-time validation of form input names against Rust structs.

use azumi::html;
use serde::Deserialize;

/// Define the form data structure
#[derive(Deserialize)]
#[allow(dead_code)] // Fields are used in macro validation
pub struct UserRegistration {
    pub username: String,
    pub email: String,
    pub password: String,
    pub age: Option<u32>,
    pub role: String,
}

#[azumi::component]
pub fn form_binding_demo() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson31.css" />
        <div class="container">
            <h1>"Lesson 31: Type-Safe Forms"</h1>

            <div class="info-box">
                <p>
                    "This form uses " <code>"bind={UserRegistration}"</code> ". "
                    "Azumi validates at compile-time that every " <code>"name"</code>
                    " attribute matches a field in the struct."
                </p>
                <p>
                    "Try changing a name to something invalid (e.g., 'usrname') and see the compile error!"
                </p>
            </div>

            <div class="form-card">
                // The bind attribute connects the form to the struct
                <form bind={UserRegistration} method="POST" action="/api/register">
                    <div class="form-group">
                        <label for="username">"Username"</label>
                        <input type="text" id="username" name="userame" required />
                        <span class="hint">"Must match field: username"</span>
                    </div>

                    <div class="form-group">
                        <label for="email">"Email Address"</label>
                        <input type="email" id="email" name="email" required />
                    </div>

                    <div class="form-group">
                        <label for="password">"Password"</label>
                        <input type="password" id="password" name="password" required />
                    </div>

                    <div class="form-group">
                        <label for="age">"Age (Optional)"</label>
                        <input type="number" id="age" name="age" />
                    </div>

                    <div class="form-group">
                        <label for="role">"Role"</label>
                        <select id="role" name="role">
                            <option value="user">"Standard User"</option>
                            <option value="admin">"Administrator"</option>
                        </select>
                    </div>

                    <button type="submit">"Register Account"</button>
                </form>
            </div>
        </div>
    }
}

pub async fn lesson31_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @form_binding_demo() }))
}
