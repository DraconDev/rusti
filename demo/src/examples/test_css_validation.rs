use azumi::html;

#[azumi::component]
pub fn test_css_validation() -> impl azumi::Component {
    html! {
        <style>
            .test_class {
                color: "this is not a color at all";
                width: "!!!invalid!!!";
                background: "()()()";
            }
        </style>
        <div class={test_class}>"Test"</div>
    }
}
