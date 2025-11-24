use rusti::rusti;

pub fn script_style_demo() -> impl rusti::Component {
    let primary_color = "#3b82f6"; // Blue-500
    let font_size = 16;
    let user_name = "Rustacean";
    let items = vec!["Apple", "Banana", "Cherry"];
    let show_debug = true;

    rusti! {
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8" />
            <title>Script & Style Demo</title>
            <style>
                /* Standard CSS with braces - treated as text */
                body {
                    font-family: sans-serif;
                    background-color: #f0f9ff;
                    padding: 2rem;
                }

                .card {
                    background: white;
                    border-radius: 8px;
                    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
                    padding: "2em";
                    max-width: 600px;
                    margin: 0 auto;
                }

                /* Dynamic CSS using @{ ... } */
                .dynamic-text {
                    color: @{ primary_color };
                    font-size: @{ format!("{}px", font_size) };
                }

                /* Control flow in CSS */
                @if show_debug {
                    @{ "
                    .debug-info {
                        border: 1px dashed red;
                        padding: 10px;
                        margin-top: 20px;
                        background: #fff0f0;
                    }
                    " }
                }
            </style>
        </head>
        <body>
            <div class="card">
                <h1 class="dynamic-text">Script & Style Injection Demo</h1>
                <p>This page demonstrates the new <code>{r#"@{ ... }"#}</code> syntax.</p>

                <div id="app"></div>

                @if show_debug {
                    <div class="debug-info">
                        <strong>Debug Mode:</strong> ON
                    </div>
                }
            </div>

            <script>
                // Standard JS Object - treated as text
                const config = {
                    appName: "RustiDemo",
                    version: "1.0.0",
                    features: {
                        darkMode: false,
                        analytics: true
                    }
                };

                console.log("Config loaded:", config);

                // Injecting Rust variables using @{ ... }
                const userName = "@{ user_name }";
                const items = [
                    // Using @for loop to generate JS array elements
                    @for item in &items {
                        "@{ item }",
                    }
                ];

                // Using string concatenation instead of backticks
                const message = "Hello, " + userName + "! You have " + items.length + " items.";

                document.getElementById("app").innerHTML =
                    "<p>" + message + "</p>" +
                    "<ul>" +
                        items.map(function(item) { return "<li>" + item + "</li>"; }).join("") +
                    "</ul>";

                // Control flow in Script
                @if show_debug {
                    console.log("Debug mode is enabled from Rust!");
                    console.log("Items:", items);
                }
            </script>
        </body>
        </html>
    }
}
