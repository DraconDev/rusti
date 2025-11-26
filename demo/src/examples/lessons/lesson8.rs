use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 8: Real-World Examples
pub fn lesson8() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Lesson 8: Real-World Examples - Azumi"</title>
                <style src="/static/lessons.css" />
                <style src="lesson8.css" />
            </head>
            <body>
                <div class="lesson-container">
                    <header class="lesson-header">
                        <a href="/" class="back-link">"← Back to Lessons"</a>
                        <div class="lesson-number">"Lesson 8"</div>
                        <h1 class="lesson-title">"Real-World Examples"</h1>
                        <p class="lesson-subtitle">"See Azumi in action with practical applications"</p>
                    </header>

                    <section class="section">
                        <h2 class="section-title">"🎯 What You'll Learn"</h2>
                        <p>"Real-world applications combining all Azumi features: components, control flow, layouts, and HTMX integration."</p>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"📝 Todo Application"</h2>
                        <p>"A complete todo app with add, complete, and delete functionality:"</p>

                        <div class="todo-demo">
                            <div class="todo-header">
                                <h3>"My Todos"</h3>
                                <span class="todo-count">"3 items"</span>
                            </div>
                            <div class="todo-list">
                                <div class="todo-item">
                                    <input type="checkbox" checked />
                                    <span class="completed">"Learn Azumi"</span>
                                    <button class="delete-btn">"×"</button>
                                </div>
                                <div class="todo-item">
                                    <input type="checkbox" />
                                    <span>"Build an app"</span>
                                    <button class="delete-btn">"×"</button>
                                </div>
                                <div class="todo-item">
                                    <input type="checkbox" />
                                    <span>"Deploy to production"</span>
                                    <button class="delete-btn">"×"</button>
                                </div>
                            </div>
                            <div class="todo-input">
                                <input type="text" placeholder="Add a new todo..." class="todo-input-field" />
                                <button class="add-btn">"Add"</button>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"💬 Comment System"</h2>
                        <p>"Interactive comment system with threading and real-time updates:"</p>

                        <div class="comments-demo">
                            <div class="comment">
                                <div class="comment-header">
                                    <span class="comment-author">"Alice"</span>
                                    <span class="comment-time">"2 hours ago"</span>
                                </div>
                                <div class="comment-content">"Azumi makes web development so much cleaner!"</div>
                                <div class="comment-actions">
                                    <button class="reply-btn">"Reply"</button>
                                </div>
                                <div class="comment-replies">
                                    <div class="comment reply">
                                        <div class="comment-header">
                                            <span class="comment-author">"Bob"</span>
                                            <span class="comment-time">"1 hour ago"</span>
                                        </div>
                                        <div class="comment-content">"Totally agree! The type safety is amazing."</div>
                                    </div>
                                </div>
                            </div>
                            <div class="comment-form">
                                <textarea placeholder="Add a comment..." class="comment-textarea"></textarea>
                                <button class="submit-btn">"Post Comment"</button>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"📊 Dashboard with Charts"</h2>
                        <p>"Data dashboard with charts and real-time updates:"</p>

                        <div class="dashboard-demo">
                            <div class="dashboard-header">
                                <h3>"Analytics Dashboard"</h3>
                                <span class="dashboard-time">"Last updated: Just now"</span>
                            </div>
                            <div class="dashboard-grid">
                                <div class="stat-card">
                                    <div class="stat-value">"1,234"</div>
                                    <div class="stat-label">"Total Users"</div>
                                    <div class="stat-change positive">"+12% from last month"</div>
                                </div>
                                <div class="stat-card">
                                    <div class="stat-value">"98.5%"</div>
                                    <div class="stat-label">"Uptime"</div>
                                    <div class="stat-change neutral">"No change"</div>
                                </div>
                                <div class="stat-card">
                                    <div class="stat-value">"456"</div>
                                    <div class="stat-label">"Daily Active"</div>
                                    <div class="stat-change positive">"+5% from yesterday"</div>
                                </div>
                                <div class="chart-placeholder">
                                    <div class="chart-title">"User Growth"</div>
                                    <div class="chart-bars">
                                        <div class="chart-bar" style="height: 60%"></div>
                                        <div class="chart-bar" style="height: 80%"></div>
                                        <div class="chart-bar" style="height: 45%"></div>
                                        <div class="chart-bar" style="height: 90%"></div>
                                        <div class="chart-bar" style="height: 75%"></div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🏪 E-commerce Product Grid"</h2>
                        <p>"Product catalog with filtering and search:"</p>

                        <div class="products-demo">
                            <div class="product-filters">
                                <input type="text" placeholder="Search products..." class="search-input" />
                                <select class="filter-select">
                                    <option>"All Categories"</option>
                                    <option>"Electronics"</option>
                                    <option>"Books"</option>
                                    <option>"Clothing"</option>
                                </select>
                            </div>
                            <div class="products-grid">
                                <div class="product-card">
                                    <div class="product-image">"📱"</div>
                                    <div class="product-info">
                                        <h4>"Smartphone"</h4>
                                        <p class="product-price">"$699"</p>
                                        <p class="product-rating">"⭐⭐⭐⭐⭐"</p>
                                    </div>
                                </div>
                                <div class="product-card">
                                    <div class="product-image">"📚"</div>
                                    <div class="product-info">
                                        <h4>"Rust Programming"</h4>
                                        <p class="product-price">"$39"</p>
                                        <p class="product-rating">"⭐⭐⭐⭐⭐"</p>
                                    </div>
                                </div>
                                <div class="product-card">
                                    <div class="product-image">"👕"</div>
                                    <div class="product-info">
                                        <h4>"Developer T-Shirt"</h4>
                                        <p class="product-price">"$25"</p>
                                        <p class="product-rating">"⭐⭐⭐⭐"</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🚀 Key Takeaways"</h2>
                        <div class="highlight-box">
                            <p><strong>"What makes Azumi perfect for production:"</strong></p>
                            <ul>
                                <li>"✅ <strong>Type Safety</strong> - Catch errors at compile time"</li>
                                <li>"✅ <strong>Performance</strong> - Zero runtime overhead"</li>
                                <li>"✅ <strong>Maintainability</strong> - Clean separation of concerns"</li>
                                <li>"✅ <strong>Scalability</strong> - Component-based architecture"</li>
                                <li>"✅ <strong>Developer Experience</strong> - Great IDE support"</li>
                                <li>"✅ <strong>CSS Safety</strong> - No dead styles or conflicts"</li>
                            </ul>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🎓 Congratulations!"</h2>
                        <div class="congratulations-box">
                            <p>"You've completed all Azumi lessons! You now know:"</p>
                            <ul>
                                <li>"✅ Strict quoting rules for type safety"</li>
                                <li>"✅ CSS validation and automatic scoping"</li>
                                <li>"✅ Global styles and design tokens"</li>
                                <li>"✅ Control flow with @if, @for, @match, @let"</li>
                                <li>"✅ Components with props"</li>
                                <li>"✅ HTMX integration for interactivity"</li>
                                <li>"✅ Layout composition"</li>
                                <li>"✅ Real-world application patterns"</li>
                            </ul>
                            <p><strong>"Ready to build amazing applications with Azumi!"</strong></p>
                        </div>
                    </section>

                    <nav class="nav-buttons">
                        <a href="/lesson-7" class="btn btn-secondary">"← Previous: Layouts"</a>
                        <a href="/" class="btn">"🎉 Complete: Back to Home"</a>
                    </nav>
                </div>
            </body>
        </html>
    }
}

pub async fn lesson8_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson8()))
}
