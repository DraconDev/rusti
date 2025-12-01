use azumi::html;

pub fn homepage() -> impl azumi::Component {
    html! {
        <div>
            <h1>"Homepage"</h1>
        </div>
    }
}

pub fn homepage_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&homepage()))
}
