use axum::response::{Html, IntoResponse};
use azumi::html;

/// Advanced Components - Forms, modals, validation, and complex nesting
pub fn advanced_components() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Advanced Components - Azumi"</title>
                <style src="/static/advanced_components.css" />
                <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css" />
            </head>
            <body>
                <div class="page-container">
                    <header class="page-header">
                        <h1>"🧩 Advanced Components"</h1>
                        <p>"Forms, modals, validation, and complex component patterns"</p>
                        <a href="/" class="back-link">"← Back to Home"</a>
                    </header>

                    <main class="examples-container">
                        <!-- Form Component -->
                        <section class="component-section">
                            <h2>"📝 Contact Form with Validation"</h2>
                            <p>"A complete form component with client-side validation"</p>
                            
                            <div class="form-demo">
                                <form class="contact-form">
                                    <div class="form-group">
                                        <label for="name">"Name" <span class="required">"*"</span></label>
                                        <input type="text" id="name" name="name" placeholder="Enter your name" required />
                                        <span class="error-message" id="name-error">"Name is required"</span>
                                    </div>
                                    
                                    <div class="form-group">
                                        <label for="email">"Email" <span class="required">"*"</span></label>
                                        <input type="email" id="email" name="email" placeholder="your@email.com" required />
                                        <span class="error-message" id="email-error">"Valid email required"</span>
                                    </div>
                                    
                                    <div class="form-group">
                                        <label for="message">"Message"</label>
                                        <textarea id="message" name="message" placeholder="Your message here..."></textarea>
                                    </div>
                                    
                                    <button type="submit" class="submit-btn">
                                        <i class="fas fa-paper-plane"></i>
                                        "Send Message"
                                    </button>
                                </form>
                            </div>
                        </section>

                        <!-- Modal Component -->
                        <section class="component-section">
                            <h2>"🪟 Interactive Modal"</h2>
                            <p>"A reusable modal component with different states"</p>
                            
                            <div class="modal-demo">
                                <button class="demo-btn primary" onclick="openModal('info')">
                                    <i class="fas fa-info-circle"></i>
                                    "Info Modal"
                                </button>
                                <button class="demo-btn warning" onclick="openModal('confirm')">
                                    <i class="fas fa-question-circle"></i>
                                    "Confirm Modal"
                                </button>
                                <button class="demo-btn danger" onclick="openModal('error')">
                                    <i class="fas fa-exclamation-triangle"></i>
                                    "Error Modal"
                                </button>
                            </div>
                        </section>

                        <!-- Complex Card Component -->
                        <section class="component-section">
                            <h2>"🎴 Complex Card Layout"</h2>
                            <p>"Multi-level component composition with dynamic content"</p>
                            
                            <div class="cards-grid">
                                <div class="advanced-card featured">
                                    <div class="card-header">
                                        <div class="card-avatar">"👤"</div>
                                        <div class="card-meta">
                                            <h3>"Alice Johnson"</h3>
                                            <span class="user-role">"Senior Developer"</span>
                                        </div>
                                        <div class="card-actions">
                                            <button class="icon-btn" title="Message">
                                                <i class="fas fa-envelope"></i>
                                            </button>
                                            <button class="icon-btn" title="Follow">
                                                <i class="fas fa-user-plus"></i>
                                            </button>
                                        </div>
                                    </div>
                                    
                                    <div class="card-body">
                                        <p>"Passionate about Rust and web development. Love building scalable applications."</p>
                                        
                                        <div class="skill-tags">
                                            <span class="skill-tag">"Rust"</span>
                                            <span class="skill-tag">"TypeScript"</span>
                                            <span class="skill-tag">"WebAssembly"</span>
                                        </div>
                                    </div>
                                    
                                    <div class="card-stats">
                                        <div class="stat">
                                            <span class="stat-number">"127"</span>
                                            <span class="stat-label">"Projects"</span>
                                        </div>
                                        <div class="stat">
                                            <span class="stat-number">"2.5k"</span>
                                            <span class="stat-label">"Followers"</span>
                                        </div>
                                        <div class="stat">
                                            <span class="stat-number">"⭐ 4.9"</span>
                                            <span class="stat-label">"Rating"</span>
                                        </div>
                                    </div>
                                </div>

                                <div class="advanced-card">
                                    <div class="card-header">
                                        <div class="card-avatar">"🚀"</div>
                                        <div class="card-meta">
                                            <h3>"Project Launch"</h3>
                                            <span class="project-status">"In Progress"</span>
                                        </div>
                                        <div class="card-actions">
                                            <button class="icon-btn" title="Edit">
                                                <i class="fas fa-edit"></i>
                                            </button>
                                        </div>
                                    </div>
                                    
                                    <div class="card-body">
                                        <p>"New Azumi component library with advanced features and TypeScript support."</p>
                                        
                                        <div class="progress-bar">
                                            <div class="progress-fill" style="width: 75%"></div>
                                            <span class="progress-text">"75% Complete"</span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </section>

                        <!-- Component Code Example -->
                        <section class="code-section">
                            <h2>"💻 Component Code Example"</h2>
                            <p>"This is how you build reusable components with props"</p>
                            
                            <div class="code-example">
                                <div class="code-header">
                                    <span class="file-name">"user_card.rs"</span>
                                    <button class="copy-btn" onclick="copyCode(this)">
                                        <i class="fas fa-copy"></i>
                                        "Copy"
                                    </button>
                                </div>
                                <pre class="code-block"><code>{"#[azumi::component]\nfn UserCard(\n    name: &str,\n    role: &str,\n    avatar: &str,\n    #[prop(default = \"false\")] featured: bool,\n) -> impl azumi::Component {\n    html! {\n        <style src=\"/static/user_card.css\" />\n        <div class={format!(\"user-card {}\", if featured { \"featured\" } else { \"\" })}>\n            <div class=\"card-avatar\">{avatar}</div>\n            <h3>{name}</h3>\n            <span class=\"user-role\">{role}</span>\n        </div>\n    }\n}\n\n// Usage:\n@UserCard(\n    name=\"Alice\",\n    role=\"Developer\", \n    avatar=\"👩‍💻\",\n    featured=true\n)"}</code></pre>
                            </div>
                        </section>
                    </main>
                </div>

                <!-- Modal Overlay -->
                <div id="modal-overlay" class="modal-overlay" onclick="closeModal(event)">
                    <div class="modal" onclick="event.stopPropagation()">
                        <div class="modal-header">
                            <h3 id="modal-title">"Modal Title"</h3>
                            <button class="modal-close" onclick="closeModal()">
                                <i class="fas fa-times"></i>
                            </button>
                        </div>
                        <div class="modal-body">
                            <p id="modal-content">"Modal content goes here"</p>
                        </div>
                        <div class="modal-footer">
                            <button class="btn secondary" onclick="closeModal()">"Cancel"</button>
                            <button class="btn primary" onclick="closeModal()">"Confirm"</button>
                        </div>
                    </div>
                </div>

                <script>
                    <script type="application/json">
                        {r#"
                        {
                            "modals": {
                                "info": {
                                    "title": "Information",
                                    "content": "This is an informational modal dialog."
                                },
                                "confirm": {
                                    "title": "Confirm Action", 
                                    "content": "Are you sure you want to perform this action?"
                                },
                                "error": {
                                    "title": "Error",
                                    "content": "Something went wrong. Please try again."
                                }
                            }
                        }
                        "#}
                    </script>
                </script>
            </body>
        </html>
    }
}

pub async fn advanced_components_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&advanced_components()))
}
