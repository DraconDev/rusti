//! Lesson 16: JavaScript Integration
//!
//! Loading external JavaScript libraries and inline scripts
use azumi::html;

/// Interactive Chart.js example
pub fn interactive_chart() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson16.css" />
        <div class="lesson16-container">
            <header class="lesson16-header">
                <h1 class="lesson16-title">"Lesson 16: JavaScript Integration"</h1>
                <p class="lesson16-subtitle">"External libraries + inline scripts with Azumi"</p>
            </header>

            <section class="lesson16-section">
                <h2 class="section-title">"Chart.js Example"</h2>
                <canvas id="myChart" width="400" height="200" class="chart-canvas"></canvas>
            </section>

            <section class="lesson16-section">
                <h2 class="section-title">"Key Points"</h2>
                <ul class="lesson16-list">
                    <li>"✅ External <script src='CDN'> for libraries"</li>
                    <li>"✅ Inline <script> for custom JS"</li>
                    <li>"✅ No inline styles - use classes"</li>
                    <li>"✅ Azumi validates HTML structure/accessibility"</li>
                    <li>"✅ JS runs after DOM ready"</li>
                </ul>
            </section>
        </div>

        // External Chart.js CDN
        <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>

        // Inline script - runs after render
        <script>
            "document.addEventListener('DOMContentLoaded', function() {
                const ctx = document.getElementById('myChart').getContext('2d');
                new Chart(ctx, {
                    type: 'bar',
                    data: {
                        labels: ['Red', 'Blue', 'Yellow', 'Green', 'Purple', 'Orange'],
                        datasets: [{
                            label: 'Azumi Dataset',
                            data: [12, 19, 3, 5, 2, 3],
                            backgroundColor: [
                                'rgba(255, 99, 132, 0.2)',
                                'rgba(54, 162, 235, 0.2)',
                                'rgba(255, 205, 86, 0.2)',
                                'rgba(75, 192, 192, 0.2)',
                                'rgba(153, 102, 255, 0.2)',
                                'rgba(255, 159, 64, 0.2)'
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
                        scales: {
                            y: {
                                beginAtZero: true
                            }
                        }
                    }
                });
            });"
        </script>
    }
}

/// Handler
pub async fn lesson16_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&interactive_chart()))
}
