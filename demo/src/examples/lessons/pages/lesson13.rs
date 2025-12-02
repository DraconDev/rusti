use azumi::html;

/// Lesson 13: Real-time Features with External Scripts
///
/// Real-time features using external scripts (no inline scripts)
#[azumi::component]
pub fn chat_interface() -> impl azumi::Component {
    html! {
        <style>
            .chat_container { display: "grid"; gap: "1rem"; max-width: "600px"; }
            .messages { height: "400px"; overflow-y: "auto"; border: "1px solid #ddd"; padding: "1rem"; }
            .message { margin-bottom: "0.5rem"; padding: "0.5rem"; background: "#f0f0f0"; border-radius: "4px"; }
            .input_area { display: "grid"; gap: "0.5rem"; }
            .chat_input { padding: "0.5rem"; }
            .send_button { padding: "0.5rem"; background: "#2196f3"; color: "white"; border: "none"; }
        </style>
        <div class={chat_container}>
            <div class={messages} id="messages">
                <div class={message}>"Welcome to Azumi Chat!"</div>
            </div>
            <div class={input_area}>
                <input class={chat_input} type="text" id="chat-input" placeholder="Type your message..." />
                <button class={send_button}>"Send"</button>
            </div>
        </div>
        <!-- External script import (required by Azumi) -->
        <script src="/static/chat.js"></script>
    }
}

/// Example: Real-time notifications
#[azumi::component]
pub fn realtime_notifications() -> impl azumi::Component {
    html! {
        <style>
            .notifications_container { padding: "1.5rem"; background: "#f9f9f9"; }
            .notification { padding: "0.75rem"; margin: "0.5rem 0"; background: "white"; border: "1px solid #eee"; border-radius: "4px"; }
            .notification_new { background: "#e8f5e9"; }
            .notification_old { background: "#ffebee"; }
            .notification_button { padding: "0.5rem 1rem"; background: "#4caf50"; color: "white"; border: "none"; cursor: "pointer"; }
        </style>
        <div class={notifications_container}>
            <h3>"Real-time Notifications"</h3>

            <div id="notification-list">
                <div class={notification} class={notification_new}>
                    <p>"New message received"</p>
                    <p class={notification_button}>"Dismiss"</p>
                </div>

                <div class={notification} class={notification_old}>
                    <p>"System update available"</p>
                    <p class={notification_button}>"Update Now"</p>
                </div>
            </div>

            <button class={notification_button} id="refresh-btn">"Refresh Notifications"</button>
        </div>
        <script src="/static/notifications.js"></script>
    }
}

/// Example: Live data visualization
#[azumi::component]
pub fn live_data_visualization() -> impl azumi::Component {
    html! {
        <style>
            .visualization_container { padding: "1.5rem"; }
            .data_display { display: "grid"; grid-template-columns: "repeat(auto-fit, minmax(150px, 1fr))"; gap: "1rem"; }
            .data_card { padding: "1rem"; background: "#f0f0f0"; border: "1px solid #eee"; text-align: "center"; }
            .data_value { font-size: "1.5rem"; font-weight: "bold"; color: "#2196f3"; }
            .data_label { font-size: "0.9rem"; color: "#666"; }
            .update_button { padding: "0.75rem"; background: "#ff4081"; color: "white"; border: "none"; cursor: "pointer"; }
        </style>
        <div class={visualization_container}>
            <h3>"Live Data Dashboard"</h3>

            <div class={data_display} id="live-data">
                <div class={data_card}>
                    <div class={data_value}>"1,245"</div>
                    <div class={data_label}>"Active Users"</div>
                </div>

                <div class={data_card}>
                    <div class={data_value}>"87%"</div>
                    <div class={data_label}>"System Health"</div>
                </div>

                <div class={data_card}>
                    <div class={data_value}>"42"</div>
                    <div class={data_label}>"Pending Tasks"</div>
                </div>
            </div>

            <button class={update_button} id="update-data">"Update Data"</button>
        </div>
        <script src="/static/data-visualization.js"></script>
    }
}

/// Main lesson demonstration component
#[azumi::component]
pub fn lesson13() -> impl azumi::Component {
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
                <h1 class={main_title}>"Lesson 13: Real-time Features with External Scripts"</h1>
                <p class={subtitle}>"Real-time features using external scripts"</p>
            </header>

            <section class={key_points}>
                <h2 class={section_title}>"Key Concepts"</h2>
                <ul class={points_list}>
                    <li class={point}>"✅ External script integration"</li>
                    <li class={point}>"✅ Real-time data updates"</li>
                    <li class={point}>"✅ No inline scripts allowed"</li>
                    <li class={point}>"✅ Proper script importing"</li>
                    <li class={point}>"✅ Interactive component patterns"</li>
                </ul>
            </section>

            <section class={examples}>
                <div class={example_card}>
                    @chat_interface()
                </div>
                <div class={example_card}>
                    @realtime_notifications()
                </div>
                <div class={example_card}>
                    @live_data_visualization()
                </div>
            </section>
        </div>
    }
}

// Handler for Axum
pub async fn lesson13_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson13()))
}