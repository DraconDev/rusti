use azumi::{html, Component};

#[test]
fn test_css_hot_reload_dependency() {
    // This test verifies that the CSS file is tracked as a dependency
    // If test_hotreload.css changes, this file should recompile

    let component = html! {
        <div>
            <style src="/tests/test_hotreload.css" />
            <div class="test-class">"Test Content"</div>
        </div>
    };

    let rendered = azumi::render_to_string(&component);
    assert!(rendered.contains("test-class"));

    // The mere fact that this compiles means include_bytes! worked
    // and the CSS file is now tracked by cargo
}
