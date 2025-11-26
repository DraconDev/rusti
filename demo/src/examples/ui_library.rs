use axum::response::{Html, IntoResponse};
use azumi::{html, Component};

pub async fn ui_library_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&ui_library_demo()))
}

fn ui_library_demo() -> impl Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"UI Library - Azumi"</title>
                <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css" />
                <style src="/static/ui_library.css" />
            </head>
            <body>
                <div class="container">
                    <header class="page-header">
                        <h1>"Azumi UI Library"</h1>
                        <p>"Reusable component library demonstrating advanced patterns"</p>
                        <a href="/" class="back-link">"← Back to Home"</a>
                    </header>

                    <main class="content">
                        <section class="demo-section">
                            <h2>"Buttons"</h2>
                            <div class="button-showcase">
                                <div class="button-group">
                                    <p>"Primary Buttons:"</p>
                                    @Button(text="Primary", variant="primary")
                                    @Button(text="Large Primary", variant="primary", size="large")
                                    @Button(text="Disabled", variant="primary", disabled=true)
                                </div>
                                
                                <div class="button-group">
                                    <p>"Secondary Buttons:"</p>
                                    @Button(text="Secondary", variant="secondary")
                                    @Button(text="Large Secondary", variant="secondary", size="large")
                                    @Button(text="Secondary Outline", variant="secondary", style="outline")
                                </div>

                                <div class="button-group">
                                    <p>"Danger Buttons:"</p>
                                    @Button(text="Delete", variant="danger")
                                    @Button(text="Cancel", variant="secondary", style="outline")
                                </div>

                                <div class="button-group">
                                    <p>"Icon Buttons:"</p>
                                    @Button(icon="heart", text="Like", variant="primary")
                                    @Button(icon="trash", variant="danger")
                                    @Button(icon="settings", variant="secondary")
                                </div>
                            </div>
                        </section>

                        <section class="demo-section">
                            <h2>"Cards"</h2>
                            <div class="card-grid">
                                @Card(title="Simple Card") {
                                    <p>"This is a simple card component with children content."</p>
                                    <button class="btn-small">"Action"</button>
                                }
                                
                                @Card(
                                    title="Featured Card",
                                    variant="featured"
                                ) {
                                    <div class="card-metadata">
                                        <span class="card-date">"2024-01-15"</span>
                                        <span class="card-author">"John Doe"</span>
                                    </div>
                                    <p>"This is a featured card with custom styling and metadata."</p>
                                }

                                @Card(title="Card with Footer") {
                                    <div class="card-content">
                                        <p>"This card has a custom footer section."</p>
                                    </div>
                                    <div class="card-footer">
                                        <span class="card-views">"1.2k views"</span>
                                        @Button(text="Read More", variant="primary", size="small")
                                    </div>
                                }
                            </div>
                        </section>

                        <section class="demo-section">
                            <h2>"Status Indicators"</h2>
                            <div class="status-grid">
                                @StatusBadge(status="success", text="Active")
                                @StatusBadge(status="warning", text="Pending")
                                @StatusBadge(status="error", text="Failed")
                                @StatusBadge(status="info", text="In Progress")
                                @StatusBadge(status="neutral", text="Draft")
                            </div>
                        </section>

                        <section class="demo-section">
                            <h2>"Progress Indicators"</h2>
                            <div class="progress-grid">
                                <div class="progress-item">
                                    <label>"Loading Progress:"</label>
                                    @ProgressBar(progress=75, show_percentage=true)
                                </div>
                                
                                <div class="progress-item">
                                    <label>"Upload Progress:"</label>
                                    @ProgressBar(progress=45, variant="success", show_percentage=true)
                                </div>
                                
                                <div class="progress-item">
                                    <label>"Task Completion:"</label>
                                    @ProgressBar(progress=100, variant="success", show_percentage=true)
                                </div>
                            </div>
                        </section>

                        <section class="demo-section">
                            <h2>"Interactive Components"</h2>
                            <div class="interactive-grid">
                                @InteractiveToggle(initial_state=false, label="Dark Mode")
                                @InteractiveToggle(initial_state=true, label="Notifications")
                                @InteractiveToggle(initial_state=false, label="Analytics")
                            </div>
                        </section>
                    </main>
                </div>
            </body>
        </html>
    }
}

// ========================================
// BUTTON COMPONENT
// ========================================

#[azumi::component]
fn Button(
    #[prop(default = "\"\"")] text: &'static str,
    #[prop(default = "\"primary\"")] variant: &'static str,
    #[prop(default = "\"medium\"")] size: &'static str,
    #[prop(default = "\"solid\"")] style: &'static str,
    #[prop(default = "\"\"")] icon: &'static str,
    #[prop(default = "false")] disabled: bool,
) -> impl Component {
    let button_class = format!("button {} {} {}", variant, size, style);
    
    html! {
        <button 
            class={button_class}
            disabled={disabled}
            >
            @if !icon.is_empty() {
                <i class={format!("fas fa-{}", icon)}></i>
                @if !text.is_empty() { " " }
            }
            @if !text.is_empty() { {text} }
        </button>
    }
}

// ========================================
// CARD COMPONENT
// ========================================

#[azumi::component]
fn Card(
    title: &'static str,
    #[prop(default = "\"default\"")] variant: &'static str,
    children: impl Component,
) -> impl Component {
    let card_class = format!("card {}", variant);
    
    html! {
        <div class={card_class}>
            <div class="card-header">
                <h3>{title}</h3>
            </div>
            <div class="card-body">
                {children}
            </div>
        </div>
    }
}

// ========================================
// STATUS BADGE COMPONENT
// ========================================

#[azumi::component]
fn StatusBadge(status: &'static str, text: &'static str) -> impl Component {
    let badge_class = format!("status-badge {}", status);
    
    html! {
        <span class={badge_class}>
            <span class="status-dot"></span>
            {text}
        </span>
    }
}

// ========================================
// PROGRESS BAR COMPONENT
// ========================================

#[azumi::component]
fn ProgressBar(
    progress: u8,
    #[prop(default = "\"primary\"")] variant: &'static str,
    #[prop(default = "false")] show_percentage: bool,
) -> impl Component {
    let clamped_progress = progress.min(100);
    let progress_class = format!("progress-bar {}", variant);
    let progress_width = format!("width: {}%", clamped_progress);
    let progress_text = if show_percentage {
        format!("{}%", clamped_progress)
    } else {
        "".to_string()
    };
    
    html! {
        <div class="progress-container">
            <div class={progress_class}>
                <div
                    class="progress-fill"
                    style={progress_width.as_str()}
                    ></div>
            </div>
            @if show_percentage {
                <span class="progress-text">{progress_text.as_str()}</span>
            }
        </div>
    }
}

// ========================================
// INTERACTIVE TOGGLE COMPONENT
// ========================================

#[azumi::component]
fn InteractiveToggle(
    initial_state: bool,
    label: &'static str,
) -> impl Component {
    let toggle_id = format!("toggle-{}", label.replace(" ", "-").to_lowercase());
    let checked_class = if initial_state { "checked" } else { "" };
    
    html! {
        <div class="toggle-item">
            <label for={toggle_id} class="toggle-label">{label}</label>
            <div class="toggle-wrapper">
                <input 
                    type="checkbox" 
                    id={toggle_id}
                    class="toggle-input"
                    checked={initial_state}
                    />
                <label for={toggle_id} class="toggle-slider {checked_class}">
                    <span class="toggle-button"></span>
                </label>
            </div>
        </div>
    }
}