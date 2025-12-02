use azumi::html;

/// Lesson 10: Accessibility Patterns
///
/// Accessibility-validated components
#[azumi::component]
pub fn accessible_card<'a>(
    title: &'a str,
    description: &'a str,
    image_url: &'a str,
    alt_text: &'a str,
) -> impl azumi::Component + 'a {
    html! {
        <style>
            .card { max-width: "300px"; border: "1px solid #ddd"; border-radius: "8px"; overflow: "hidden"; }
            .card_image { width: "100%"; height: "200px"; object-fit: "cover"; }
            .card_content { padding: "1rem"; }
            .card_title { font-size: "1.2rem"; margin-bottom: "0.5rem"; }
            .card_description { color: "#666"; }
            #card_title { margin: "0"; }
            #card_desc { margin: "0"; }
        </style>
        <article class={card} aria-labelledby="card_title" aria-describedby="card_desc">
            <img class={card_image} src={image_url} alt={alt_text} />
            <div class={card_content}>
                <h3 id="card_title" class={card_title}>{title}</h3>
                <p id="card_desc" class={card_description}>{description}</p>
            </div>
        </article>
    }
}

/// Example: Accessible form with proper labels
#[azumi::component]
pub fn accessible_form() -> impl azumi::Component {
    html! {
        <style>
            .accessible_form { display: "grid"; gap: "1rem"; max-width: "400px"; }
            .form_group { display: "grid"; gap: "0.5rem"; }
            .form_label { font-weight: "bold"; }
            .form_input { padding: "0.5rem"; border: "1px solid #ddd"; }
            .form_button { padding: "0.75rem"; background: "#2196f3"; color: "white"; border: "none"; cursor: "pointer"; }
            #form_title { margin: "0"; }
            #username { margin: "0"; }
            #password { margin: "0"; }
        </style>
        <form class={accessible_form} aria-labelledby="form_title">
            <h2 id="form_title">"Accessible Form"</h2>

            <div class={form_group}>
                <label class={form_label} for="username">"Username"</label>
                <input class={form_input} type="text" id="username" name="username" required aria-required="true" />
            </div>

            <div class={form_group}>
                <label class={form_label} for="password">"Password"</label>
                <input class={form_input} type="password" id="password" name="password" required aria-required="true" />
            </div>

            <button class={form_button} type="submit">"Submit"</button>
        </form>
    }
}

/// Example: Accessible navigation
#[azumi::component]
pub fn accessible_navigation() -> impl azumi::Component {
    html! {
        <style>
            .nav_container { padding: "1rem"; }
            .nav_list { list-style: "none"; padding: "0"; display: "grid"; gap: "0.5rem"; }
            .nav_item { padding: "0.5rem"; }
            .nav_link { color: "#2196f3"; text-decoration: "none"; }
            .nav_link:hover { text-decoration: "underline"; }
            #nav_heading { margin: "0"; }
        </style>
        <nav class={nav_container} aria-label="Main navigation">
            <h3 id="nav_heading">"Site Navigation"</h3>
            <ul class={nav_list} aria-labelledby="nav_heading">
                <li class={nav_item}>
                    <a class={nav_link} href="#home" aria-current="page">"Home"</a>
                </li>
                <li class={nav_item}>
                    <a class={nav_link} href="#about">"About"</a>
                </li>
                <li class={nav_item}>
                    <a class={nav_link} href="#contact">"Contact"</a>
                </li>
            </ul>
        </nav>
    }
}

/// Main lesson demonstration component
#[azumi::component]
pub fn lesson10() -> impl azumi::Component {
    html! {
        <style>
            .container { padding: "20px"; }
            .header { text-align: "center"; margin-bottom: "30px"; }
            .main_title { font-size: "32px"; color: "#333"; }
            .subtitle { font-size: "18px"; color: "#666"; }
            .key_points { background: "#f9f9f9"; padding: "20px"; border-radius: "8px"; margin-bottom: "30px"; }
            .section_title { font-size: "20px"; margin-bottom: "15px"; }
            .points_list { list-style: "none"; padding: "0"; }
            .point { margin-bottom: "10px"; }
            .examples { display: "grid"; gap: "20px"; }
            .example_card { border: "1px solid #ddd"; padding: "20px"; border-radius: "8px"; }
        </style>
        <div class={container}>
            <header class={header}>
                <h1 class={main_title}>"Lesson 10: Accessibility Patterns"</h1>
                <p class={subtitle}>"Accessibility-validated components"</p>
            </header>

            <section class={key_points}>
                <h2 class={section_title}>"Key Concepts"</h2>
                <ul class={points_list}>
                    <li class={point}>"✅ Proper ARIA attributes"</li>
                    <li class={point}>"✅ Semantic HTML structure"</li>
                    <li class={point}>"✅ Accessible form controls"</li>
                    <li class={point}>"✅ Keyboard navigation support"</li>
                    <li class={point}>"✅ Screen reader compatibility"</li>
                </ul>
            </section>

            <section class={examples}>
                <div class={example_card}>
                    @accessible_card(
                        title="Accessible Card",
                        description="This card demonstrates proper accessibility attributes",
                        image_url="https://via.placeholder.com/300x200",
                        alt_text="Placeholder image showing accessibility features"
                    )
                </div>
                <div class={example_card}>
                    @accessible_form()
                </div>
                <div class={example_card}>
                    @accessible_navigation()
                </div>
            </section>
        </div>
    }
}

// Handler for Axum
pub async fn lesson10_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson10()))
}
