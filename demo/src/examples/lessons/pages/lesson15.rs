//! Lesson 15: Reusable Form Components
//!
//! Building flexible, type-safe form components

use azumi::html;

// ============================================================================
// SECTION 1: Input Field Component
// ============================================================================

#[derive(Clone, Debug)]
pub struct InputProps {
    pub label: String,
    pub name: String,
    pub input_type: String,
    pub placeholder: String,
    pub required: bool,
}

#[azumi::component]
pub fn input_field<'a>(
    label: &'a str,
    name: &'a str,
    input_type: &'a str,
    placeholder: &'a str,
    required: bool,
) -> impl azumi::Component + 'a {
    html! {
        <style src="/static/pages/lesson15.css" />
        <div class="form-group">
            <label class="form-label" for={name}>
                {label}
                @if required {
                    <span class="required-mark">" *"</span>
                }
            </label>
            <input
                type={input_type}
                name={name}
                id={name}
                class="form-input"
                placeholder={placeholder}
                required={required}
            />
        </div>
    }
}

// ============================================================================
// SECTION 2: Textarea Component
// ============================================================================

#[derive(Clone, Debug)]
pub struct TextareaProps {
    pub label: String,
    pub name: String,
    pub placeholder: String,
    pub rows: u32,
    pub required: bool,
}

#[azumi::component]
pub fn textarea_field<'a>(
    label: &'a str,
    name: &'a str,
    placeholder: &'a str,
    rows: u32,
    required: bool,
) -> impl azumi::Component + 'a {
    html! {
        <style src="/static/pages/lesson15.css" />
        <div class="form-group">
            <label class="form-label" for={name}>
                {label}
                @if required {
                    <span class="required-mark">" *"</span>
                }
            </label>
            <textarea
                name={name}
                id={name}
                class="form-textarea"
                placeholder={placeholder}
                rows={rows.to_string()}
                required={required}
            ></textarea>
        </div>
    }
}

// ============================================================================
// SECTION 3: Select Component
// ============================================================================

#[derive(Clone, Debug)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

#[derive(Clone, Debug)]
pub struct SelectProps {
    pub label: String,
    pub name: String,
    pub options: Vec<SelectOption>,
    pub required: bool,
}

#[azumi::component]
pub fn select_field<'a>(
    label: &'a str,
    name: &'a str,
    options: &'a [SelectOption],
    required: bool,
) -> impl azumi::Component + 'a {
    html! {
        <style src="/static/pages/lesson15.css" />
        <div class="form-group">
            <label class="form-label" for={name}>
                {label}
                @if required {
                    <span class="required-mark">" *"</span>
                }
            </label>
            <select
                name={name}
                id={name}
                class="form-select"
                required={required}
            >
                <option value="">"-- Select an option --"</option>
                @for option in options {
                    <option value={&option.value}>{&option.label}</option>
                }
            </select>
        </div>
    }
}

// ============================================================================
// SECTION 4: Checkbox Component
// ============================================================================

#[derive(Clone, Debug)]
pub struct CheckboxProps {
    pub label: String,
    pub name: String,
    pub checked: bool,
}

#[azumi::component]
pub fn checkbox_field<'a>(
    label: &'a str,
    name: &'a str,
    checked: bool,
) -> impl azumi::Component + 'a {
    html! {
        <style src="/static/pages/lesson15.css" />
        <div class="form-group-inline">
            <input
                type="checkbox"
                name={name}
                id={name}
                class="form-checkbox"
                checked={checked}
            />
            <label class="form-label-inline" for={name}>
                {label}
            </label>
        </div>
    }
}

// ============================================================================
// SECTION 5: Radio Group Component
// ============================================================================

#[derive(Clone, Debug)]
pub struct RadioOption {
    pub value: String,
    pub label: String,
}

#[derive(Clone, Debug)]
pub struct RadioGroupProps {
    pub label: String,
    pub name: String,
    pub options: Vec<RadioOption>,
    pub required: bool,
}

#[azumi::component]
pub fn radio_group<'a>(
    label: &'a str,
    name: &'a str,
    options: &'a [RadioOption],
    required: bool,
) -> impl azumi::Component + 'a {
    html! {
        <style src="/static/pages/lesson15.css" />
        <div class="form-group">
            <label class="form-label">
                {label}
                @if required {
                    <span class="required-mark">" *"</span>
                }
            </label>
            <div class="radio-group">
                @for option in options {
                    <div class="radio-item">
                        <input
                            type="radio"
                            name={name}
                            id={format!("{}_{}", name, option.value)}
                            value={&option.value}
                            class="form-radio"
                            required={required}
                        />
                        <label class="form-label-inline" for={format!("{}_{}", name, option.value)}>
                            {&option.label}
                        </label>
                    </div>
                }
            </div>
        </div>
    }
}

// ============================================================================
// SECTION 6: Complete Form Example
// ============================================================================

