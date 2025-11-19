use rusti::rusti;

pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
}

impl ButtonVariant {
    fn classes(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "bg-blue-600 text-white hover:bg-blue-700",
            ButtonVariant::Secondary => "bg-gray-200 text-gray-800 hover:bg-gray-300",
            ButtonVariant::Danger => "bg-red-600 text-white hover:bg-red-700",
        }
    }
}

pub fn button<'a>(
    label: &'a str,
    variant: ButtonVariant,
    onclick: Option<&'a str>,
) -> impl rusti::Component + 'a {
    let classes = format!(
        "px-4 py-2 rounded-lg font-semibold transition-colors duration-200 {}",
        variant.classes()
    );

    let onclick_attr = onclick.unwrap_or("");

    rusti! {
        <button class={&classes} onclick={onclick_attr}>
            { label }
        </button>
    }
}

pub fn card<'a>(
    title: &'a str,
    body: &'a str,
    footer: Option<&'a str>,
) -> impl rusti::Component + 'a {
    rusti! {
        <div class="bg-white rounded-xl shadow-md overflow-hidden border border-gray-100 hover:shadow-lg transition-shadow duration-300">
            <div class="p-6 border-b border-gray-100 bg-gray-50">
                <h3 class="text-xl font-bold text-gray-800">{ title }</h3>
            </div>
            <div class="p-6">
                <p class="text-gray-600 leading-relaxed">{ body }</p>
            </div>
            @if let Some(footer_text) = footer {
                <div class="p-4 bg-gray-50 border-t border-gray-100 text-sm text-gray-500 text-right">
                    { footer_text }
                </div>
            }
        </div>
    }
}

pub fn components_page() -> impl rusti::Component {
    rusti! {
        <html>
            <head>
                <title>Components Demo</title>
                <script src="https://cdn.tailwindcss.com"></script>
            </head>
            <body class="bg-gray-50 min-h-screen p-12">
                <div class="max-w-4xl mx-auto space-y-12">
                    <header class="text-center">
                        <h1 class="text-4xl font-black text-gray-900 mb-4">Reusable Components</h1>
                        <p class="text-lg text-gray-600">Building blocks with Rusti</p>
                    </header>

                    <section class="space-y-6">
                        <h2 class="text-2xl font-bold text-gray-800 border-b pb-2">Buttons</h2>
                        <div class="flex gap-4 p-6 bg-white rounded-xl shadow-sm">
                            @button("Primary Action", ButtonVariant::Primary, Some("alert('Primary clicked!')"))
                            @button("Secondary Action", ButtonVariant::Secondary, None)
                            @button("Delete Item", ButtonVariant::Danger, Some("confirm('Are you sure?')"))
                        </div>
                    </section>

                    <section class="space-y-6">
                        <h2 class="text-2xl font-bold text-gray-800 border-b pb-2">Cards</h2>
                        <div class="grid md:grid-cols-2 gap-6">
                            @card(
                                "Simple Card",
                                "This is a basic card component with a title and body text. It demonstrates how to pass string slices to components.",
                                None
                            )
                            @card(
                                "Card with Footer",
                                "This card includes an optional footer section. The footer is conditionally rendered using Rust's pattern matching.",
                                Some("Last updated: Just now")
                            )
                        </div>
                    </section>

                    <footer class="text-center pt-8">
                        <a href="/" class="text-blue-600 hover:underline">Back to Home</a>
                    </footer>
                </div>
            </body>
        </html>
    }
}
