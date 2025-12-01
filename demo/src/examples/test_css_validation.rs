use azumi::html;

#[azumi::component]
pub fn test_css_validation() -> impl azumi::Component {
    html! {
        <style>
            .test_class {
                color: "cent er";
                width: "10pz";
                background: "#gggggg";
            }
        </style>
        <div class={test_class}>"Testing strict validation"</div>
    }
}
