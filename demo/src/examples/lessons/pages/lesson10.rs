use axum::response::{Html, IntoResponse};
use azumi::html;

/// Lesson 10: Advanced Control Flow Patterns
pub fn lesson10() -> impl azumi::Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"Lesson 10: Advanced Control Flow Patterns - Azumi"</title>
                <style src="/static/pages/lessons.css" />
            </head>
            <body>
                <div class="lesson-container">
                    <header class="lesson-header">
                        <a href="/" class="back-link">"← Back to Lessons"</a>
                        <div class="lesson-number">"Control Flow"</div>
                        <h1 class="lesson-title">"Advanced Control Flow Patterns"</h1>
                        <p class="lesson-subtitle">"Combine @if, @for, @match, @let for sophisticated template logic"</p>
                    </header>

                    <section class="section">
                        <h2 class="section-title">"🎯 What You'll Learn"</h2>
                        <p>"This advanced lesson brings together all control flow concepts:"</p>
                        <ul>
                            <li>"Combining multiple control flow structures effectively"</li>
                            <li>"Complex conditional rendering patterns"</li>
                            <li>"Data processing and transformation workflows"</li>
                            <li>"Performance considerations for nested structures"</li>
                            <li>"Real-world pattern examples and anti-patterns"</li>
                        </ul>
                        <div class="prerequisite-box">
                            <strong>"📋 Prerequisites:"</strong>
                            <p>"Master all previous control flow lessons: " <a href="/lesson-6">"@if"</a> ", " <a href="/lesson-7">"@for"</a> ", " <a href="/lesson-8">"@match"</a> ", and " <a href="/lesson-9">"@let"</a> "."</p>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🏗️ Pattern 1: Nested Conditional Rendering"</h2>
                        <p>"Build complex UI states by combining @if, @let, and @match:"</p>

                        <div class="code-example">
                            <h4>"User Permission System"</h4>
                            <pre class="code-block">{"pub fn user_permissions(user: &User, resource: &Resource) -> impl azumi::Component {"}
    {"    html! {"}
    {"        <div class=\"permission-check\">"}
    {"            @let user_role = &user.role;"}
    {"            @let has_permission = {"}
    {"                match (user_role, &resource.required_permission) {"}
    {"                    (\"admin\", _) => true,"}
    {"                    (\"moderator\", \"read\") => true,"}
    {"                    (\"user\", \"read\") => true,"}
    {"                    _ => false,"}
    {"                }"}
    {"            };"}
    {"            "}
    {"            @if user.is_active {"}
    {"                @if has_permission {"}
    {"                    <button class=\"btn-primary\">\"Access Resource\"</button>"}
    {"                } @else {"}
    {"                    <div class=\"permission-denied\">"}
    {"                        <p>\"Insufficient permissions\"</p>"}
    {"                        @if user_role == \"user\" {"}
    {"                            <p>\"Contact an administrator for access\"</p>"}
    {"                        }"}
    {"                    </div>"}
    {"                }"}
    {"            } @else {"}
    {"                <div class=\"account-suspended\">"}
    {"                    <p>\"Your account is suspended\"</p>"}
    {"                </div>"}
    {"            }"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"📊 Pattern 2: Data Processing Pipeline"</h2>
                        <p>"Transform and process collections with @let, @for, and @match:"</p>

                        <div class="code-example">
                            <h4>"Dashboard Analytics"</h4>
                            <pre class="code-block">{"pub fn analytics_dashboard(analytics: &[AnalyticsData]) -> impl azumi::Component {"}
    {"    html! {"}
    {"        <div class=\"dashboard\">"}
    {"            @let processed_data = {"}
    {"                let mut data = analytics.to_vec();"}
    {"                data.sort_by(|a, b| b.views.cmp(&a.views));"}
    {"                data.into_iter().take(10).collect::<Vec<_>>()"}
    {"            };"}
    {"            "}
    {"            @let total_views: i32 = analytics.iter().map(|d| d.views).sum();"}
    {"            @let avg_engagement = if !analytics.is_empty() {"}
    {"                analytics.iter().map(|d| d.engagement_rate).sum::<f64>() / analytics.len() as f64"}
    {"            } else {"}
    {"                0.0"}
    {"            };"}
    {"            "}
    {"            <!-- Summary Cards -->"}
    {"            <div class=\"summary-cards\">"}
    {"                <div class=\"card\">"}
    {"                    <h3>{\"Total Views: \" total_views.to_string()}</h3>"}
    {"                </div>"}
    {"                <div class=\"card\">"}
    {"                    <h3>{\"Avg Engagement: \" format!(\"{:.1}%\", avg_engagement * 100.0)}</h3>"}
    {"                </div>"}
    {"            </div>"}
    {"            "}
    {"            <!-- Top Content -->"}
    {"            <div class=\"content-list\">"}
    {"                @for item in processed_data {"}
    {"                    @let performance_level = match item.engagement_rate {"}
    {"                        rate if rate > 0.1 => \"excellent\","}
    {"                        rate if rate > 0.05 => \"good\","}
    {"                        rate if rate > 0.02 => \"fair\","}
    {"                        _ => \"poor\","}
    {"                    };"}
    {"                    "}
    {"                    <div class={format!(\"content-item {}\", performance_level)}>"}
    {"                        <h4>{&item.title}</h4>"}
    {"                        <p>{\"Views: \" item.views.to_string() \" | Engagement: \" format!(\"{:.1}%\", item.engagement_rate * 100.0)}</p>"}
    {"                        <span class=\"badge\">{performance_level}</span>"}
    {"                    </div>"}
    {"                }"}
    {"            </div>"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🛒 Pattern 3: E-commerce Cart Logic"</h2>
                        <p>"Complex business logic with multiple data sources:"</p>

                        <div class="code-example">
                            <h4>"Shopping Cart with Discounts"</h4>
                            <pre class="code-block">{"pub fn shopping_cart(cart: &[CartItem], user: &User) -> impl azumi::Component {"}
    {"    html! {"}
    {"        <div class=\"cart-container\">"}
    {"            @let subtotal: f64 = cart.iter().map(|item| item.price * item.quantity as f64).sum();"}
    {"            @let discount_percent = {"}
    {"                match (user.membership_tier.as_str(), subtotal) {"}
    {"                    (\"premium\", _) => 0.15,"}
    {"                    (\"gold\", s) if s > 100.0 => 0.10,"}
    {"                    (\"silver\", s) if s > 200.0 => 0.05,"}
    {"                    _ => 0.0,"}
    {"                }"}
    {"            };"}
    {"            @let discount_amount = subtotal * discount_percent;"}
    {"            @let final_total = subtotal - discount_amount;"}
    {"            @let free_shipping = subtotal > 50.0 || user.membership_tier == \"premium\";"}
    {"            "}
    {"            @if !cart.is_empty() {"}
    {"                <!-- Cart Items -->"}
    {"                <div class=\"cart-items\">"}
    {"                    @for item in cart {"}
    {"                        <div class=\"cart-item\">"}
    {"                            <h4>{&item.product_name}</h4>"}
    {"                            <p>{\"$\" format!(\"{:.2}\", item.price) \" × \" item.quantity.to_string()}</p>"}
    {"                            <p class=\"item-total\">{\"$\" format!(\"{:.2}\", item.price * item.quantity as f64)}</p>"}
    {"                        </div>"}
    {"                    }"}
    {"                </div>"}
    {"                "}
    {"                <!-- Cart Summary -->"}
    {"                <div class=\"cart-summary\">"}
    {"                    <div class=\"summary-line\">"}
    {"                        <span>\"Subtotal:\"</span>"}
    {"                        <span>{\"$\" format!(\"{:.2}\", subtotal)}</span>"}
    {"                    </div>"}
    {"                    "}
    {"                    @if discount_amount > 0.0 {"}
    {"                        <div class=\"summary-line discount\">"}
    {"                            <span>{\"Discount (\" format!(\"{:.0}%\", discount_percent * 100.0) \"):\"</span>"}
    {"                            <span>{\"-$ \" format!(\"{:.2}\", discount_amount)}</span>"}
    {"                        </div>"}
    {"                    }"}
    {"                    "}
    {"                    @let shipping_cost = if free_shipping { 0.0 } else { 9.99 };"}
    {"                    <div class=\"summary-line\">"}
    {"                        <span>\"Shipping:\"</span>"}
    {"                        <span>{"}
    {"                            @if free_shipping {"}
    {"                                <span class=\"free-shipping\">\"FREE\"</span>"}
    {"                            } @else {"}
    {"                                {\"$\" format!(\"{:.2}\", shipping_cost)}"}
    {"                            }"}
    {"                        </span>"}
    {"                    </div>"}
    {"                    "}
    {"                    <div class=\"summary-line total\">"}
    {"                        <span>\"Total:\"</span>"}
    {"                        <span>{\"$\" format!(\"{:.2}\", final_total + shipping_cost)}</span>"}
    {"                    </div>"}
    {"                    "}
    {"                    <button class=\"checkout-btn\">\"Proceed to Checkout\"</button>"}
    {"                </div>"}
    {"            } @else {"}
    {"                <div class=\"empty-cart\">"}
    {"                    <h3>\"Your cart is empty\"</h3>"}
    {"                    <p>\"Add some items to get started\"</p>"}
    {"                    <a href=\"/products\" class=\"btn\">\"Browse Products\"</a>"}
    {"                </div>"}
    {"            }"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"⚡ Pattern 4: Nested Loop Performance"</h2>
                        <p>"Optimize complex nested structures for better performance:"</p>

                        <div class="performance-section">
                            <div class="performance-item">
                                <h4>"❌ Inefficient Pattern"</h4>
                                <pre class="code-block">{"@for category in categories {"}
    {"    @for product in products {"}
    {"        @if product.category_id == category.id {"}
    {"            @let avg_rating = product.reviews.iter().map(|r| r.rating).sum::<f64>() / product.reviews.len() as f64;"}
    {"            <div>{&product.name}: {format!(\"{:.1}\", avg_rating)}</div>"}
    {"        }"}
    {"    }"}
{"}"}</pre>
                                <p class="performance-note">"This performs O(n×m) operations and recalculates averages repeatedly"</p>
                            </div>

                            <div class="performance-item">
                                <h4>"✅ Optimized Pattern"</h4>
                                <pre class="code-block">{"@let processed_products = {"}
    {"    let mut product_map = std::collections::HashMap::new();"}
    {"    for product in products {"}
    {"        let avg_rating = if !product.reviews.is_empty() {"}
    {"            product.reviews.iter().map(|r| r.rating).sum::<f64>() / product.reviews.len() as f64"}
    {"        } else { 0.0 };"}
    {"        product_map.insert(product.id, (product.clone(), avg_rating));"}
    {"    }"}
    {"    product_map"}
{"};"}
{""}
{"@for category in categories {"}
    {"    <h3>{&category.name}</h3>"}
    {"    @for (_, (product, rating)) in &processed_products {"}
    {"        @if product.category_id == category.id {"}
    {"            <div>{&product.name}: {format!(\"{:.1}\", rating)}</div>"}
    {"        }"}
    {"    }"}
{"}"}</pre>
                                <p class="performance-note">"Pre-processes data once, reducing complexity and improving performance"</p>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"🎨 Pattern 5: Dynamic UI Composition"</h2>
                        <p>"Build flexible user interfaces with dynamic component selection:"</p>

                        <div class="code-example">
                            <h4>"Widget System"</h4>
                            <pre class="code-block">{"pub fn dashboard_widgets(user: &User, widgets: &[WidgetConfig]) -> impl azumi::Component {"}
    {"    html! {"}
    {"        <div class=\"dashboard-grid\">"}
    {"            @let visible_widgets = widgets.iter().filter(|w| w.is_enabled).collect::<Vec<_>>();"}
    {"            "}
    {"            @for widget in visible_widgets {"}
    {"                @let widget_class = match (widget.size, widget.position) {"}
    {"                    (\"large\", _) => \"widget-large\","}
    {"                    (\"medium\", pos) if pos % 3 == 0 => \"widget-medium widget-wide\","}
    {"                    (\"small\", _) => \"widget-small\","}
    {"                    _ => \"widget-medium\","}
    {"                };"}
    {"                @let show_header = !widget.title.is_empty();"}
    {"                "}
    {"                <div class={widget_class}>"}
    {"                    @if show_header {"}
    {"                        <div class=\"widget-header\">"}
    {"                            <h3>{&widget.title}</h3>"}
    {"                            @if user.has_permission(\"widget:configure\") {"}
    {"                                <button class=\"widget-settings\">⚙️</button>"}
    {"                            }"}
    {"                        </div>"}
    {"                    }"}
    {"                    "}
    {"                    <div class=\"widget-content\">"}
    {"                        @match widget.widget_type {"}
    {"                            \"chart\" => { <ChartComponent data=&widget.data /> }"}
    {"                            \"table\" => { <TableComponent data=&widget.data /> }"}
    {"                            \"list\" => { <ListComponent data=&widget.data /> }"}
    {"                            \"stats\" => { <StatsComponent data=&widget.data /> }"}
    {"                            _ => { <div>\"Unknown widget type\"</div> }"}
    {"                        }"}
    {"                    </div>"}
    {"                </div>"}
    {"            }"}
    {"        </div>"}
    {"    }"}
    {"}"}</pre>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"📋 Pattern 6: Form Validation & Processing"</h2>
                        <p>"Complex form handling with validation and error states:"</p>

                        <div class="code-example">
                            <h4>"Multi-step Form"</h4>
                            <pre class="code-block">{"pub fn multi_step_form(form_data: &FormData, errors: &[ValidationError]) -> impl azumi::Component {"}
    {"    html! {"}
    {"        <form class=\"multi-step-form\">"}
    {"            @let current_step = form_data.current_step;"}
    {"            @let step_errors = errors.iter().filter(|e| e.step == current_step).collect::<Vec<_>>();"}
    {"            @let is_first_step = current_step == 1;"}
    {"            @let is_last_step = current_step == form_data.total_steps;"}
    {"            @let can_proceed = step_errors.is_empty();"}
    {"            "}
    {"            <!-- Progress Indicator -->"}
    {"            <div class=\"form-progress\">"}
    {"                @for step in 1..=form_data.total_steps {"}
    {"                    <div class={format!(\"progress-step {}\", {"}
    {"                        if step < current_step { \"completed\" }"}
    {"                        else if step == current_step { \"active\" }"}
    {"                        else { \"pending\" }"}
    {"                    })}>"}
    {"                        <span class=\"step-number\">{step.to_string()}</span>"}
    {"                        <span class=\"step-label\">"}
    {"                            @match step {"}
    {"                                1 => \"Personal Info\","}
    {"                                2 => \"Address\","}
    {"                                3 => \"Payment\","}
    {"                                _ => \"Review\","}
    {"                            }"}
    {"                        </span>"}
    {"                    </div>"}
    {"                    @if step < form_data.total_steps {"}
    {"                        <div class=\"progress-connector\"></div>"}
    {"                    }"}
    {"                }"}
    {"            </div>"}
    {"            "}
    {"            <!-- Form Fields -->"}
    {"            <div class=\"form-step-content\">"}
    {"                @match current_step {"}
    {"                    1 => {"}
    {"                        @let name_error = step_errors.iter().find(|e| e.field == \"name\");"}
    {"                        <div class={format!(\"form-group {}\", name_error.map(|_| \"error\").unwrap_or(\"\"))}>"}
    {"                            <label>\"Full Name\"</label>"}
    {"                            <input type=\"text\" value={&form_data.name} />"}
    {"                            @if let Some(error) = name_error {"}
    {"                                <span class=\"error-message\">{&error.message}</span>"}
    {"                            }"}
    {"                        </div>"}
    {"                        <!-- More step 1 fields -->"}
    {"                    }"}
    {"                    // ... other steps"}
    {"                    _ => <div>\"Unknown step\"</div>"}
    {"                }"}
    {"            </div>"}
    {"            "}
    {"            <!-- Navigation Buttons -->"}
    {"            <div class=\"form-navigation\">"}
    {"                @if !is_first_step {"}
    {"                    <button type=\"button\" class=\"btn-secondary\">\"Previous\"</button>"}
    {"                }"}
    {"                "}
    {"                @if is_last_step {"}
    {"                    <button type=\"submit\" class=\"btn-primary\" disabled={!can_proceed}>\"Submit\"</button>"}
    {"                } @else {"}
    {"                    <button type=\"button\" class=\"btn-primary\" disabled={!can_proceed}>\"Next\"</button>"}
    {"                }"}
    {"            </div>"}
    {"        </form>"}
    {"    }"}
    {"}"}</pre>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"⚠️ Common Anti-Patterns"</h2>
                        <p>"Avoid these common mistakes in complex control flow:"</p>

                        <div class="anti-patterns">
                            <div class="anti-pattern-item">
                                <h4>"🔄 Over-nesting"</h4>
                                <p>"Deeply nested control structures become hard to read and maintain"</p>
                                <pre class="code-block">{"@if condition1 {"}
    {"    @if condition2 {"}
    {"        @if condition3 {"}
    {"            // Complex nested logic"}
    {"        }"}
    {"    }"}
{"}"}</pre>
                            </div>

                            <div class="anti-pattern-item">
                                <h4>"🧮 Heavy Computations"</h4>
                                <p>"Complex calculations should be done in Rust, not templates"</p>
                                <pre class="code-block">{"@let result = {"}
    {"    // Complex business logic in template"}
    {"    heavy_calculation(data1, data2, data3)"}
{"};"}</pre>
                            </div>

                            <div class="anti-pattern-item">
                                <h4>"🔁 Duplicate Logic"</h4>
                                <p>"Repeated similar patterns should be extracted to functions"</p>
                                <pre class="code-block">{"// Instead of repeating this pattern"}
    {"@if !items.is_empty() {"}
    {"    @for item in items {"}
    {"        // Process item"}
    {"    }"}
{"}"}</pre>
                            </div>
                        </div>
                    </section>

                    <section class="section">
                        <h2 class="section-title">"📝 Practice Exercise"</h2>
                        <div class="exercise-box">
                            <h4>"🏋️ Build a Notification Center"</h4>
                            <p>"Create a comprehensive notification system using all control flow patterns:"</p>
                            
                            <div class="exercise-requirements">
                                <h5>"Requirements:"</h5>
                                <ul>
                                    <li>"Filter notifications by type (info, warning, error, success)"</li>
                                    <li>"Group notifications by date/time"</li>
                                    <li>"Show unread count and priority indicators"</li>
                                    <li>"Allow bulk actions (mark all read, delete selected)"</li>
                                    <li>"Support different views (list, grouped, timeline)"</li>
                                </ul>
                            </div>

                            <div class="exercise-instructions">
                                <h5>"Success Criteria:"</h5>
                                <ul>
                                    <li>"Use @let for computed values (counts, grouping)"</li>
                                    <li>"Use @if for conditional rendering (empty states, permissions)"</li>
                                    <li>"Use @for for displaying notification lists"</li>
                                    <li>"Use @match for notification types and actions"</li>
                                    <li>"Maintain good performance with large datasets"</li>
                                </ul>
                            </div>
                        </div>
                    </section>

                    <nav class="nav-buttons">
                        <a href="/lesson-9" class="btn btn-secondary">"← Previous: Local Variables"</a>
                        <a href="/lesson-11" class="btn">"Next: Introduction to Components →"</a>
                    </nav>
                </div>
            </body>
        </html>
    }
}

pub async fn lesson10_handler() -> impl IntoResponse {
    Html(azumi::render_to_string(&lesson10()))
}