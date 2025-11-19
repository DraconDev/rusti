// Example showing XSS protection with automatic HTML escaping
use rusti::rusti;

pub fn safe_display(user_input: &str) -> impl rusti::Component + '_ {
    rusti! {
        <div class="p-4 bg-yellow-50 border border-yellow-200 rounded">
            <h3 class="font-bold">User Input (Auto-Escaped):</h3>
            <p class="mt-2 font-mono bg-white p-2 rounded">{ user_input }</p>
            <p class="text-sm text-gray-600 mt-2">
                All dynamic content is automatically HTML-escaped for XSS protection.
            </p>
        </div>
    }
}
