// Simple manual test for CSS hot reload
//
// To test:
// 1. Run: cargo build
// 2. Modify tests/test_hotreload.css (change .test-class to .test-class-renamed)
// 3. Run: cargo build (should FAIL with CSS validation error)
// 4. If it doesn't fail, hot reload isn't working

use azumi::html;

fn test_component() -> impl azumi::Component {
    html! {
        <style src="/tests/test_hotreload.css" />
        <div class="test-class">"Test"</div>
    }
}

fn main() {
    let rendered = azumi::render_to_string(&test_component());
    println!("{}", rendered);
}