#[azumi::component]
pub fn contact_form_demo() -> impl azumi::Component {
    let countries = vec![
        SelectOption {
            value: "us".to_string(),
            label: "United States".to_string(),
        },
        SelectOption {
            value: "uk".to_string(),
            label: "United Kingdom".to_string(),
        },
        SelectOption {
            value: "ca".to_string(),
            label: "Canada".to_string(),
        },
        SelectOption {
            value: "au".to_string(),
            label: "Australia".to_string(),
        },
    ];

    let contact_methods = vec![
        RadioOption {
            value: "email".to_string(),
            label: "Email".to_string(),
        },
        RadioOption {
            value: "phone".to_string(),
            label: "Phone".to_string(),
        },
        RadioOption {
            value: "mail".to_string(),
            label: "Mail".to_string(),
        },
    ];

    html! {
        <style src="/static/pages/lesson15.css" />
        <div class="lesson-container">
            <header class="lesson-header">
                <h1 class="lesson-title">"Lesson 15: Reusable Form Components"</h1>
                <p class="lesson-subtitle">
                    "Build type-safe, accessible form components with validation"
                </p>
            </header>

            <section class="lesson-section">
                <h2 class="section-title">"Introduction"</h2>
                <p class="section-text">
                    "Forms are essential for user interaction. This lesson teaches you how to create "
                    "reusable form components that are accessible, type-safe, and easy to compose."
                </p>
            </section>

            <section class="lesson-section">
                <h2 class="section-title">"Individual Form Components"</h2>

                <div class="demo-grid">
                    <div class="demo-column">
                        <h3 class="demo-heading">"Text Input"</h3>
                        {input_field(&InputProps {
                            label: "Email Address".to_string(),
                            name: "email_demo".to_string(),
                            input_type: "email".to_string(),
                            placeholder: "you@example.com".to_string(),
                            required: true,
                        })}
                    </div>

                    <div class="demo-column">
                        <h3 class="demo-heading">"Select Dropdown"</h3>
                    {select_field(&SelectProps {
                        label: "Country".to_string(),
                        name: "country_demo".to_string(),
                        options: countries.clone(),
                        required: false,
                    })}
                    </div>
                </div>

                <div class="demo-grid">
                    <div class="demo-column">
                        <h3 class="demo-heading">"Checkbox"</h3>
                        {checkbox_field(&CheckboxProps {
                            label: "Subscribe to newsletter".to_string(),
                            name: "newsletter_demo".to_string(),
                            checked: false,
                        })}
                    </div>

                    <div class="demo-column">
                        <h3 class="demo-heading">"Textarea"</h3>
                        {textarea_field(&TextareaProps {
                            label: "Additional Notes".to_string(),
                            name: "notes_demo".to_string(),
                            placeholder: "Enter any additional information...".to_string(),
                            rows: 4,
                            required: false,
                        })}
                    </div>
                </div>
            </section>

            <section class="lesson-section">
                <h2 class="section-title">"Complete Contact Form"</h2>
                <p class="section-text">
                    "Combining multiple form components to create a complete contact form."
                </p>

                <form class="contact-form">
                    {input_field(&InputProps {
                        label: "Full Name".to_string(),
                        name: "full_name".to_string(),
                        input_type: "text".to_string(),
                        placeholder: "John Doe".to_string(),
                        required: true,
                    })}

                    {input_field(&InputProps {
                        label: "Email Address".to_string(),
                        name: "email".to_string(),
                        input_type: "email".to_string(),
                        placeholder: "john@example.com".to_string(),
                        required: true,
                    })}

                    {input_field(&InputProps {
                        label: "Phone Number".to_string(),
                        name: "phone".to_string(),
                        input_type: "tel".to_string(),
                        placeholder: "+1 (555) 123-4567".to_string(),
                        required: false,
                    })}

                    {select_field(&SelectProps {
                        label: "Country".to_string(),
                        name: "country".to_string(),
                        options: countries.clone(),
                        required: true,
                    })}

                    {radio_group(&RadioGroupProps {
                        label: "Preferred Contact Method".to_string(),
                        name: "contact_method".to_string(),
                        options: contact_methods.clone(),
                        required: true,
                    })}

                    {textarea_field(&TextareaProps {
                        label: "Message".to_string(),
                        name: "message".to_string(),
                        placeholder: "Tell us more about your inquiry...".to_string(),
                        rows: 6,
                        required: true,
                    })}

                    {checkbox_field(&CheckboxProps {
                        label: "I agree to the privacy policy".to_string(),
                        name: "privacy_agree".to_string(),
                        checked: false,
                    })}

                    <div class="form-actions">
                        <button type="submit" class="btn-submit">"Submit Form"</button>
                        <button type="reset" class="btn-reset">"Reset"</button>
                    </div>
                </form>
            </section>

            <section class="lesson-section">
                <h2 class="section-title">"Key Takeaways"</h2>
                <ul class="takeaways-list">
                    <li>"✓ Create reusable form components with clear Props structures"</li>
                    <li>"✓ Use proper input types for better UX and validation"</li>
                    <li>"✓ Always include labels for accessibility"</li>
                    <li>"✓ Mark required fields visually"</li>
                    <li>"✓ Use `for` attribute to link labels with inputs"</li>
                    <li>"✓ Compose complex forms from simple building blocks"</li>
                    <li>"✓ Remember to import CSS and use `+ '_` for borrowed data"</li>
                </ul>
            </section>
        </div>
    }
}

/// Axum handler for Lesson 15
pub async fn lesson15_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&contact_form_demo()))
}
