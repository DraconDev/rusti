use azumi::html;

/// Lesson 12: Database Integration Patterns
///
/// Database-connected components
#[azumi::component]
pub fn database_integration_example() -> impl azumi::Component {
    html! {
        <style>
            .database_container { padding: "1.5rem"; background: "#f9f9f9"; }
            .database_info { margin: "0.5rem 0"; padding: "0.5rem"; background: "white"; border: "1px solid #eee"; }
            .query_result { font-family: "monospace"; background: "#f0f0f0"; padding: "0.5rem"; margin: "0.5rem 0"; }
        </style>
        <div class={database_container}>
            <h3>"Database Integration"</h3>

            <div class={database_info}>
                <p>"Azumi supports database integration patterns"</p>
                <p>"Example: SQLite connection"</p>
            </div>

            <div class={query_result}>
                <p>"SELECT * FROM users WHERE active = true;"</p>
                <p>"Results: 5 records found"</p>
            </div>
        </div>
    }
}

/// Example: User data display
#[azumi::component]
pub fn user_data_display() -> impl azumi::Component {
    html! {
        <style>
            .user_data_container { padding: "1.5rem"; }
            .user_table { width: "100%"; border-collapse: "collapse"; margin: "1rem 0"; }
            .user_table th { background: "#f0f0f0"; padding: "0.5rem"; text-align: "left"; border: "1px solid #ddd"; }
            .user_table td { padding: "0.5rem"; border: "1px solid #ddd"; }
            .user_row:hover { background: "#f9f9f9"; }
        </style>
        <div class={user_data_container}>
            <h3>"User Data from Database"</h3>

            <table class={user_table}>
                <thead>
                    <tr>
                        <th>"ID"</th>
                        <th>"Name"</th>
                        <th>"Email"</th>
                        <th>"Status"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr class={user_row}>
                        <td>"1"</td>
                        <td>"John Doe"</td>
                        <td>"john@example.com"</td>
                        <td>"Active"</td>
                    </tr>
                    <tr class={user_row}>
                        <td>"2"</td>
                        <td>"Jane Smith"</td>
                        <td>"jane@example.com"</td>
                        <td>"Active"</td>
                    </tr>
                    <tr class={user_row}>
                        <td>"3"</td>
                        <td>"Bob Johnson"</td>
                        <td>"bob@example.com"</td>
                        <td>"Inactive"</td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}

/// Example: Database operations
#[azumi::component]
pub fn database_operations() -> impl azumi::Component {
    html! {
        <style>
            .operations_container { padding: "1.5rem"; background: "#f9f9f9"; }
            .operation_card { margin: "0.5rem 0"; padding: "1rem"; background: "white"; border: "1px solid #eee"; }
            .operation_title { font-weight: "bold"; color: "#2196f3"; }
        </style>
        <div class={operations_container}>
            <h3>"Database Operations"</h3>

            <div class={operation_card}>
                <div class={operation_title}>"CREATE"</div>
                <p>"INSERT INTO users (name, email) VALUES ('New User', 'new@example.com')"</p>
            </div>

            <div class={operation_card}>
                <div class={operation_title}>"READ"</div>
                <p>"SELECT * FROM users WHERE id = 1"</p>
            </div>

            <div class={operation_card}>
                <div class={operation_title}>"UPDATE"</div>
                <p>"UPDATE users SET status = 'active' WHERE id = 3"</p>
            </div>

            <div class={operation_card}>
                <div class={operation_title}>"DELETE"</div>
                <p>"DELETE FROM users WHERE status = 'inactive'"</p>
            </div>
        </div>
    }
}

/// Main lesson demonstration component
#[azumi::component]
pub fn lesson12() -> impl azumi::Component {
    html! {
        <style>
            .container { padding: "20px"; }
            .header { text-align: "center"; margin-bottom: "30px"; }
            .main_title { font-size: "32px"; color: "#333"; }
            .subtitle { font-size: "18px"; color: "#666"; }
            .key_points { background: "#f9f9f9"; padding: "20px"; border-radius: "8px"; margin-bottom: "30px"; }
            .section_title { font-size: "20px"; margin-bottom: "15px"; }
            .points_list { list-style: "none"; padding: "0"; }
            .point { margin-bottom: "10px"; }
            .examples { display: "grid"; gap: "20px"; }
            .example_card { border: "1px solid #ddd"; padding: "20px"; border-radius: "8px"; }
        </style>
        <div class={container}>
            <header class={header}>
                <h1 class={main_title}>"Lesson 12: Database Integration Patterns"</h1>
                <p class={subtitle}>"Database-connected components"</p>
            </header>

            <section class={key_points}>
                <h2 class={section_title}>"Key Concepts"</h2>
                <ul class={points_list}>
                    <li class={point}>"✅ Database connection patterns"</li>
                    <li class={point}>"✅ SQL query integration"</li>
                    <li class={point}>"✅ Data display components"</li>
                    <li class={point}>"✅ CRUD operation patterns"</li>
                    <li class={point}>"✅ Type-safe database access"</li>
                </ul>
            </section>

            <section class={examples}>
                <div class={example_card}>
                    @database_integration_example()
                </div>
                <div class={example_card}>
                    @user_data_display()
                </div>
                <div class={example_card}>
                    @database_operations()
                </div>
            </section>
        </div>
    }
}

// Handler for Axum
pub async fn lesson12_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson12()))
}