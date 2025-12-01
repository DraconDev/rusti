use azumi::html;

#[azumi::component]
pub fn test_css_validation() -> impl azumi::Component {
    html! {
        <style>
            .test_class {
                color: "blue";
                width: "100px";
                background: "#ff0000";
            }
        </style>
        <div class={test_class}>"CSS Validation Working!"</div>
    }
}
