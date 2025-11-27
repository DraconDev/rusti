//! Lesson 15: reusable_form_component.rs
//!
//! Building flexible form components
use azumi::html;

/// Input field component
#[derive(Clone)]
pub struct InputFieldProps {
    pub field_type: String,
    pub name: String,
    pub label: String,
    pub value: String,
    pub placeholder: Option<String>,
    pub required: bool,
}

pub fn input_field(props: InputFieldProps) -> impl azumi::Component {
    html! {
        <div class="input-field">
            <label for={&props.name}>{&props.label}</label>
            <input 
                type={&props.field_type}
                id={&props.name}
                name={&props.name}
                value={&props.value}
                placeholder={props.placeholder}
                required={props.required} />
        </div>
    }
}

/// Select field component
#[derive(Clone)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

#[derive(Clone)]
pub struct SelectFieldProps {
    pub name: String,
    pub label: String,
    pub options: Vec<SelectOption>,
    pub selected_value: String,
    pub required: bool,
}

pub fn select_field(props: SelectFieldProps) -> impl azumi::Component {
    html! {
        <div class="select-field">
            <label for={&props.name}>{&props.label}</label>
            <select 
                id={&props.name}
                name={&props.name}
                required={props.required}>
                @for option in &props.options {
                    <option 
                        value={&option.value}
                        selected={option.value == props.selected_value}>
                        {&option.label}
                    </option>
                }
            </select>
        </div>
    }
}

/// Textarea component
#[derive(Clone)]
pub struct TextareaProps {
    pub name: String,
    pub label: String,
    pub value: String,
    pub rows: usize,
    pub placeholder: Option<String>,
}

pub fn textarea_field(props: TextareaProps) -> impl azumi::Component {
    html! {
        <div class="textarea-field">
            <label for={&props.name}>{&props.label}</label>
            <textarea 
                id={&props.name}
                name={&props.name}
                rows={props.rows.to_string()}
                placeholder={props.placeholder}>
                {&props.value}
            </textarea>
        </div>
    }
}

/// Checkbox component
#[derive(Clone)]
pub struct CheckboxProps {
    pub name: String,
    pub label: String,
    pub checked: bool,
    pub value: String,
}

pub fn checkbox_field(props: CheckboxProps) -> impl azumi::Component {
    html! {
        <div class="checkbox-field">
            <label>
                <input 
                    type="checkbox"
                    name={&props.name}
                    value={&props.value}
                    checked={props.checked} />
                {&props.label}
            </label>
        </div>
    }
}

/// Radio group component
#[derive(Clone)]
pub struct RadioOption {
    pub value: String,
    pub label: String,
}

#[derive(Clone)]
pub struct RadioGroupProps {
    pub name: String,
    pub label: String,
    pub options: Vec<RadioOption>,
    pub selected_value: String,
}

pub fn radio_group(props: RadioGroupProps) -> impl azumi::Component {
    html! {
        <div class="radio-group">
            <fieldset>
                <legend>{&props.label}</legend>
                @for option in &props.options {
                    <label class="radio-label">
                        <input 
                            type="radio"
                            name={&props.name}
                            value={&option.value}
                            checked={option.value == props.selected_value} />
                        {&option.label}
                    </label>
                }
            </fieldset>
        </div>
    }
}

/// Complete form component
#[derive(Clone)]
pub struct ContactForm {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub subject: String,
    pub message: String,
    pub contact_preference: String,
    pub newsletter_signup: bool,
    pub terms_accepted: bool,
}

pub fn contact_form(form_data: &ContactForm) -> impl azumi::Component {
    let subjects = vec![
        SelectOption {
            value: "general".to_string(),
            label: "General Inquiry".to_string(),
        },
        SelectOption {
            value: "support".to_string(),
            label: "Technical Support".to_string(),
        },
        SelectOption {
            value: "sales".to_string(),
            label: "Sales".to_string(),
        },
        SelectOption {
            value: "feedback".to_string(),
            label: "Feedback".to_string(),
        },
    ];

    let contact_preferences = vec![
        RadioOption {
            value: "email".to_string(),
            label: "Email".to_string(),
        },
        RadioOption {
            value: "phone".to_string(),
            label: "Phone".to_string(),
        },
        RadioOption {
            value: "both".to_string(),
            label: "Both Email and Phone".to_string(),
        },
    ];

    html! {
        <style src="/static/pages/lesson15.css" />
        <div class="form-demo">
            <h1>"Contact Form with Reusable Components"</h1>
            
            <form class="contact-form" action="/contact" method="POST">
                <h2>"Get in Touch"</h2>
                
                <div class="form-row">
                    @input_field(InputFieldProps {
                        field_type: "text".to_string(),
                        name: "name".to_string(),
                        label: "Full Name".to_string(),
                        value: form_data.name.clone(),
                        placeholder: Some("Enter your full name".to_string()),
                        required: true,
                    })
                    
                    @input_field(InputFieldProps {
                        field_type: "email".to_string(),
                        name: "email".to_string(),
                        label: "Email Address".to_string(),
                        value: form_data.email.clone(),
                        placeholder: Some("your.email@example.com".to_string()),
                        required: true,
                    })
                </div>

                <div class="form-row">
                    @input_field(InputFieldProps {
                        field_type: "tel".to_string(),
                        name: "phone".to_string(),
                        label: "Phone Number".to_string(),
                        value: form_data.phone.clone(),
                        placeholder: Some("+1 (555) 123-4567".to_string()),
                        required: false,
                    })
                    
                    @select_field(SelectFieldProps {
                        name: "subject".to_string(),
                        label: "Subject".to_string(),
                        options: subjects,
                        selected_value: form_data.subject.clone(),
                        required: true,
                    })
                </div>

                @textarea_field(TextareaProps {
                    name: "message".to_string(),
                    label: "Message".to_string(),
                    value: form_data.message.clone(),
                    rows: 5,
                    placeholder: Some("Tell us more about your inquiry...".to_string()),
                })

                @radio_group(RadioGroupProps {
                    name: "contact_preference".to_string(),
                    label: "How would you like us to contact you?".to_string(),
                    options: contact_preferences,
                    selected_value: form_data.contact_preference.clone(),
                })

                <div class="form-checkboxes">
                    @checkbox_field(CheckboxProps {
                        name: "newsletter".to_string(),
                        label: "Subscribe to our newsletter".to_string(),
                        checked: form_data.newsletter_signup,
                        value: "yes".to_string(),
                    })
                    
                    @checkbox_field(CheckboxProps {
                        name: "terms".to_string(),
                        label: "I agree to the Terms of Service".to_string(),
                        checked: form_data.terms_accepted,
                        value: "accepted".to_string(),
                    })
                </div>

                <div class="form-actions">
                    <button type="submit" class="btn btn-primary">"Send Message"</button>
                    <button type="reset" class="btn btn-secondary">"Reset Form"</button>
                </div>
            </form>
        </div>
    }
}

/// Advanced form with validation states
pub fn advanced_form_demo() -> impl azumi::Component {
    let filled_form = ContactForm {
        name: "Alice Johnson".to_string(),
        email: "alice@example.com".to_string(),
        phone: "+1 (555) 123-4567".to_string(),
        subject: "support".to_string(),
        message: "I'm having trouble with the login system. Can you help me?".to_string(),
        contact_preference: "email".to_string(),
        newsletter_signup: true,
        terms_accepted: false,
    };

    contact_form(&filled_form)
}

/// Handler for Axum
pub async fn lesson15_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&advanced_form_demo()))
}