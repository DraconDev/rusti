// About page demo for Rusti

pub fn about_page() -> impl rusti::Component {
    let year = 2025;
    rusti! {
        <html>
            @page_head("About Rusti")
            <body class="bg-slate-900 min-h-screen text-slate-200">
                <div class="container mx-auto px-4 py-12 max-w-7xl">
                    @page_header("About Rusti", "Learn more about this demo framework")
                    <main>
                        <section class="mb-12">
                            <h2 class="text-4xl font-bold text-slate-100 mb-8 text-center">What is Rusti?</h2>
                            <p class="text-slate-300 max-w-3xl mx-auto">
                                Rusti is a Rust macro that lets you write HTML-like syntax directly in Rust,
                                providing zero‑cost, type‑safe UI rendering with automatic XSS protection.
                            </p>
                            <p class="text-slate-300 max-w-3xl mx-auto mt-4">
                                This demo showcases components, conditionals, loops, pattern matching,
                                HTMX interactivity, dynamic attributes, and more.
                            </p>
                        </section>
                    </main>
                    @page_footer(year)
                </div>
            </body>
        </html>
    }
}

// Wrapper for routing
pub fn about_page_wrapper() -> impl rusti::Component {
    let year = 2025;
    rusti! {
        <html>
            @page_head("About Rusti Demo")
            <body class="bg-slate-900 min-h-screen text-slate-200">
                <div class="container mx-auto px-4 py-12 max-w-7xl">
                    @page_header("About Rusti Demo", "Overview of the Rusti demo application")
                    <main>
                        @about_page()
                    </main>
                    @page_footer(year)
                </div>
            </body>
        </html>
    }
}
