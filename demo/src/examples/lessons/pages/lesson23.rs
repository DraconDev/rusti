//! Lesson 23: CDN Libraries
//!
//! Loading external CSS and JavaScript from CDNs

use azumi::html;

#[azumi::component]
pub fn cdn_demo() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson23.css" />

        // Load animate.css from CDN
        <link
            href="https://cdn.jsdelivr.net/npm/animate.css@4.1.1/animate.min.css"
            rel="stylesheet"
        />

        <div class="container">
            <h1>"Lesson 23: CDN Libraries"</h1>

            <h2>"Animated Elements"</h2>
            <div class="demo-box">
                <div class="animate__animated animate__bounce card">
                    <h3>"Bouncing!"</h3>
                    <p>"Using animate.css from CDN"</p>
                </div>

                <div class="animate__animated animate__fadeIn card">
                    <h3>"Fading in!"</h3>
                    <p>"Applied via CSS classes"</p>
                </div>

                <div class="animate__animated animate__slideInUp card">
                    <h3>"Sliding up!"</h3>
                    <p>"No JavaScript needed"</p>
                </div>
            </div>

            <h2>"Chart.js from CDN"</h2>
            <canvas id="myChart" width="400" height="200"></canvas>

            <div class="note">
                <strong>"CDN Rules:"</strong>
                <ul>
                    <li>"✅ https:// links allowed"</li>
                    <li>"✅ Both CSS and JS"</li>
                    <li>"✅ External libraries"</li>
                    <li>"❌ Local files - use " <code>"<style src>"</code></li>
                </ul>
            </div>
        </div>

        // Load Chart.js from CDN
        <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>

        // Initialize chart
        <script src="/static/lesson23-chart.js"></script>
    }
}

pub async fn lesson23_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @cdn_demo() }))
}
