use axum::response::{Html, IntoResponse};
use azumi::html;

pub async fn forms_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&forms_demo()))
}

fn forms_demo() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>"Forms Demo"</title>
                <style src="/static/homepage.css" />
                <style src="/static/forms.css" />
            </head>
            <body>
                <div class="container">
                    <h1>"Forms Example"</h1>
                    <a href="/">"← Back to Home"</a>

                    <div class="card" style="max-width: 500px; margin: 20px auto;">
                        <form action="/forms" method="post">
                            <div class="form-group">
                                <label for="name">"Full Name"</label>
                                <input type="text" id="name" name="name" placeholder="John Doe" required />
                            </div>

                            <div class="form-group">
                                <label for="email">"Email Address"</label>
                                <input type="email" id="email" name="email" placeholder="john@example.com" required />
                            </div>

                            <div class="form-group">
                                <label for="type">"Inquiry Type"</label>
                                <select id="type" name="type">
                                    <option value="support">"Support"</option>
                                    <option value="sales">"Sales"</option>
                                    <option value="other">"Other"</option>
                                </select>
                            </div>

                            <div class="form-group">
                                <label for="message">"Message"</label>
                                <textarea id="message" name="message" rows="4"></textarea>
                            </div>

                            <button type="submit">"Send Message"</button>
                        </form>
                    </div>
                </div>
            </body>
        </html>
    }
}
