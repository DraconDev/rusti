//! Lesson 30: Loading States
//!
//! Skeleton screens and loading indicators

use azumi::html;

#[azumi::component]
pub fn skeleton_card() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson30.css" />
        <div class="card skeleton">
            <div class="skeleton-img"></div>
            <div class="skeleton-text title"></div>
            <div class="skeleton-text body"></div>
            <div class="skeleton-text body short"></div>
        </div>
    }
}

#[azumi::component]
pub fn content_card<'a>(title: &'a str, body: &'a str) -> impl azumi::Component + 'a {
    html! {
        <style src="/static/pages/lesson30.css" />
        <div class="card content">
            <div class="card-img-placeholder">"IMG"</div>
            <h3>{title}</h3>
            <p>{body}</p>
        </div>
    }
}

#[azumi::component]
pub fn loading_demo() -> impl azumi::Component {
    html! {
        @let is_loading = true;
        <style src="/static/pages/lesson30.css" />
        <div class="container">
            <h1>"Lesson 30: Loading States"</h1>

            <div class="grid">
                <div class="col">
                    <h2>"Loading State (Skeleton)"</h2>
                    @skeleton_card()
                </div>

                <div class="col">
                    <h2>"Loaded State"</h2>
                    @content_card(
                        title="Awesome Content",
                        body="This content has finished loading and is ready to view."
                    )
                </div>
            </div>

            <div class="toggle-demo">
                <h2>"Dynamic Toggle"</h2>
                <p>"Simulating state change:"</p>

                @if is_loading {
                    @skeleton_card()
                } else {
                    @content_card(title="Loaded!", body="Data is here.")
                }
            </div>
        </div>
    }
}

pub async fn lesson30_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&html! { @loading_demo() }))
}
