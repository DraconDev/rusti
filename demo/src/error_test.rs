use rusti::rusti;

fn main() {
    // Uncomment one by one to test error messages

    // 1. Unclosed element
    rusti! { <div> };

    // 2. Mismatched closing tag
    // rusti! { <div></span> };

    // 3. Invalid attribute
    // rusti! { <div class=></div> };

    // 4. Missing expression brace
    // rusti! { @if true <div></div> };
}
