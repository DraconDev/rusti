<![CDATA[//! Lesson 16: javascript_integration.rs
//!
//! Loading and using external JavaScript
//! This lesson demonstrates how to safely load external JavaScript libraries
//! and execute client-side code from Azumi templates.

use azumi::html;

/// Interactive chart using Chart.js CDN
/// Demonstrates loading external scripts and DOM manipulation
pub fn interactive_chart() -> impl azumi::Component {
    html! {
        <html>
            <head>
                <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
            </head>
            <body>
                <div style="padding: 2rem; font-family: system-ui;">
                    <h1>"Chart.js Integration Example"</h1>
                    <canvas id="myChart" width="600" height="300" style="border: 1px solid #ccc; border-radius: 8px;"></canvas>
                </div>
                <script>
                    r##"const ctx = document.getElementById('myChart').getContext('2d');
                    new Chart(ctx, {
                        type: 'bar',
                        data