
use azumi::html;

// This test intentionally uses positional arguments to verify error message quality
// It should NOT compile - we're checking that the error is helpful

fn test_component<'a>(text: &'a str, color: &'a str) -> impl azumi::Component + 'a {
    html! {
        <div>{text}" "{color}</div>
    }
}

fn main() {
    // This should produce a clear error message pointing to the exact problem
    let _ = html! {
        <div>
            @test_component("Hello", "blue")
        </div>
    };
}
