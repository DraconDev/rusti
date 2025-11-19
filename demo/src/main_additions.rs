// TEMPORARY FILE - These functions will be added to main.rs

// Lists and iteration demo
fn lists_page() -> impl rusti::Component {
    let year = 2025;
    let items = vec!["Apples", "Bananas", "Oranges", "Grapes"];
    let scores = vec![95, 87, 92, 78, 88];

    rusti! {
        <html>
            @page_head("Lists Demo")
            <body class="bg-gradient-to-br from-blue-50 via-indigo-50 to-purple-50 min-h-screen">
                <div class="container mx-auto px-4 py-12 max-w-7xl">
                    @page_header("Lists & Iteration", "Dynamic list rendering with @for loops")
                    <main>
                        <div class="space-y-8">
                            <div class="bg-white rounded-2xl p-8 shadow-lg">
                                <h2 class="text-2xl font-bold text-gray-800 mb-4">Simple List</h2>
                                <ul class="space-y-2">
                                    @for item in items {
                                        <li class="p-3 bg-blue-50 rounded">
                                            { item }
                                        </li>
                                    }
                                </ul>
                                <pre class="bg-gray-800 text-green-400 p-3 rounded mt-4 text-sm"><code>{"@for item in items { <li>{ item }</li> }"}</code></pre>
                            </div>

                            <div class="bg-white rounded-2xl p-8 shadow-lg">
                                <h2 class="text-2xl font-bold text-gray-800 mb-4">Styled Score List</h2>
                                <div class="space-y-2">
                                    @for score in scores {
                                        <div class="flex items-center gap-4 p-4 bg-gradient-to-r from-purple-50 to-pink-50 rounded-lg">
                                            <span class="text-3xl font-bold text-purple-600">{ score }</span>
                                            @if score >= 90 {
                                                <span class="px-3 py-1 bg-green-500 text-white rounded-full text-sm">A</span>
                                            } else {
                                                <span class="px-3 py-1 bg-yellow-500 text-white rounded-full text-sm">B</span>
                                            }
                                        </div>
                                    }
                                </div>
                            </div>
                        </div>
                    </main>
                    @page_footer(year)
                </div>
            </body>
        </html>
    }
}

// XSS protection demo
fn xss_page() -> impl rusti::Component {
    let year = 2025;
    let user_input = "<script>alert('XSS')</script>";

    rusti! {
        <html>
            @page_head("XSS Protection Demo")
            <body class="bg-gradient-to-br from-blue-50 via-indigo-50 to-purple-50 min-h-screen">
                <div class="container mx-auto px-4 py-12 max-w-7xl">
                    @page_header("XSS Protection", "Automatic HTML escaping keeps your app secure")
                    <main>
                        <div class="space-y-8">
                            <div class="bg-white rounded-2xl p-8 shadow-lg">
                                <h2 class="text-2xl font-bold text-gray-800 mb-4">Automatic Escaping</h2>
                                <div class="space-y-4">
                                    <div class="p-4 bg-red-50 border-l-4 border-red-500">
                                        <p class="font-bold text-red-700">Malicious Input:</p>
                                        <code class="text-sm bg-white p-2 rounded">{ user_input }</code>
                                    </div>
                                    <div class="p-4 bg-green-50 border-l-4 border-green-500">
                                        <p class="font-bold text-green-700">Safely Rendered:</p>
                                        <p class="bg-white p-2 rounded">{ user_input }</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </main>
                    @page_footer(year)
                </div>
            </body>
        </html>
    }
}

// Attributes demo
fn attributes_page() -> impl rusti::Component {
    let year = 2025;
    let btn_class = "px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700";

    rusti! {
        <html>
            @page_head("Attributes Demo")
            <body class="bg-gradient-to-br from-blue-50 via-indigo-50 to-purple-50 min-h-screen">
                <div class="container mx-auto px-4 py-12 max-w-7xl">
                    @page_header("Dynamic Attributes", "Interpolate values into HTML attributes")
                    <main>
                        <div class="space-y-8">
                            <div class="bg-white rounded-2xl p-8 shadow-lg">
                                <h2 class="text-2xl font-bold text-gray-800 mb-4">Dynamic Classes</h2>
                                <button class={btn_class}>Styled Button</button>
                                <pre class="bg-gray-800 text-green-400 p-3 rounded mt-4 text-sm"><code>{"<button class={btn_class}>...</button>"}</code></pre>
                            </div>
                        </div>
                    </main>
                    @page_footer(year)
                </div>
            </body>
        </html>
    }
}
