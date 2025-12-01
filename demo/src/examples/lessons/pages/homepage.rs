use azumi::{html};

pub fn homepage() -> impl azumi::Component {
    style! {
        .homepage_title {
            color: "purple";
            font-size: "3em";
        }
    }
    html! {
        <div>
            <h1 class={homepage_title}>
                "Homepage"
            </h1>
        </div>
    }
}

pub fn homepage_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&homepage()))
}
