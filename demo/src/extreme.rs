// Comprehensive Examples - Showcasing all Rusti! features with inline styles
use axum::response::{Html, IntoResponse};
use rusti::rusti;

/// Example 1: Basic HTML with inline styles
pub fn basic_html_example() -> impl rusti::Component {
    rusti! {
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Basic HTML Example</title>
                <style>
                    body {
                        margin: 0;
                        padding: 20px;
                        font-family: system-ui;
                        background: linear-gradient(135deg, #f00 0%, #f00 100%);
                        min-height: 100vh;
                    }
                    .container {
                        max-width: 800px;
                        margin: 0 auto;
                        background: #f00;
                        padding: 2rem;
                        border-radius: 12px;
                        box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
                    }
                    h1 {
                        color: #fff;
                        margin-top: 0;
                    }
                </style>
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

/// Example 2: Dynamic Content
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
                <style>
                    body {
                        margin: 0;
                        padding: 20px;
                        font-family: Segoe UI, Tahoma, Geneva, Verdana, sans-serif;
                        background: linear-gradient(to bottom right, rgb(26, 32, 44), rgb(45, 55, 72));
                        color: white;
                        min-height: 100vh;
                    }
                    .card {
                        background: rgba(255, 255, 255, 0.1);
                        backdrop-filter: blur(10px);
                        border: 1px solid rgba(255, 255, 255, 0.2);
                        border-radius: 16px;
                        padding: 2rem;
                        max-width: 600px;
                        margin: 2rem auto;
                    }
                    .badge {
                        display: inline-block;
                        background: rgb(245, 101, 101);
                        color: white;
                        padding: 0.25rem 0.75rem;
                        border-radius: 9999px;
                        font-size: 0.875rem;
                        font-weight: bold;
                    }
                    .status {
                        color: rgb(104, 211, 145);
                        font-weight: bold;
                    }
                </style>
            </head>
            <body>
                <div class="card">
                    <h1>Welcome, { user_name }!</h1>
                    <p>You have <span class="badge">{ message_count }</span> new messages.</p>
                    @if logged_in {
                        <p class="status">Status: Logged In</p>
                    } else {
                        <p>Please log in to continue.</p>
                    }
                </div>
            </body>
        </html>
    }
}

/// Example 3: Loops and Lists
pub fn loops_example() -> impl rusti::Component {
    let fruits = vec!["Apple", "Banana", "Cherry", "Grapes", "Orange"];
    let colors = vec![
        ("Red", "rgb(239,68,68)"),
        ("Green", "rgb(16,185,129)"),
        ("Blue", "rgb(59,130,246)"),
        ("Purple", "rgb(139,92,246)"),
    ];

    rusti! {
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Loops Example</title>
                <style>
                    body {
                        margin: 0;
                        padding: 20px;
                        font-family: Arial, sans-serif;
                        background: linear-gradient(45deg, rgb(255, 107, 107), rgb(78, 205, 196));
                        min-height: 100vh;
                    }
                    .container {
                        max-width: 800px;
                        margin: 0 auto;
                        background: white;
                        padding: 2rem;
                        border-radius: 20px;
                        box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
                    }
                    h2 {
                        color: rgb(45, 55, 72);
                        border-bottom: 3px solid rgb(78, 205, 196);
                        padding-bottom: 0.5rem;
                    }
                    ul {
                        list-style: none;
                        padding: 0;
                    }
                    li {
                        background: rgb(247, 250, 252);
                        margin: 0.5rem 0;
                        padding: 1rem;
                        border-radius: 8px;
                        border-left: 4px solid rgb(78, 205, 196);
                        transition: transform 0.2s;
                    }
                    li:hover {
                        transform: translateX(5px);
                        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
                    }
                    .color-box {
                        display: inline-block;
                        width: 30px;
                        height: 30px;
                        border-radius: 6px;
                        margin-right: 10px;
                        vertical-align: middle;
                        border: 2px solid rgb(226, 232, 240);
                    }
                </style>
            </head>
            <body>
                <div class="container">
                    <h2>Fruit List</h2>
                    <ul>
                        @for fruit in fruits.iter() {
                            <li>{ fruit }</li>
                        }
                    </ul>

                    <h2>Color Palette</h2>
                    <ul>
                        @for (name, hex) in colors.iter() {
                            <li>
                                <span class="color-box" style={hex}></span>
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
                <style>
                    body {
                        margin: 0;
                        padding: 20px;
                        font-family: Inter, sans-serif;
                        background: linear-gradient(to right, rgb(67, 67, 67), rgb(0, 0, 0));
                        min-height: 100vh;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                    }
                    .status-card {
                        background: white;
                        padding: 3rem;
                        border-radius: 24px;
                        box-shadow: 0 25px 50px rgba(0, 0, 0, 0.5);
                        text-align: center;
                        min-width: 400px;
                    }
                    .status-badge {
                        display: inline-block;
                        padding: 1rem 2rem;
                        border-radius: 12px;
                        font-size: 1.5rem;
                        font-weight: bold;
                        margin: 1rem 0;
                    }
                    .status-active {
                        background: linear-gradient(135deg, rgb(102, 126, 234) 0%, rgb(118, 75, 162) 100%);
                        color: white;
                    }
                    .status-pending {
                        background: linear-gradient(135deg, rgb(240, 147, 251) 0%, rgb(245, 87, 108) 100%);
                        color: white;
                    }
                    .status-suspended {
                        background: linear-gradient(135deg, rgb(250, 112, 154) 0%, rgb(254, 225, 64) 100%);
                        color: rgb(45, 55, 72);
                    }
                    .status-archived {
                        background: linear-gradient(135deg, rgb(137, 247, 254) 0%, rgb(102, 166, 255) 100%);
                        color: white;
                    }
                </style>
            </head>
            <body>
                <div class="status-card">
                    <h1>Account Status</h1>
                    @match user_status {
                        Status::Active => {
                            <div class="status-badge status-active">
                                Active
                            </div>
                            <p>Your account is active and ready to use!</p>
                        }
                        Status::Pending => {
                            <div class="status-badge status-pending">
                                Pending
                            </div>
                            <p>Your account is awaiting approval.</p>
                        }
                        Status::Suspended => {
                            <div class="status-badge status-suspended">
                                Suspended
                            </div>
                            <p>Your account has been temporarily suspended.</p>
                        }
                        Status::Archived => {
                            <div class="status-badge status-archived">
                                Archived
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
fn card<'a>(title: &'a str, content: impl rusti::Component + 'a) -> impl rusti::Component + 'a {
    rusti! {
        <div class="card">
            <h3 class="card-title">{ title }</h3>
            <div class="card-content">
                @content
            </div>
        </div>
    }
}

fn button<'a>(label: &'a str, color: &'a str) -> impl rusti::Component + 'a {
    let class_name = format!("btn btn-{}", color);
    rusti! {
        <button class={class_name.as_str()}>{ label }</button>
    }
}

pub fn component_composition_example() -> impl rusti::Component {

    rusti! {
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Component Composition</title>
                <style>
                    body {
                        margin: 0;
                        padding: 20px;
                        font-family: system-ui;
                        background: linear-gradient(to bottom, rgb(30, 58, 138), rgb(59, 130, 246));
                        min-height: 100vh;
                    }
                    .container {
                        max-width: 1200
