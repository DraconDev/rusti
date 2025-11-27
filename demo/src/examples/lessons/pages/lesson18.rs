//! Lesson 18: real_time_updates.rs
//!
//! Real-time updates with server-sent events and HTMX
use azumi::html;

/// Live chat component
pub fn live_chat() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson18.css" />
        <div class="chat-demo">
            <h1>"Real-Time Updates"</h1>
            <p>"Server-sent events and live data updates"</p>
            
            <section class="demo-section">
                <h2>"Live Chat"</h2>
                
                <div id="chat-messages" hx-sse="connect:/chat/stream" class="chat-container">
                    <div class="chat-message">
                        <strong>"System"</strong>
                        <span>"Welcome to the live chat!"</span>
                    </div>
                </div>

                <form hx-post="/chat/message" hx-swap="none" class="chat-form">
                    <input 
                        type="text" 
                        name="message" 
                        placeholder="Type your message..."
                        class="chat-input"
                        required />
                    <button type="submit" class="btn btn-primary">"Send"</button>
                </form>
            </section>

            <section class="demo-section">
                <h2>"Server-Sent Events Features"</h2>
                <div class="features-grid">
                    <div class="feature-card">
                        <h3>"Auto Reconnect"</h3>
                        <p>"Automatically reconnect if connection drops"</p>
                    </div>
                    <div class="feature-card">
                        <h3>"Event Filtering"</h3>
                        <p>"Filter events by name or type"</p>
                    </div>
                    <div class="feature-card">
                        <h3>"Multiple Connections"</h3>
                        <p>"Multiple SSE streams supported"</p>
                    </div>
                </div>
            </section>
        </div>
    }
}

/// Live dashboard with metrics
pub fn live_dashboard() -> impl azumi::Component {
    html! {
        <div class="dashboard-demo">
            <h2>"Live Dashboard"</h2>
            
            <div class="metrics-grid">
                <div class="metric-card">
                    <h3>"Active Users"</h3>
                    <div class="metric-value" hx-sse="connect:/metrics/users" hx-swap="innerHTML">
                        "0"
                    </div>
                </div>
                <div class="metric-card">
                    <h3>"Server Load"</h3>
                    <div class="metric-value" hx-sse="connect:/metrics/load" hx-swap="innerHTML">
                        "0%"
                    </div>
                </div>
                <div class="metric-card">
                    <h3>"Response Time"</h3>
                    <div class="metric-value" hx-sse="connect:/metrics/response" hx-swap="innerHTML">
                        "0ms"
                    </div>
                </div>
            </div>

            <div class="chart-container">
                <h3>"Live Data Chart"</h3>
                <div hx-sse="connect:/metrics/chart" hx-swap="innerHTML">
                    <canvas id="liveChart" width="600" height="300">
                        "Loading live data..."
                    </canvas>
                </div>
            </div>
        </div>
    }
}

