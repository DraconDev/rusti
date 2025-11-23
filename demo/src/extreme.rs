// Comprehensive Examples - Showcasing all Rusti! features
use axum::response::{Html, IntoResponse};
use rusti::rusti;

/// Example 1: Basic HTML with inline styles
pub fn basic_html_example() -> impl rusti::Component {
    let styles = r"body{margin:0;padding:20px;font-family:system-ui;background:linear-gradient(135deg,#667eea 0%,#764ba2 100%);min-height:100vh}.container{max-width:800px;margin:0 auto;background:white;padding:2rem;border-radius:12px;box-shadow:0 10px 40px rgba(0,0,0,0.2)}h1{color:#667eea;margin-top:0}";
    
    rusti! {
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Basic HTML Example</title>
                <style>{styles}</style>
            </head>
            <body>
                <div class="container">
                    <h1>Basic HTML Example</h1>
                    <p>This demonstrates basic HTML structure with inline styles.</p>
                    <p>All styles are defined directly in the style tag!</p>
                </div>
            </body>
        </html>
    }
}

/// Example 2: Dynamic Content & Interpolation
pub fn dynamic_content_example() -> impl rusti::Component {
    let user_name = "Alice";
    let message_count = 5;
    let logged_in = true;
    
    rusti! {
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Dynamic Content Example</title>
                <style>body{margin:0;padding:20px;font-family:Segoe UI,Tahoma,Geneva,Verdana,sans-serif;background:linear-gradient(to bottom right,#1a202c,#2d3748);color:white;min-height:100vh}.card{background:rgba(255,255,255,0.1);backdrop-filter:blur(10px);border:1px solid rgba(255,255,255,0.2);border-radius:16px;padding:2rem;max-width:600px;margin:2rem auto}.badge{display:inline-block;background:#f56565;color:white;padding:0.25rem 0.75rem;border-radius:9999px;font-size:0.875rem;font-weight:bold}.status{color:#68d391;font-weight:bold}</style>
            </head>
            <body>
                <div class="card">
                    <h1>Welcome, { user_name }!</h1>
                    <p>You have <span class="badge">{ message_count }</span> new messages.</p>
                    @if logged_in {
                        <p class="status">✓ Status: Logged In</p>
                    } @else {
                        <p>Please log in to continue.</p>
                    }
                </div>
            </body>
        </html>
    }
}

/// Example 3: Loops and Lists
pub fn loops_example() -> impl rusti::Component {
    let fruits = vec!["🍎 Apple", "🍌 Banana", "🍒 Cherry", "🍇 Grapes", "🍊 Orange"];
    let colors = vec![
        ("Red", "#ef4444"),
        ("Green", "#10b981"),
        ("Blue", "#3b82f6"),
        ("Purple", "#8b5cf6"),
    ];
    
    rusti! {
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Loops Example</title>
                <style>body{margin:0;padding:20px;font-family:Arial,sans-serif;background:linear-gradient(45deg,#ff6b6b,#4ecdc4);min-height:100vh}.container{max-width:800px;margin:0 auto;background:white;padding:2rem;border-radius:20px;box-shadow:0 20px 60px rgba(0,0,0,0.3)}h2{color:#2d3748;border-bottom:3px solid #4ecdc4;padding-bottom:0.5rem}ul{list-style:none;padding:0}li{background:#f7fafc;margin:0.5rem 0;padding:1rem;border-radius:8px;border-left:4px solid #4ecdc4;transition:transform 0.2s}li:hover{transform:translateX(5px);box-shadow:0 4px 8px rgba(0,0,0,0.1)}.color-box{display:inline-block;width:30px;height:30px;border-radius:6px;margin-right:10px;vertical-align:middle;border:2px solid #e2e8f0}</style>
            </head>
            <body>
                <div class="container">
                    <h2>Fruit List</h2>
                    <ul>
                        @for fruit in fruits {
                            <li>{ fruit }</li>
                        }
                    </ul>
                    
                    <h2>Color Palette</h2>
                    <ul>
                        @for (name, hex) in colors {
                            <li>
                                <span class="color-box" style={"background-color:"}{hex}></span>
                                <strong>{ name }</strong> - { hex }
                            </li>
                        }
                    </ul>
                </div>
            </body>
        </html>
    }
}

/// Example 4: Pattern Matching
pub fn pattern_matching_example() -> impl rusti::Component {
    #[derive(Clone)]
    enum Status {
        Active,
        Pending,
        Suspended,
        Archived,
    }
    
    let user_status = Status::Active;
    
    rusti! {
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Pattern Matching Example</title>
                <style>body{margin:0;padding:20px;font-family:Inter,sans-serif;background:linear-gradient(to right,#434343,#000000);min-height:100vh;display:flex;align-items:center;justify-content:center}.status-card{background:white;padding:3rem;border-radius:24px;box-shadow:0 25px 50px rgba(0,0,0,0.5);text-align:center;min-width:400px}.status-badge{display:inline-block;padding:1rem 2rem;border-radius:12px;font-size:1.5rem;font-weight:bold;margin:1rem 0}.status-active{background:linear-gradient(135deg,#667eea 0%,#764ba2 100%);color:white}.status-pending{background:linear-gradient(135deg,#f093fb 0%,#f5576c 100%);color:white}.status-suspended{background:linear-gradient(135deg,#fa709a 0%,#fee140 100%);color:#2d3748}.status-archived{background:linear-gradient(135deg,#89f7fe 0%,#66a6ff 100%);color:white}</style>
            </head>
            <body>
                <div class="status-card">
                    <h1>Account Status</h1>
                    @match user_status {
                        Status::Active => {
                            <div class="status-badge status-active">
                                ✓ Active
                            </div>
                            <p>Your account is active and ready to use!</p>
                        }
                        Status::Pending => {
                            <div class="status-badge status-pending">
                                ⏳ Pending
                            </div>
                            <p>Your account is awaiting approval.</p>
                        }
                        Status::Suspended => {
                            <div class="status-badge status-suspended">
                                ⚠ Suspended
                            </div>
                            <p>Your account has been temporarily suspended.</p>
                        }
                        Status::Archived => {
                            <div class="status-badge status-archived">
                                📦 Archived
                            </div>
                            <p>This account has been archived.</p>
                        }
                    }
                </div>
            </body>
        </html>
    }
}

/// Example 5: Component Composition
fn card(title: &str, content: impl rusti::Component) -> impl rusti::Component + '_ {
    rusti! {
        <div class="card">
            <h3 class="card-title">{ title }</h3>
            <div class="card-content">
                @content
            </div>
        </div>
    }
}

fn button(label: &str, color: &str) -> impl rusti::Component + '_ {
    rusti! {
        <button class={"btn btn-"}{color}>
            { label }
        </button>
    }
}

pub fn component_composition_example() -> impl rusti::Component {
    rusti! {
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Component Composition</title>
                <style>body{margin:0;padding:20px;font-family:system-ui;background:linear-gradient(to bottom,#1e3a8a,#3b82f6);min-height:100vh}.container{max-width:1200px;margin:0 auto}h1{color:white;text-align:center;font-size:2.5rem;margin-bottom:2rem}.grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(300px,1fr));gap:2rem}.card{background:white;border-radius:16px;padding:1.5rem;box-shadow:0 10px 30px rgba(0,0,0,0.3);transition:transform 0.3s}.card:hover{transform:translateY(-5px)}.card-title{margin-top:0;color:#1e3a8a;border-bottom:2px solid #3b82f6;padding-bottom:0.5rem}.card-content{margin-top:1rem}.btn{padding:0.75rem 1.5rem;border:none;border-radius:8px;font-weight:bold;cursor:pointer;margin-right:0.5rem;margin-top:0.5rem;transition:all 0.2s}.btn:hover{transform:scale(1.05)}.btn-primary{background:#3b82f6;color:white}.btn-success{background:#10b981;color:white}.btn-danger{background:#ef4444;color:white}.btn-warning{background:#f59e0b;color:white}</style>
            </head>
            <body>
                <div class="container">
                    <h1>Component Composition Demo</h1>
                    <div class="grid">
                        @card("User Profile", {
                            <p>Manage your profile settings and preferences.</p>
                            @button("Edit Profile", "primary")
                            @button("Change Password", "warning")
                        })
                        
                        @card("Notifications", {
                            <p>You have 3 new notifications waiting for you.</p>
                            @button("View All", "success")
                            @button("Mark as Read", "primary")
                        })
                        
                        @card("Danger Zone", {
                            <p>Irreversible actions. Proceed with caution.</p>
                            @button("Delete Account", "danger")
                        })
                    </div>
                </div>
            </body>
        </html>
    }
}

/// Example 6: Complex Interactive Dashboard (HTMX Ready)
pub fn interactive_dashboard_example() -> impl rusti::Component {
    let stats = vec![
        ("Active Users", "1,234", "↑ 12%"),
        ("Revenue", "$45,678", "↑ 8%"),
        ("Orders", "892", "↓ 3%"),
        ("Satisfaction", "98%", "↑ 2%"),
    ];
    
    rusti! {
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Interactive Dashboard</title>
                <script src="https://unpkg.com/htmx.org@1.9.10"></script>
                <style>*{margin:0;padding:0;box-sizing:border-box}body{font-family:Segoe UI,Tahoma,Geneva,Verdana,sans-serif;background:linear-gradient(135deg,#0f172a 0%,#1e293b 100%);color:white;min-height:100vh;padding:2rem}.dashboard{max-width:1400px;margin:0 auto}h1{font-size:3rem;margin-bottom:2rem;background:linear-gradient(to right,#60a5fa,#a78bfa);-webkit-background-clip:text;-webkit-text-fill-color:transparent;background-clip:text}.stats-grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(250px,1fr));gap:1.5rem;margin-bottom:2rem}.stat-card{background:rgba(30,41,59,0.7);backdrop-filter:blur(10px);border:1px solid rgba(255,255,255,0.1);border-radius:16px;padding:1.5rem;transition:all 0.3s}.stat-card:hover{transform:translateY(-5px);box-shadow:0 20px 40px rgba(96,165,250,0.3);border-color:rgba(96,165,250,0.5)}.stat-label{font-size:0.875rem;color:#94a3b8;margin-bottom:0.5rem}.stat-value{font-size:2rem;font-weight:bold;margin-bottom:0.25rem}.stat-change{font-size:0.875rem;color:#10b981}.actions{display:flex;gap:1rem;flex-wrap:wrap;margin-bottom:2rem}.action-btn{padding:1rem 2rem;border:none;border-radius:12px;font-weight:bold;cursor:pointer;transition:all 0.2s;font-size:1rem}.action-btn:hover{transform:scale(1.05);box-shadow:0 10px 20px rgba(0,0,0,0.3)}.btn-blue{background:linear-gradient(135deg,#3b82f6,#1d4ed8);color:white}.btn-purple{background:linear-gradient(135deg,#a78bfa,#7c3aed);color:white}.btn-green{background:linear-gradient(135deg,#10b981,#059669);color:white}.content-area{background:rgba(30,41,59,0.5);border:1px solid rgba(255,255,255,0.1);border-radius:16px;padding:2rem;min-height:200px}@keyframes fadeIn{from{opacity:0;transform:translateY(20px)}to{opacity:1;transform:translateY(0)}}.fade-in{animation:fadeIn 0.5s ease-out}</style>
            </head>
            <body>
                <div class="dashboard">
                    <h1>📊 Analytics Dashboard</h1>
                    
                    <div class="stats-grid">
                        @for (label, value, change) in stats {
                            <div class="stat-card fade-in">
                                <div class="stat-label">{ label }</div>
                                <div class="stat-value">{ value }</div>
                                <div class="stat-change">{ change }</div>
                            </div>
                        }
                    </div>
                    
                    <div class="actions">
                        <button class="action-btn btn-blue" hx-get="/api/data" hx-target="#content">
                            Load Data
                        </button>
                        <button class="action-btn btn-purple" hx-post="/api/refresh" hx-target="#content">
                            Refresh Stats
                        </button>
                        <button class="action-btn btn-green" hx-get="/api/export" hx-target="#content">
                            Export Report
                        </button>
                    </div>
                    
                    <div id="content" class="content-area">
                        <p>Click a button above to load dynamic content...</p>
                    </div>
                </div>
            </body>
        </html>
    }
}

/// Example 7: Form with Validation Styles
pub fn form_example() -> impl rusti::Component {
    rusti! {
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Form Example</title>
                <style>body{margin:0;padding:20px;font-family:system-ui;background:linear-gradient(to bottom right,#ff6e7f,#bfe9ff);min-height:100vh;display:flex;align-items:center;justify-content:center}.form-container{background:white;padding:2.5rem;border-radius:20px;box-shadow:0 20px 60px rgba(0,0,0,0.2);max-width:500px;width:100%}h1{margin-top:0;color:#2d3748}.form-group{margin-bottom:1.5rem}label{display:block;margin-bottom:0.5rem;color:#4a5568;font-weight:600}input,textarea{width:100%;padding:0.75rem;border:2px solid #e2e8f0;border-radius:8px;font-size:1rem;transition:all 0.2s}input:focus,textarea:focus{outline:none;border-color:#3b82f6;box-shadow:0 0 0 3px rgba(59,130,246,0.1)}input:invalid{border-color:#ef4444}.submit-btn{width:100%;padding:1rem;background:linear-gradient(135deg,#667eea 0%,#764ba2 100%);color:white;border:none;border-radius:8px;font-size:1.1rem;font-weight:bold;cursor:pointer;transition:transform 0.2s}.submit-btn:hover{transform:translateY(-2px);box-shadow:0 10px 20px rgba(102,126,234,0.4)}</style>
            </head>
            <body>
                <div class="form-container">
                    <h1>Contact Us</h1>
                    <form action="/submit" method="post">
                        <div class="form-group">
                            <label for="name">Name</label>
                            <input type="text" id="name" name="name" required />
                        </div>
                        
                        <div class="form-group">
                            <label for="email">Email</label>
                            <input type="email" id="email" name="email" required />
                        </div>
                        
                        <div class="form-group">
                            <label for="message">Message</label>
                            <textarea id="message" name="message" rows="5" required></textarea>
                        </div>
                        
                        <button type="submit" class="submit-btn">
                            Send Message
                        </button>
                    </form>
                </div>
            </body>
        </html>
    }
}

// Axum handlers for all examples
pub async fn basic_html_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&basic_html_example()))
}

pub async fn dynamic_content_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&dynamic_content_example()))
}

pub async fn loops_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&loops_example()))
}

pub async fn pattern_matching_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&pattern_matching_example()))
}

pub async fn component_composition_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&component_composition_example()))
}

pub async fn dashboard_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&interactive_dashboard_example()))
}

pub async fn form_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&form_example()))
}
