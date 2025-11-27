//! Lesson 16: javascript_integration.rs
//!
//! Loading external JavaScript libraries (no inline scripts allowed)
use azumi::html;

/// Chart component using Chart.js from external CDN
pub fn chart_component() -> impl azumi::Component {
    html! {
        <div class="chart-demo">
            <h1>"JavaScript Integration with Azumi"</h1>

            <section class="demo-section">
                <h2>"Chart.js Integration"</h2>
                <p>"Loading external JavaScript libraries for interactive charts"</p>

                <canvas
                    id="salesChart"
                    width="400"
                    height="200"
                    class="chart-canvas"
                    data-chart-type="bar"
                    data-chart-labels="Jan,Feb,Mar,Apr,May,Jun"
                    data-chart-data="12000,19000,15000,25000,22000,30000">
                    "Your browser does not support the canvas element."
                </canvas>
            </section>

            <section class="demo-section">
                <h2>"Library Features"</h2>
                <div class="feature-grid">
                    <div class="feature-card">
                        <h3>"Chart.js"</h3>
                        <p>"Beautiful charts loaded from CDN"</p>
                    </div>
                    <div class="feature-card">
                        <h3>"External Files"</h3>
                        <p>"All JS must be in external files"</p>
                    </div>
                    <div class="feature-card">
                        <h3>"No Inline Scripts"</h3>
                        <p>"Azumi enforces clean separation"</p>
                    </div>
                </div>
            </section>
        </div>

        <!-- Load Chart.js from CDN (external JS only) -->
        <link href="https://cdn.jsdelivr.net/npm/chart.js" />

        <!-- JSON data for chart initialization -->
        <script type="application/json">
            {
                "salesData": {
                    "labels": ["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
                    "datasets": [{
                        "label": "Sales 2024",
                        "data": [12000, 19000, 15000, 25000, 22000, 30000],
                        "backgroundColor": [
                            "rgba(255, 99, 132, 0.2)",
                            "rgba(54, 162, 235, 0.2)",
                            "rgba(255, 205, 86, 0.2)",
                            "rgba(75, 192, 192, 0.2)",
                            "rgba(153, 102, 255, 0.2)",
                            "rgba(255, 159, 64, 0.2)"
                        ],
                        "borderColor": [
                            "rgba(255, 99, 132, 1)",
                            "rgba(54, 162, 235, 1)",
                            "rgba(255, 205, 86, 1)",
                            "rgba(75, 192, 192, 1)",
                            "rgba(153, 102, 255, 1)",
                            "rgba(255, 159, 64, 1)"
                        ],
                        "borderWidth": 1
                    }]
                }
            }
        </script>
    }
}

/// Advanced data visualization with multiple chart types
pub fn data_visualization() -> impl azumi::Component {
    html! {
        <div class="viz-demo">
            <h1>"Advanced JavaScript Integration"</h1>

            <section class="demo-section">
                <h2>"Pie Chart - Technology Usage"</h2>
                <canvas
                    id="techChart"
                    width="300"
                    height="300"
                    class="chart-canvas-small"
                    data-chart-type="pie"
                    data-chart-labels="JavaScript,Python,Rust,Go,Other"
                    data-chart-data="35,25,20,12,8">
                    "Browser not supported"
                </canvas>
            </section>

            <section class="demo-section">
                <h2>"Line Chart - Website Traffic"</h2>
                <canvas
                    id="trafficChart"
                    width="600"
                    height="300"
                    class="chart-canvas"
                    data-chart-type="line"
                    data-chart-labels="00:00,04:00,08:00,12:00,16:00,20:00"
                    data-chart-data="120,190,300,500,200,300">
                    "Browser not supported"
                </canvas>
            </section>

            <section class="demo-section">
                <h2>"Chart Configuration"</h2>
                <p>"All chart data is passed via data attributes and JSON"</p>
                <div class="config-example">
                    <code><canvas data-chart-type="bar" data-chart-data="1,2,3"></canvas></code>
                </div>
            </section>
        </div>

        <!-- Load Chart.js from CDN -->
        <link href="https://cdn.jsdelivr.net/npm/chart.js" />

        <!-- Chart configuration data -->
        <script type="application/json">
            {
                "techChartConfig": {
                    "type": "pie",
                    "data": {
                        "labels": ["JavaScript", "Python", "Rust", "Go", "Other"],
                        "datasets": [{
                            "data": [35, 25, 20, 12, 8],
                            "backgroundColor": [
                                "#FF6384",
                                "#36A2EB",
                                "#FFCE56",
                                "#4BC0C0",
                                "#9966FF"
                            ]
                        }]
                    },
                    "options": {
                        "responsive": true,
                        "plugins": {
                            "title": {
                                "display": true,
                                "text": "Programming Language Usage"
                            }
                        }
                    }
                },
                "trafficChartConfig": {
                    "type": "line",
                    "data": {
                        "labels": ["00:00", "04:00", "08:00", "12:00", "16:00", "20:00"],
                        "datasets": [{
                            "label": "Unique Visitors",
                            "data": [120, 190, 300, 500, 200, 300],
                            "borderColor": "rgb(75, 192, 192)",
                            "backgroundColor": "rgba(75, 192, 192, 0.2)",
                            "tension": 0.1
                        }]
                    },
                    "options": {
                        "responsive": true,
                        "plugins": {
                            "title": {
                                "display": true,
                                "text": "24-Hour Website Traffic"
                            }
                        }
                    }
                }
            }
        </script>
    }
}

/// External JavaScript file integration example
pub fn external_js_demo() -> impl azumi::Component {
    html! {
        <div class="external-js-demo">
            <h1>"External JavaScript Integration"</h1>

            <section class="demo-section">
                <h2>"Loading Multiple Libraries"</h2>
                <p>"External JavaScript files are loaded via CDN links"</p>

                <div class="library-grid">
                    <div class="library-card">
                        <h3>"Chart.js"</h3>
                        <p>"For data visualization"</p>
                        <code><link href="https://cdn.jsdelivr.net/npm/chart.js" /></code>
                    </div>
                    <div class="library-card">
                        <h3>"Moment.js"</h3>
                        <p>"For date handling"</p>
                        <code><link href="https://cdn.jsdelivr.net/npm/moment" /></code>
                    </div>
                    <div class="library-card">
                        <h3>"Lodash"</h3>
                        <p>"For utility functions"</p>
                        <code><link href="https://cdn.jsdelivr.net/npm/lodash" /></code>
                    </div>
                </div>
            </section>

            <section class="demo-section">
                <h2>"Data Passing"</h2>
                <p>"Pass data to JavaScript via JSON script tags"</p>

                <div class="data-example">
                    <h4>"HTML Structure:"</h4>
                    <code>
                        <script type="application/json"><br>
                        &nbsp;&nbsp;{"{ \"user\": \"Alice\", \"role\": \"admin\" }"}<br>
                        </script>
                    </code>
                </div>
            </section>

            <section class="demo-section">
                <h2>"Interactive Demo"</h2>
                <div class="interactive-demo">
                    <button id="loadChartBtn" class="btn btn-primary">"Load Chart"</button>
                    <div id="chartContainer" class="chart-container">
                        "Click the button to load a chart dynamically"
                    </div>
                </div>
            </section>
        </div>

        <!-- Load Chart.js from CDN -->
        <link href="https://cdn.jsdelivr.net/npm/chart.js" />

        <!-- Dynamic chart data -->
        <script type="application/json">
            {
                "dynamicChartData": {
                    "type": "doughnut",
                    "data": {
                        "labels": ["Desktop", "Mobile", "Tablet"],
                        "datasets": [{
                            "data": [45, 35, 20],
                            "backgroundColor": [
                                "#FF6384",
                                "#36A2EB",
                                "#FFCE56"
                            ]
                        }]
                    },
                    "options": {
                        "responsive": true,
                        "plugins": {
                            "title": {
                                "display": true,
                                "text": "Device Usage"
                            }
                        }
                    }
                }
            }
        </script>
    }
}

/// Main JavaScript integration demo
pub fn javascript_integration_demo() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson16.css" />
        <div class="js-integration">
            <header class="demo-header">
                <h1>"JavaScript Integration Demo"</h1>
                <p>"External JavaScript files only - No inline scripts allowed"</p>
            </header>

            <nav class="demo-nav">
                <button id="basicChart" class="nav-btn active">"Basic Charts"</button>
                <button id="advancedViz" class="nav-btn">"Advanced Visualizations"</button>
                <button id="externalJS" class="nav-btn">"External JS"</button>
            </nav>

            <main class="demo-content">
                <div id="basicChartContent" class="chart-content">
                    @chart_component()
                </div>

                <div id="advancedVizContent" class="chart-content" style="display: none;">
                    @data_visualization()
                </div>

                <div id="externalJSContent" class="chart-content" style="display: none;">
                    @external_js_demo()
                </div>
            </main>
        </div>

        <!-- Load Chart.js from CDN -->
        <link href="https://cdn.jsdelivr.net/npm/chart.js" />
    }
}

/// Handler for Axum
pub async fn lesson16_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&javascript_integration_demo()))
}