/// Notification system
pub fn notification_system() -> impl azumi::Component {
    html! {
        <div class="notification-demo">
            <h2>"Notification System"</h2>
            
            <button 
                hx-post="/notifications/trigger"
                hx-sse="connect:/notifications/stream"
                hx-swap="none"
                class="btn btn-primary">"Trigger Notification"</button>
            
            <div class="notification-container">
                <h3>"Notifications"</h3>
                <div hx-sse="connect:/notifications/stream" hx-swap="beforeend" class="notifications">
                    <div class="notification">
                        <strong>"System"</strong>
                        <span>"Welcome to the notification system!"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Progress tracking
pub fn progress_tracking() -> impl azumi::Component {
    html! {
        <div class="progress-demo">
            <h2>"Progress Tracking"</h2>
            
            <button 
                hx-post="/progress/start"
                hx-sse="connect:/progress/stream"
                hx-target="#progress-bar"
                hx-swap="innerHTML"
                class="btn btn-primary">"Start Long Task"</button>
            
            <div class="progress-container">
                <div id="progress-bar" class="progress-bar">
                    <div class="progress-fill" style="width: 0%"></div>
                </div>
                <div id="progress-text" class="progress-text">"Ready to start"</div>
            </div>
            
            <div class="progress-info">
                <h3>"Progress Features:"</h3>
                <ul>
                    <li>"Real-time progress updates"</li>
                    <li>"Percentage and text feedback"</li>
                    <li>"Multiple concurrent tasks"</li>
                    <li>"Error handling"</li>
                </ul>
            </div>
        </div>
    }
}

/// Live data table
pub fn live_data_table() -> impl azumi::Component {
    html! {
        <div class="table-demo">
            <h2>"Live Data Table"</h2>
            
            <div class="table-controls">
                <button 
                    hx-post="/table/refresh"
                    hx-sse="connect:/table/stream"
                    hx-target="closest .table-demo"
                    hx-swap="outerHTML"
                    class="btn btn-secondary">"Refresh Data"</button>
            </div>
            
            <table class="live-table">
                <thead>
                    <tr>
                        <th>"ID"</th>
                        <th>"Name"</th>
                        <th>"Status"</th>
                        <th>"Last Update"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"1"</td>
                        <td>"Alice Johnson"</td>
                        <td class="status-online">"Online"</td>
                        <td>"Just now"</td>
                    </tr>
                    <tr>
                        <td>"2"</td>
                        <td>"Bob Smith"</td>
                        <td class="status-offline">"Offline"</td>
                        <td>"5 minutes ago"</td>
                    </tr>
                    <tr>
                        <td>"3"</td>
                        <td>"Carol Davis"</td>
                        <td class="status-away">"Away"</td>
                        <td>"2 minutes ago"</td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}

/// Complete real-time demo
pub fn real_time_updates_demo() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson18.css" />
        <div class="realtime-integration">
            <header class="demo-header">
                <h1>"Real-Time Updates Demo"</h1>
                <p>"Server-sent events and live data streaming"</p>
            </header>

            <nav class="demo-nav">
                <button id="chatDemo" class="nav-btn active">"Live Chat"</button>
                <button id="dashboardDemo" class="nav-btn">"Live Dashboard"</button>
                <button id="notificationsDemo" class="nav-btn">"Notifications"</button>
                <button id="progressDemo" class="nav-btn">"Progress"</button>
                <button id="tableDemo" class="nav-btn">"Live Table"</button>
            </nav>

            <main class="demo-content">
                <div id="chatDemoContent" class="demo-section">
                    @live_chat()
                </div>
                
                <div id="dashboardDemoContent" class="demo-section" style="display: none;">
                    @live_dashboard()
                </div>
                
                <div id="notificationsDemoContent" class="demo-section" style="display: none;">
                    @notification_system()
                </div>
                
                <div id="progressDemoContent" class="demo-section" style="display: none;">
                    @progress_tracking()
                </div>
                
                <div id="tableDemoContent" class="demo-section" style="display: none;">
                    @live_data_table()
                </div>
            </main>

            <aside class="demo-sidebar">
                <h3>"SSE Benefits"</h3>
                <ul>
                    <li>"No polling needed"</li>
                    <li>"Automatic reconnection"</li>
                    <li>"Simple implementation"</li>
                    <li>"Cross-browser support"</li>
                </ul>
                
                <h3>"Common Use Cases"</h3>
                <ul>
                    <li>"Live chat"</li>
                    <li>"Dashboard updates"</li>
                    <li>"Notifications"</li>
                    <li>"Progress tracking"</li>
                    <li>"Stock prices"</li>
                </ul>
                
                <h3>"HTMX SSE Attributes"</h3>
                <ul>
                    <li>"hx-sse='connect:/stream'"</li>
                    <li>"hx-swap='innerHTML'"</li>
                    <li>"hx-swap='beforeend'"</li>
                </ul>
            </aside>
        </div>
    }
}

/// Handler for Axum
pub async fn lesson18_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&real_time_updates_demo()))
}