//! Lesson 19: layout_system.rs
//!
//! Building reusable layout patterns
use azumi::html;

/// Basic layout wrapper
pub fn basic_layout(title: &str, content: impl azumi::Component) -> impl azumi::Component {
    html! {
        <div class="basic-layout">
            <header class="layout-header">
                <h1>{title}</h1>
            </header>
            <main class="layout-content">
                {content}
            </main>
        </div>
    }
}

/// Application layout with navigation
pub fn app_layout(title: &str, content: impl azumi::Component) -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson19.css" />
        <div class="app-layout">
            <header class="app-header">
                <div class="header-container">
                    <h1 class="app-title">{title}</h1>
                    <nav class="app-nav">
                        <a href="/" class="nav-link">"Home"</a>
                        <a href="/about" class="nav-link">"About"</a>
                        <a href="/contact" class="nav-link">"Contact"</a>
                    </nav>
                </div>
            </header>

            <div class="app-content">
                <aside class="app-sidebar">
                    <h3>"Navigation"</h3>
                    <ul class="sidebar-menu">
                        <li><a href="/dashboard">"Dashboard"</a></li>
                        <li><a href="/profile">"Profile"</a></li>
                        <li><a href="/settings">"Settings"</a></li>
                    </ul>
                </aside>

                <main class="app-main">
                    {content}
                </main>
            </div>

            <footer class="app-footer">
                <p>"© 2024 Azumi Demo"</p>
            </footer>
        </div>
    }
}

/// Dashboard layout with widgets
pub fn dashboard_layout(content: impl azumi::Component) -> impl azumi::Component {
    html! {
        <div class="dashboard-layout">
            <div class="dashboard-header">
                <h2>"Dashboard"</h2>
                <div class="header-actions">
                    <button class="btn btn-primary">"New Item"</button>
                    <button class="btn btn-secondary">"Export"</button>
                </div>
            </div>

            <div class="dashboard-content">
                {content}
            </div>
        </div>
    }
}

/// Widget component for dashboards
pub struct WidgetProps {
    pub title: String,
    pub content: impl azumi::Component,
    pub width: usize, // 1-4 columns
}

pub fn widget(props: WidgetProps) -> impl azumi::Component {
    let width_class = match props.width {
        1 => "widget-1",
        2 => "widget-2",
        3 => "widget-3",
        4 => "widget-4",
        _ => "widget-1",
    };

    html! {
        <div class={format!("widget {}", width_class)}>
            <div class="widget-header">
                <h3>{props.title}</h3>
                <button class="widget-close">"×"</button>
            </div>
            <div class="widget-content">
                {props.content}
            </div>
        </div>
    }
}

/// Grid layout for widgets
pub fn widget_grid(widgets: Vec<WidgetProps>) -> impl azumi::Component {
    html! {
        <div class="widget-grid">
            @for widget in widgets {
                @widget(widget)
            }
        </div>
    }
}

/// Modal layout component
pub struct ModalProps {
    pub is_open: bool,
    pub title: String,
    pub content: impl azumi::Component,
    pub actions: impl azumi::Component,
}

pub fn modal_layout(props: ModalProps) -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson19.css" />
        @if props.is_open {
            <div class="modal-overlay">
                <div class="modal">
                    <div class="modal-header">
                        <h3>{props.title}</h3>
                        <button class="modal-close">"×"</button>
                    </div>
                    <div class="modal-body">
                        {props.content}
                    </div>
                    <div class="modal-footer">
                        {props.actions}
                    </div>
                </div>
            </div>
        }
    }
}

/// Two-column layout
pub fn two_column_layout(left: impl azumi::Component, right: impl azumi::Component) -> impl azumi::Component {
    html! {
        <div class="two-column-layout">
            <div class="column-left">
                {left}
            </div>
            <div class="column-right">
                {right}
            </div>
        </div>
    }
}

/// Card-based layout
pub fn card_layout(cards: Vec<impl azumi::Component>) -> impl azumi::Component {
    html! {
        <div class="card-layout">
            <div class="card-grid">
                @for card in cards {
                    <div class="card">
                        {card}
                    </div>
                }
            </div>
        </div>
    }
}

