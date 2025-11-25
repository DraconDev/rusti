use axum::response::{Html, IntoResponse};
use rusti::rusti;

// Define a component with optional props using the #[component] macro
#[rusti::component]
pub fn Message(
    #[prop(default = "\"info\".to_string()")] variant: String,
    #[prop(default = "false")] dismissible: bool,
    children: impl rusti::Component,
) -> impl rusti::Component {
    let bg_color = match variant.as_str() {
        "success" => "bg-green-100 text-green-800",
        "error" => "bg-red-100 text-red-800",
        "warning" => "bg-yellow-100 text-yellow-800",
        _ => "bg-blue-100 text-blue-800", // info default
    };

    rusti! {
        <div class={format!("p-4 rounded-md mb-4 flex justify-between items-center {}", bg_color)}>
            <div>
                @children
            </div>
            @if dismissible {
                <button class="ml-4 text-sm font-bold opacity-50 hover:opacity-100">
                    Dismiss
                </button>
            }
        </div>
    }
}

pub fn optional_props_demo() -> impl rusti::Component {
    rusti! {
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8" />
            <title>Optional Props Demo</title>
            <script src="https://cdn.tailwindcss.com"></script>
        </head>
        <body class="p-8 bg-gray-50">
            <h1 class="text-2xl font-bold mb-6">Optional Props Demo</h1>

            <h2 class="text-xl font-semibold mb-4">1. Default Props (Info, Non-dismissible)</h2>
            <!-- No props provided, should use defaults -->
            @Message() {
                This is a default message (Info variant).
            }

            <h2 class="text-xl font-semibold mb-4 mt-8">2. Explicit Props (Success, Dismissible)</h2>
            <!-- All props provided -->
            @Message(variant = "success".to_string(), dismissible = true) {
                Operation completed successfully!
            }

            <h2 class="text-xl font-semibold mb-4 mt-8">3. Partial Props (Error, Default Dismissible)</h2>
            <!-- Only variant provided -->
            @Message(variant = "error".to_string()) {
                An error occurred while processing your request.
            }

            <h2 class="text-xl font-semibold mb-4 mt-8">4. Partial Props (Warning, Explicit Dismissible)</h2>
            <!-- Variant and dismissible provided -->
            @Message(variant = "warning".to_string(), dismissible = true) {
                Please review your settings.
            }
        </body>
        </html>
    }
}

pub async fn optional_props_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&optional_props_demo()))
}
