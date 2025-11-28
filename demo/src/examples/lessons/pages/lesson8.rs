//! Lesson 8: nested_control_flow.rs
//!
//! Combining @if, @for, @match
use azumi::html;

#[derive(Clone, Copy)]
enum Widget<'a> {
    Chart(&'a str),
    Table(&'a str),
}

/// Dashboard with nested control flow
#[azumi::component]
pub fn dashboard(is_admin: bool) -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson8.css" />
        <div>
            @if is_admin {
                <h2>"Admin Dashboard"</h2>
                <div class="widgets">
                    @for widget in &[
                        Widget::Chart("sales-data"),
                        Widget::Table("user-list"),
                    ] {
                        @match widget {
                            Widget::Chart(name) => {
                                @let label = format!("Chart: {}", name);
                                <div class="widget chart">{label}</div>
                            },
                            Widget::Table(name) => <div class="widget table">{format!("Table: {}", name)}</div>
                        }
                    }
                </div>
            } else {
                <h2>"User Dashboard"</h2>
            }
        </div>
    }
}

/// Admin dashboard example
#[azumi::component]
pub fn admin_dashboard() -> impl azumi::Component {
    html! {
        @dashboard(is_admin=true)
    }
}

/// User dashboard example
#[azumi::component]
pub fn user_dashboard() -> impl azumi::Component {
    html! {
        @dashboard(is_admin=false)
    }
}
pub async fn lesson8_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&admin_dashboard()))
}
