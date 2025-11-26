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
                            Widget::Table(name) => {
                                @let label = format!("Table: {}", name);
                                <div class="widget table">{label}</div>
                            },
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
pub fn admin_dashboard() -> impl azumi::Component {
    dashboard(true)
}

/// User dashboard example
pub fn user_dashboard() -> impl azumi::Component {
    dashboard(false)
}
