use axum::response::{Html, IntoResponse};
use azumi::{html, Component};

pub async fn advanced_components_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&advanced_components_demo()))
}

fn advanced_components_demo() -> impl Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Advanced Components - Azumi"</title>
                <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css" />
                <style src="/static/advanced_components.css" />
            </head>
            <body>
                <div class="container">
                    <header class="page-header">
                        <h1>"Advanced Component Patterns"</h1>
                        <p>"Demonstrating modals, forms with validation, and nested composition"</p>
                        <a href="/" class="back-link">"← Back to Home"</a>
                    </header>

                    <main class="content">
                        // Section 1: Form with Validation States
                        <section class="demo-section">
                            <h2>"1. Form Components with Validation"</h2>
                            <p class="section-desc">"Reusable input components with error states"</p>

                            @ValidationForm()
                        </section>

                        // Section 2: Modal/Dialog Component
                        <section class="demo-section">
                            <h2>"2. Modal Component"</h2>
                            <p class="section-desc">"Composable modal with different content"</p>

                            <button class="btn-primary" onclick="document.getElementById('modal1').style.display='flex'">"Open Modal"</button>

                            @Modal(
                                id="modal1",
                                title="Example Modal"
                            ) {
                                <p>"This is a reusable modal component."</p>
                                <p>"You can put any content inside using the children pattern."</p>
                            }
                        </section>

                        // Section 3: Nested Component Composition
                        <section class="demo-section">
                            <h2>"3. Nested Component Composition"</h2>
                            <p class="section-desc">"Complex nesting with props and children"</p>

                            @Card(title="User Dashboard") {
                                @StatCard(label="Total Users", value="1,234", icon="users", color="blue")
                                @StatCard(label="Active Sessions", value="89", icon="bolt", color="green")
                                @StatCard(label="Pending Tasks", value="42", icon="clipboard-list", color="orange")
                            }
                        </section>
                    </main>
                </div>
            </body>
        </html>
    }
}

// ========================================
// FORM COMPONENTS WITH VALIDATION
// ========================================

#[azumi::component]
fn ValidationForm() -> impl Component {
    html! {
        <form class="validation-form">
            @FormInput(
                label = "Email Address",
                name = "email",
                input_type = "email",
                placeholder = "you@example.com",
                error = "Please enter a valid email address"
            )

            @FormInput(
                label = "Password",
                name = "password",
                input_type = "password",
                placeholder = "••••••••",
                error = "Password must be at least 8 characters"
            )

            @FormInput(
                label = "Username",
                name = "username",
                input_type = "text",
                placeholder = "johndoe",
                error = ""
            )

            <button type="submit" class="btn-primary">"Submit"</button>
        </form>
    }
}

#[azumi::component]
fn FormInput(
    label: &'static str,
    name: &'static str,
    input_type: &'static str,
    placeholder: &'static str,
    #[prop(default = "\"\"")] error: &'static str,
) -> impl Component {
    let has_error = !error.is_empty();
    let input_class = if has_error {
        "form-input error"
    } else {
        "form-input"
    };

    html! {
        <div class="form-group">
            <label for={name} class="form-label">{label}</label>
            <input
                type={input_type}
                id={name}
                name={name}
                class={input_class}
                placeholder={placeholder}
                />
            @if has_error {
                <span class="error-message">
                    <i class="fas fa-exclamation-circle"></i>
                    " " {error}
                </span>
            } else {
                <span class="success-message">
                    <i class="fas fa-check-circle"></i>
                    " " "Valid input"
                </span>
            }
        </div>
    }
}

// ========================================
// MODAL COMPONENT
// ========================================

#[azumi::component]
fn Modal(id: &'static str, title: &'static str, children: impl Component) -> impl Component {
    html! {
        <div id={id} class="modal-overlay" onclick="this.style.display='none'">
            <div class="modal-content" onclick="event.stopPropagation()">
                <div class="modal-header">
                    <h3 class="modal-title">{title}</h3>
                    <button
                        class="modal-close"
                        onclick={format!("document.getElementById('{}').style.display='none'", id)}
                        >
                        "×"
                    </button>
                </div>
                <div class="modal-body">
                    @children
                </div>
                <div class="modal-footer">
                    <button class="btn-secondary" onclick={format!("document.getElementById('{}').style.display='none'", id)}>"Close"</button>
                    <button class="btn-primary">"Confirm"</button>
                </div>
            </div>
        </div>
    }
}

// ========================================
// NESTED COMPOSITION COMPONENTS
// ========================================

#[azumi::component]
fn Card(title: &'static str, children: impl Component) -> impl Component {
    html! {
        <div class="card">
            <div class="card-header">
                <h3>{title}</h3>
            </div>
            <div class="card-body">
                <div class="stats-grid">
                    @children
                </div>
            </div>
        </div>
    }
}

#[azumi::component]
fn StatCard(
    label: &'static str,
    value: &'static str,
    icon: &'static str,
    color: &'static str,
) -> impl Component {
    let icon_class = format!("fas fa-{}", icon);
    let stat_class = format!("stat-card {}", color);

    html! {
        <div class={stat_class.as_str()}>
            <div class="stat-icon">
                <i class={icon_class.as_str()}></i>
            </div>
            <div class="stat-content">
                <div class="stat-value">{value}</div>
                <div class="stat-label">{label}</div>
            </div>
        </div>
    }
}
