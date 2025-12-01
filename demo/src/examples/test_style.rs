use azumi::html;

#[azumi::component]
pub fn test_style() -> impl azumi::Component {
    html! {
        <style>
            .test { color: "red"; }
        </style>
        <div class={test}>"Test"</div>
    }
}
