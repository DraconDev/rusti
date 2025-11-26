// Test file to verify CSS error positioning
use azumi::Component;

pub fn test_error_positioning() -> impl azumi::Component {
    html! {
        <div class="non-existent-class">
            <h1>"Test Error Positioning"</h1>
        </div>
    }
}
