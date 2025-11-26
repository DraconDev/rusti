<![CDATA[//! Lesson 16: JavaScript Integration
//! 
//! Loading and using external JavaScript
//! Demonstrates safe integration of third-party JavaScript libraries with Azumi templates.

use axum::response::{Html, IntoResponse};
use azumi::html;

pub fn lesson16_page() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Lesson 16: JavaScript Integration - Azumi"</title>
                <style src="/static/global.css" />
                <style src="/static/pages/lessons.css" />
                <style src="/static/pages/lesson16.css" />
            </head>
            <body>
                <div class="lesson-container">
                    <a href="/" class="back-link">
                        <span>"←"</span>
                        "Back to Lessons"
                    </a>

                    <div class="lesson-header">
                        <div class="lesson-number">"16"</div>
                        <h1 class="lesson-title">"JavaScript Integration"</h1>
                        <p class="lesson-subtitle">
                            "Load external scripts safely and interact with DOM elements from Azumi templates."
                        </p>
                    </div>

                    <section class="section">
                        <h2 class="section-title">📦 "Loading External Libraries"</h2>
                        <p>
                            "Azumi allows safe loading of external JavaScript libraries via CDN. Scripts execute after DOM load."
                        </p>

                        <div class="example-grid">
                            <div class="example-box">
                                <div class="example-label correct">"✅ Live Chart.js Example"</div>
                                <div id="chart-container" style="height: 300px; background: var(--azumi-surface); border-radius: var(--radius-md); padding: var(--spacing-sm);">
                                    <canvas id="myChart" width="400" height="250"></canvas>
                                </div>
                                <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
                                <script>
                                    "const ctx = document.getElementById('myChart').getContext('2d');
                                    new Chart(ctx, {
                                        type: 'bar',
                                        data: {
                                            labels: ['Red', 'Blue', 'Yellow', 'Green', 'Purple', 'Orange'],
                                            datasets: [{
                                                label: 'Dataset',
                                                data: [12, 19, 3, 5, 2, 3],
                                                backgroundColor: [
                                                    'rgba(255, 99, 132, 0.6)',
                                                    'rgba(54, 162, 235, 0.6)',
                                                    'rgba(255, 205, 86, 0.6)',
                                                    'rgba(75, 192, 192, 0.6)',
                                                    'rgba(153, 102, 255, 0.6)',
                                                    'rgba(255, 159, 64, 0.6)'
                                                ],
                                                borderColor: [
                                                    'rgba(255, 99, 132, 1)',
                                                    'rgba(54, 162, 235, 1)',
                                                    'rgba(255, 205, 86, 1)',
                                                    'rgba(75, 192, 192, 1)',
                                                    'rgba(153, 102, 255, 1)',
                                                    'rgba(255, 159, 64, 1)'
                                                ],
                                                borderWidth: 1
                                            }]
                                        },
                                        options: {
                                            responsive: true,
                                            maintainAspectRatio: false,
                                            scales: {
                                                y: { beginAtZero: true }
                                            }
                                        }
                                    });"
                                </script>
                            </div>

                            <div class="code-block">
                                "pub fn interactive_chart() -> impl azumi::Component {\n"
                                "    html! {\n"
                                "        <html>\n"
                                "            <head>\n"
                                "                <script src=\"https://cdn.jsdelivr.net/npm/chart.js\"></script>\n"
                                "            </head>\n"
                                "            <body>\n"
                                "                <canvas id=\"myChart\" width=\"400\" height=\"200\"></canvas>\n"
                                "                <script>\n"
                                "                    \"const ctx = document.getElementById('myChart').getContext('2d');\n"
                                "                    new Chart(ctx, {\n"
                                "                        type: 'bar',\n"
                                "                        data: {\n"
                                "                            labels: ['Red', 'Blue', 'Yellow', 'Green', 'Purple', 'Orange'],\n"
                                "                            datasets: [{\n"
                                "                                label: 'My First Dataset',\n"
                                "                                data: [12, 19, 3, 5, 2, 3],\n"
                                "                                backgroundColor: [\n"
                                "                                    'rgba(255, 99, 132