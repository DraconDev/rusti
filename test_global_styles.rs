use azumi::html;

#[azumi::component]
pub fn test_global_styles() -> impl azumi::Component {
    html! {
        <style global>
            .test_class {
                color: "red";
                background: "blue";
            }

            #test_id {
                font-size: "16px";
            }
        </style>

        <div class={test_class} id={test_id}>
            "This should work with global styles"
        </div>
    }
}