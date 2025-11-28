//! Lesson 22: Data Tables
//!
//! Building accessible, type-safe tables

use azumi::html;

#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
    role: String,
    active: bool,
}

#[azumi::component]
pub fn data_table_demo() -> impl azumi::Component {
    let users = vec![
        User { id: 1, name: "Alice Johnson".to_string(), email: "alice@example.com".to_string(), role: "Admin".to_string(), active: true },
        User { id: 2, name: "Bob Smith".to_string(), email: "bob@example.com".to_string(), role: "User".to_string(), active: true },
        User { id: 3, name: "Carol White".to_string(), email: "carol@example.com".to_string(), role: "User".to_string(), active: false },
    ];
    
    html! {
        <style src="/static/pages/lesson22.css" />
        <div class="container">
            <h1>"Lesson 22: Data Tables"</h1>
            <p class="description">"Type-safe tables with proper HTML structure"</p>

            <table class="data-table">
                <thead>
                    <tr>
                        <th>"ID"</th>
                        <th>"Name"</th>
                        <th>"Email"</th>
                        <th>"Role"</th>
                        <th>"Status"</th>
                    </tr>
                </thead>
                <tbody>
                    @for user in &users {
                        <tr class={if user.active { "active" } else { "inactive" }}>
                            <td>{user.id}</td>
                            <td class="name">{&user.name}</td>
                            <td class="email">{&user.email}</td>
                            <td>
                                <span class={format!("badge badge-{}", user.role.to_lowercase())}>
                                    {&user.role}
                                </span>
                            </td>
                            <td>
                                <span class={format!("status-{}", if user.active { "active" } else { "inactive" })}>
                                    {if user.active { "●" } else { "○" }}
                                    " "
                                    {if user.active { "Active" } else { "Inactive" }}
                                </span>
                            </td>
                        </tr>
                    }
                </tbody>
            </table>

            <div class="note">
                <strong>"Note:"</strong>
                " Azumi validates table structure at compile time. "
                "<code>"<tr>"</code> " must be inside " <code>"<thead>"</code> ", "
                <code>"<tbody>"</code> ", or " <code>"<tfoot>"</code> "!"
            </div>
        </div>
    }
}

pub async fn lesson22_handler() -> impl axum::response::IntoResponse {
    axumi::response::Html(azumi::render_to_string(&html! { @data_table_demo() }))
}
