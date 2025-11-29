//! Lesson 33: Strict Validation Rules
//!
//! Demonstrating Azumi's strict HTML structure validation.

use azumi::html;

#[azumi::component]
pub fn validation_rules_demo() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson33.css" />
        <div class="container">
            <h1>"Lesson 33: Strict Validation"</h1>

            <div class="intro">
                <p>
                    "Azumi enforces strict HTML rules at compile time. "
                    "This prevents invalid nesting that would break layout or accessibility."
                </p>
            </div>

            <div class="rule-card">
                <h2>"Rule 1: No Nested Forms"</h2>
                <div class="example">
                    <div class="bad">"&lt;form&gt;&lt;form&gt;...&lt;/form&gt;&lt;/form&gt;"</div>
                    <div class="reason">"Browsers will delete nested forms, breaking submit logic."</div>
                </div>
                // Uncomment to see error:
                // <form><form></form></form>
            </div>

            <div class="rule-card">
                <h2>"Rule 2: Buttons cannot contain interactive elements"</h2>
                <div class="example">
                    <div class="bad">"&lt;button&gt;&lt;a href='...'&gt;Link&lt;/a&gt;&lt;/button&gt;"</div>
                    <div class="reason">"Creates undefined click behavior."</div>
                </div>
                // Uncomment to see error:
                // <button><a href="#">"Link"</a></button>
            </div>

            <div class="rule-card">
                <h2>"Rule 3: Lists can only contain &lt;li&gt;"</h2>
                <div class="example">
                    <div class="bad">"&lt;ul&gt;&lt;div&gt;Item&lt;/div&gt;&lt;/ul&gt;"</div>
                    <div class="reason">"Breaks accessibility - screen readers report 0 items."</div>
                </div>
                // Uncomment to see error:
                // <ul><div>"Item"</div></ul>
            </div>

            <div class="rule-card">
                <h2>"Rule 4: Paragraphs cannot contain block elements"</h2>
                <div class="example">
                    <div class="bad">"&lt;p&gt;&lt;div&gt;Block&lt;/div&gt;&lt;/p&gt;"</div>
                    <div class="reason">"Browsers will automatically close the &lt;p&gt; tag before the block element."</div>
                </div>
                // Uncomment to see error:
                // <p><div>"Block"</div></p>
            </div>

            <div class="rule-card">
                <h2>"Rule 5: No Inline Styles/Scripts"</h2>
                <div class="example">
                    <div class="bad">"&lt;style&gt;...&lt;/style&gt;"</div>
                    <div class="reason">"Must use &lt;style src=\"...\" /&gt; for automatic scoping and hot-reloading."</div>
                </div>
                // Uncomment to see error:
                // <style>.foo { color: red; }</style>
            </div>

            <div class="rule-card">
                <h2>"Rule 6: No Local Link Tags"</h2>
                <div class="example">
                    <div class="bad">"&lt;link rel=\"stylesheet\" href=\"/local.css\" /&gt;"</div>
                    <div class="reason">"Must use &lt;style src=\"...\" /&gt; to ensure component encapsulation."</div>
                </div>
                // Uncomment to see error:
                // <link rel="stylesheet" href="/static/style.css" />
            </div>

            <div class="rule-card valid">
                <h2>"Valid Structure Example"</h2>
                <div class="example">
                    <div class="good">
                        "&lt;ul&gt;"<br/>
                        "  &lt;li&gt;Item 1&lt;/li&gt;"<br/>
                        "  &lt;li&gt;Item 2&lt;/li&gt;"<br/>
                        "&lt;/ul&gt;"
                    </div>
                </div>
                <ul>
                    <li>"Valid Item 1"</li>
                    <li>"Valid Item 2"</li>
                </ul>
            </div>
        </div>
    }
}

pub async fn lesson33_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @validation_rules_demo() }))
}
