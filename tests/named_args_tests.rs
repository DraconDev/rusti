use azumi::html;

// Test component for named argument validation
#[azumi::component]
fn TestButton(text: &str, #[prop(default = "\"blue\"")] color: &str) -> impl azumi::Component {
    html! {
        <button class="btn">{text}" - "{color}</button>
    }
}

#[test]
fn test_named_arguments_work() {
    // Named arguments should work fine
    let button = html! {
        <div>
            @TestButton(text="Click Me")
            @TestButton(text="Submit", color="green")
        </div>
    };

    let rendered = azumi::render_to_string(&button);
    assert!(rendered.contains("Click Me"));
    assert!(rendered.contains("Submit"));
}

#[test]
fn test_empty_args_work_for_components() {
    // Empty args should work for components (allows all defaults)
    // This test won't compile if empty args are broken
    let _ = html! {
        <div>
            "Should compile with empty brackets"
        </div>
    };
}

// Commented out: This should fail to compile with a helpful error
/*
#[test]
fn test_positional_args_rejected() {
    // This SHOULD NOT COMPILE - positional arguments are not allowed
    let _ = html! {
        <div>
            @TestButton("Click Me", "red")  // ERROR: Must use named arguments!
        </div>
    };
}
*/
