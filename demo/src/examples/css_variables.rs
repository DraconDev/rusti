use azumi::html;

#[azumi::component]
pub fn css_variables_demo() -> impl azumi::Component {
    html! {
        <div>"Empty"</div>
    }
}

pub async fn css_variables_handler() -> axum::response::Html<String> {
    let component = css_variables_demo();
    axum::response::Html(azumi::render_to_string(&component))
}