/// Layout composition demo
pub fn layout_composition_demo() -> impl azumi::Component {
    let widget_content_1 = html! {
        <div class="widget-metric">
            <div class="metric-value">"1,234"</div>
            <div class="metric-label">"Total Users"</div>
        </div>
    };

    let widget_content_2 = html! {
        <div class="widget-chart">
            <canvas id="layoutChart" width="200" height="100">
                "Chart data"
            </canvas>
        </div>
    };

    let widget_content_3 = html! {
        <div class="widget-list">
            <ul>
                <li>"Recent activity 1"</li>
                <li>"Recent activity 2"</li>
                <li>"Recent activity 3"</li>
            </ul>
        </div>
    };

    let modal_content = html! {
        <div>
            <p>"This is modal content passed as a component"</p>
            <p>"Layouts can be composed and reused!"</p>
        </div>
    };

    let modal_actions = html! {
        <div class="modal-actions">
            <button class="btn btn-secondary">"Cancel"</button>
            <button class="btn btn-primary">"Confirm"</button>
        </div>
    };

    let card_1 = html! {
        <h3>"Feature 1"</h3>
        <p>"First feature description"</p>
    };

    let card_2 = html! {
        <h3>"Feature 2"</h3>
        <p>"Second feature description"</p>
    };

    let card_3 = html! {
        <h3>"Feature 3"</h3>
        <p>"Third feature description"</p>
    };

    let cards = vec![card_1, card_2, card_3];

    let widgets = vec![
        WidgetProps {
            title: "User Statistics".to_string(),
            content: widget_content_1,
            width: 2,
        },
        WidgetProps {
            title: "Chart Data".to_string(),
            content: widget_content_2,
            width: 2,
        },
        WidgetProps {
            title: "Recent Activity".to_string(),
            content: widget_content_3,
            width: 4,
        },
    ];

    html! {
        <style src="/static/pages/lesson19.css" />
        <div class="layout-demo">
            <h1>"Layout System Patterns"</h1>
            
            <section class="demo-section">
                <h2>"Basic Layout"</h2>
                @basic_layout("Basic Layout Example", html! {
                    <p>"This is content in a basic layout"</p>
                    <p>"Simple header + content structure"</p>
                })
            </section>

            <section class="demo-section">
                <h2>"App Layout"</h2>
                @app_layout("App Layout Example", html! {
                    <div class="content-area">
                        <h3>"Main Content"</h3>
                        <p>"This demonstrates a full app layout with header, nav, sidebar, and footer"</p>
                        <p>"Perfect for building complete applications"</p>
                    </div>
                })
            </section>

            <section class="demo-section">
                <h2>"Dashboard with Widgets"</h2>
                @dashboard_layout(
                    @widget_grid(widgets)
                )
            </section>

            <section class="demo-section">
                <h2>"Two Column Layout"</h2>
                @two_column_layout(
                    html! {
                        <div class="left-content">
                            <h3>"Left Column"</h3>
                            <p>"Primary content goes here"</p>
                            <ul>
                                <li>"Main features"</li>
                                <li>"Primary actions"</li>
                                <li>"Important information"</li>
                            </ul>
                        </div>
                    },
                    html! {
                        <div class="right-content">
                            <h3>"Right Column"</h3>
                            <p>"Secondary content goes here"</p>
                            <ul>
                                <li>"Additional info"</li>
                                <li>"Related links"</li>
                                <li>"Supplementary data"</li>
                            </ul>
                        </div>
                    }
                )
            </section>

            <section class="demo-section">
                <h2>"Card Layout"</h2>
                @card_layout(cards)
            </section>

            <section class="demo-section">
                <h2>"Modal Layout"</h2>
                <button 
                    onclick="document.querySelector('.modal-overlay').style.display='block'"
                    class="btn btn-primary">
                    "Open Modal"
                </button>
                
                @modal_layout(ModalProps {
                    is_open: true,
                    title: "Layout Modal".to_string(),
                    content: modal_content,
                    actions: modal_actions,
                })
            </section>
        </div>
    }
}

/// Handler for Axum
pub async fn lesson19_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&layout_composition_demo()))
}