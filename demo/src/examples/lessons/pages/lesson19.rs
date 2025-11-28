//! Lesson 19: Accessibility Patterns
//!
//! Best practices for building accessible web applications

use azumi::html;

#[azumi::component]
pub fn accessibility_patterns_demo() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson19.css" />
        <div class="lesson19-container">
            <header class="lesson19-header">
                <h1 class="lesson19-title">"Lesson 19: Accessibility Patterns"</h1>
                <p class="lesson19-subtitle">
                    "Building inclusive web experiences with ARIA and semantic HTML"
                </p>
            </header>

            // Skip to main content link
            <a href="#main-content" class="skip-link">"Skip to main content"</a>

            <nav aria-label="Primary navigation" class="main-nav">
                <ul class="nav-list">
                    <li><a href="/" aria-current="page">"Home"</a></li>
                    <li><a href="/about">"About"</a></li>
                    <li><a href="/contact">"Contact"</a></li>
                </ul>
            </nav>

            <main id="main-content" class="lesson19-main">
                <section class="lesson19-section">
                    <h2 class="section-title">"Semantic HTML Landmarks"</h2>
                    <p class="section-text">
                        "Use proper semantic elements with ARIA labels for screen readers:"
                    </p>

                    <div class="example-container">
                        <div class="example-code">
                            <pre>
    "<nav aria-label=\"Primary navigation\">\n"
    "<main id=\"main-content\">\n"
    "<aside aria-labelledby=\"sidebar-title\">\n"
    "<footer role=\"contentinfo\">"
                            </pre>
                        </div>
                    </div>
                </section>

                <section class="lesson19-section">
                    <h2 class="section-title">"Form Accessibility"</h2>

                    <form class="accessible-form" aria-labelledby="form-title">
                        <h3 id="form-title">"Contact Form"</h3>

                        <div class="form-group">
                            <label for="full-name" class="form-label">
                                "Full Name"
                                <span class="required" aria-label="required">"*"</span>
                            </label>
                            <input
                                type="text"
                                id="full-name"
                                name="full_name"
                                class="form-input"
                                required=true
                                aria-required="true"
                                aria-describedby="name-hint"
                            />
                            <small id="name-hint" class="form-hint">
                                "Please enter your full legal name"
                            </small>
                        </div>

                        <div class="form-group">
                            <label for="email-address" class="form-label">
                                "Email"
                                <span class="required" aria-label="required">"*"</span>
                            </label>
                            <input
                                type="email"
                                id="email-address"
                                name="email"
                                class="form-input"
                                required=true
                                aria-required="true"
                                aria-invalid="false"
                            />
                        </div>

                        <div class="form-group">
                            <fieldset class="radio-fieldset">
                                <legend class="fieldset-legend">"Preferred Contact Method"</legend>
                                <div class="radio-group">
                                    <div class="radio-item">
                                        <input
                                            type="radio"
                                            id="contact-email"
                                            name="contact_method"
                                            value="email"
                                            class="radio-input"
                                        />
                                        <label for="contact-email" class="radio-label">"Email"</label>
                                    </div>
                                    <div class="radio-item">
                                        <input
                                            type="radio"
                                            id="contact-phone"
                                            name="contact_method"
                                            value="phone"
                                            class="radio-input"
                                        />
                                        <label for="contact-phone" class="radio-label">"Phone"</label>
                                    </div>
                                </div>
                            </fieldset>
                        </div>

                        <button type="submit" class="submit-btn" aria-label="Submit contact form">
                            "Submit"
                        </button>
                    </form>
                </section>

                <section class="lesson19-section">
                    <h2 class="section-title">"Interactive Components with ARIA"</h2>

                    <div class="dialog-demo">
                        <button
                            class="open-dialog-btn"
                            aria-haspopup="dialog"
                            aria-expanded="false"
                        >
                            "Open Dialog"
                        </button>

                        <div
                            role="dialog"
                            aria-labelledby="dialog-title"
                            aria-describedby="dialog-desc"
                            class="dialog hidden"
                            aria-hidden="true"
                        >
                            <h3 id="dialog-title">"Confirmation"</h3>
                            <p id="dialog-desc">
                                "Are you sure you want to proceed?"
                            </p>
                            <div class="dialog-actions">
                                <button class="btn-secondary">"Cancel"</button>
                                <button class="btn-primary">"Confirm"</button>
                            </div>
                        </div>
                    </div>
                </section>

                <section class="lesson19-section">
                    <h2 class="section-title">"Status Messages & Live Regions"</h2>

                    <div class="status-demo">
                        <button class="action-btn">"Save Changes"</button>

                        <div
                            role="status"
                            aria-live="polite"
                            aria-atomic="true"
                            class="status-message success"
                        >
                            <span class="status-icon">"✓"</span>
                            "Changes saved successfully!"
                        </div>

                        <div
                            role="alert"
                            aria-live="assertive"
                            class="status-message error hidden"
                        >
                            <span class="status-icon">"⚠"</span>
                            "Error: Unable to save changes"
                        </div>
                    </div>
                </section>

                <section class="lesson19-section">
                    <h2 class="section-title">"Focus Management"</h2>

                    <div class="focus-demo">
                        <p>
                            "Proper focus indicators are essential for keyboard navigation:"
                        </p>

                        <div class="button-group">
                            <button class="focus-btn">"Button 1"</button>
                            <button class="focus-btn">"Button 2"</button>
                            <button class="focus-btn">"Button 3"</button>
                        </div>

                        <p class="hint">
                            "Try tabbing through these buttons - notice the clear focus ring"
                        </p>
                    </div>
                </section>

                <section class="lesson19-section">
                    <h2 class="section-title">"Image Accessibility"</h2>

                    <div class="image-examples">
                        <figure class="image-example">
                            <img
                                src="/static/decorative-divider.svg"
                                alt=""
                                role="presentation"
                                class="decorative-img"
                            />
                            <figcaption>"Decorative image (empty alt)"</figcaption>
                        </figure>

                        <figure class="image-example">
                            <img
                                src="/static/chart-example.png"
                                alt="Bar chart showing 75% increase in user engagement over 6 months"
                                class="content-img"
                            />
                            <figcaption>"Informative image (descriptive alt)"</figcaption>
                        </figure>
                    </div>
                </section>

                <section class="lesson19-section">
                    <h2 class="section-title">"Key Takeaways"</h2>
                    <ul class="takeaways-list">
                        <li>"✓ Use semantic HTML5 elements (nav, main, aside, footer)"</li>
                        <li>"✓ Provide aria-label for landmarks and complex widgets"</li>
                        <li>"✓ Ensure all form inputs have associated labels"</li>
                        <li>"✓ Use aria-describedby for help text and hints"</li>
                        <li>"✓ Mark required fields with aria-required"</li>
                        <li>"✓ Use role=status/alert for dynamic content"</li>
                        <li>"✓ Maintain visible focus indicators"</li>
                        <li>"✓ Provide descriptive alt text for informative images"</li>
                        <li>"✓ Use empty alt for decorative images"</li>
                        <li>"✓ Test with screen readers (NVDA, JAWS, VoiceOver)"</li>
                    </ul>
                </section>
            </main>

            <aside aria-labelledby="sidebar-title" class="sidebar">
                <h2 id="sidebar-title" class="sidebar-title">"Resources"</h2>
                <ul class="resource-list">
                    <li><a href="https://www.w3.org/WAI/WCAG21/quickref/">"WCAG 2.1 Guidelines"</a></li>
                    <li><a href="https://www.w3.org/TR/wai-aria-practices/">"ARIA Authoring Practices"</a></li>
                    <li><a href="https://webaim.org/">"WebAIM Resources"</a></li>
                </ul>
            </aside>

            <footer role="contentinfo" class="lesson19-footer">
                <p>"Built with Azumi - Accessible by default"</p>
            </footer>
        </div>
    }
}

/// Axum handler for Lesson 19
pub async fn lesson19_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(
        &html! { @accessibility_patterns_demo() },
    ))
}
