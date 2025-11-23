// # Rusti Macro – Comprehensive Examples

// Below is a collection of example snippets that demonstrate the full range of features supported by the `rusti!` macro. Feel free to copy‑paste these into your own projects or use them as a reference when building components.

// ---

// ## 1. Basic HTML
// ```rust
// rusti! {
//     <div class="p-4">
//         <h1>Hello, world!</h1>
//         <p>This is a plain paragraph.</p>
//     </div>
// }
// ```
// ---

// ## 2. Dynamic Content (Interpolation)
// ```rust
// let name = "Alice";
// let count = 3;
// rusti! {
//     <p>Welcome, { name }! You have { count } new messages.</p>
// }
// ```
// ---

// ## 3. Conditional Rendering (`@if` / `@else`)
// ```rust
// let logged_in = true;
// let user = "Bob";
// rusti! {
//     <nav>
//         @if logged_in {
//             <span>Welcome, { user }!</span>
//             <a href="/logout">Logout</a>
//         } @else {
//             <a href="/login">Login</a>
//         }
//     </nav>
// }
// ```
// ---

// ## 4. Loops (`@for`)
// ```rust
// let items = vec!["Apples", "Bananas", "Cherries"];
// rusti! {
//     <ul>
//         @for item in items {
//             <li>{ item }</li>
//         }
//     </ul>
// }
// ```
// ---

// ## 5. Pattern Matching (`@match`)
// ```rust
// enum Status { Active, Pending, Suspended }
// let status = Status::Pending;
// rusti! {
//     @match status {
//         Status::Active => { <span class="green">Active</span> }
//         Status::Pending => { <span class="yellow">Pending</span> }
//         Status::Suspended => { <span class="red">Suspended</span> }
//     }
// }
// ```
// ---

// ## 6. Component Composition (`@component_name`)
// ```rust
// fn button(label: &str) -> impl rusti::Component + '_' {
//     rusti! { <button class="btn">{ label }</button> }
// }

// fn page() -> impl rusti::Component {
//     rusti! {
//         <main>
//             <h2>Dashboard</h2>
//             @button("Save")
//             @button("Cancel")
//         </main>
//     }
// }
// ```
// ---

// ## 7. Dynamic Attributes
// ```rust
// let disabled = true;
// let img_src = "/avatars/me.png";
// rusti! {
//     <button disabled={disabled}>Submit</button>
//     <img src={img_src} alt="Profile picture" />
// }
// ```
// ---

// ## 8. Attributes with Spaces Around `=`
// ```rust
// rusti! {
//     <html lang = "en">
//         <head>
//             <meta charset = "UTF-8" />
//         </head>
//     </html>
// }
// ```
// ---

// ## 9. Inline `<style>` and `<script>` (Raw Text)
// ```rust
// rusti! {
//     <style>
//         body { background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%); }
//         .card { backdrop-filter: blur(10px); border: 1px solid rgba(255,255,255,0.1); }
//     </style>
//     <script>
//         console.log("Hello from inline script!");
//     </script>
//     <div class="card p-4">
//         <h3>Styled Card</h3>
//     </div>
// }
// ```
// ---

// ## 10. Loose Styles / Scripts (No `<style>`/`<script>` wrapper)
// You can embed raw CSS/JS directly as text nodes when you need them outside of a tag (e.g., when generating a `<style>` block programmatically). The parser treats any text that is not inside `<` … `>` as plain text, so braces are preserved.
// ```rust
// let css = r#"body{background:linear-gradient(135deg,#0f172a 0%,#1e293b 100%)}"#;
// let js = r#"console.log('inline');"#;
// rusti! {
//     <style>{ css }</style>
//     <script>{ js }</script>
// }
// ```
// ---

// ## 11. Self‑Closing Tags
// ```rust
// rusti! {
//     <img src="/logo.png" alt="Logo" />
//     <br />
//     <input type="text" name="username" />
// }
// ```
// ---

// ## 12. Text with Braces (Escaping)
// If you need literal `{` or `}` inside text (e.g., showing a code snippet), wrap the text in a raw string literal or escape it with double braces `{{` `}}` inside the macro:
// ```rust
// rusti! {
//     <pre>{ "{{ let x = 5; }}" }</pre>
// }
// ```
// ---

// ## 13. Nested Components & Slots
// ```rust
// fn card(title: &str, content: impl rusti::Component) -> impl rusti::Component + '_' {
//     rusti! {
//         <section class="card">
//             <h2>{ title }</h2>
//             @content
//         </section>
//     }
// }

// fn page() -> impl rusti::Component {
//     rusti! {
//         @card("Welcome", {
//             <p>This is the inner slot content.</p>
//         })
//     }
// }
// ```
// ---

// ## 14. HTML Comments (Not Supported – Workaround)
// The parser does not currently support `<!-- comment -->`. Use Rust comments outside the macro or generate comments via a text node:
// ```rust
// rusti! {
//     <!-- This will cause a parse error -->
//     // Instead, do:
//     <p>{ "<!-- Safe comment -->" }</p>
// }
// ```
// ---

// ## 15. Complex Example – Full Page
// ```rust
// fn base_layout(title: &str, styles: &str, content: impl rusti::Component) -> impl rusti::Component + '_' {
//     rusti! {
//         <html lang="en">
//             <head>
//                 <meta charset="UTF-8" />
//                 <meta name="viewport" content="width=device-width, initial-scale=1.0" />
//                 <title>{ title }</title>
//                 <style>{ styles }</style>
//                 <script src="https://cdn.tailwindcss.com"></script>
//                 <script src="https://unpkg.com/htmx.org@1.9.10"></script>
//             </head>
//             <body class="bg-gray-900 text-white min-h-screen">
//                 <nav class="glass-card rounded-2xl p-4 mb-8">
//                     <a href="/" class="text-2xl font-bold">MyApp</a>
//                     @if is_authenticated {
//                         <a href="/profile">Profile</a>
//                         <form action="/logout" method="post"><button type="submit">Logout</button></form>
//                     } @else {
//                         <a href="/login">Login</a>
//                     }
//                 </nav>
//                 <main class="container mx-auto p-8">
//                     @content
//                 </main>
//                 <footer class="text-center mt-8">© 2025 MyApp</footer>
//             </body>
//         </html>
//     }
// }
// ```

// These examples should give you a solid foundation for using the `rusti!` macro in a variety of real‑world scenarios.

use crate::base_layout_demo::base_layout;

fn extreme_example(is_authenticated: bool, user_name: &str, items: &[&str]) -> impl rusti::Component + '_ {
    let custom_styles = "
        .glass-card {
            background: rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(10px);
            border: 1px solid rgba(255, 255, 255, 0.2);
            box-shadow: 0 4px 30px rgba(0, 0, 0, 0.1);
        }
        .animate-pulse-slow {
            animation: pulse-slow 4s cubic-bezier(0.4, 0, 0.6, 1) infinite;
        }
        @keyframes pulse-slow {
            0%, 100% { opacity: 1; }
            50% { opacity: .5; }
        }
    ";

    base_layout("Extreme Rusti! Example", custom_styles, rusti! {
        <div class="container mx-auto p-8">
            <h1 class="text-5xl font-extrabold text-center mb-12 text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-pink-600 animate-pulse-slow">
                The Ultimate Rusti! Macro Showcase
            </h1>

            <div class="grid grid-cols-1 lg:grid-cols-3 gap-8 mb-12">
                <div class="lg:col-span-2 glass-card p-8 rounded-2xl shadow-xl">
                    @if is_authenticated {
                        <h2 class="text-3xl font-bold mb-6">Welcome back, { user_name }!</h2>
                        <p class="text-lg mb-4">
                            You've unlocked the full power of Rusti! Here's a glimpse of what you can do.
                        </p>

                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-8">
                            <div class="bg-gray-800 p-6 rounded-lg shadow-md">
                                <h3 class="text-2xl font-semibold mb-4 text-blue-400">Dynamic Content</h3>
                                <p class="text-gray-300">
                                    Current time: <strong>{ chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string() }</strong>
                                </p>
                                <button
                                    class="mt-4 px-6 py-3 bg-blue-600 hover:bg-blue-700 rounded-lg text-white font-bold transition duration-300"
                                    hx-get="/api/refresh-data"
                                    hx-swap="outerHTML"
                                    hx-target="#dynamic-section"
                                >
                                    Refresh Data
                                </button>
                                <div id="dynamic-section" class="mt-4 text-gray-400">
                                    <p>Click button to load new content...</p>
                                </div>
                            </div>

                            <div class="bg-gray-800 p-6 rounded-lg shadow-md">
                                <h3 class="text-2xl font-semibold mb-4 text-green-400">Looping Elements</h3>
                                <ul class="list-disc list-inside text-gray-300">
                                    @for item in items {
                                        <li>Item: <strong>{ item }</strong></li>
                                    }
                                </ul>
                                @if items.is_empty() {
                                    <p class="text-red-400 mt-4">No items available yet!</p>
                                } else if items.len() < 3 {
                                    <p class="text-yellow-400 mt-4">Add more items for a richer experience!</p>
                                } @else {
                                    <p class="text-green-400 mt-4">You have a good selection of items!</p>
                                }
                            </div>
                        </div>
                    } @else {
                        <h2 class="text-3xl font-bold mb-6 text-red-400">Access Denied!</h2>
                        <p class="text-lg mb-4">
                            Please <a href="/login" class="text-blue-400 hover:underline    font-semibold">log in</a> to
                            view this content.
                        </p>
                    }
                </div>

                <div class="lg:col-span-1 glass-card p-8 rounded-2xl shadow-xl">
                    <h2 class="text-3xl font-bold mb-6 text-center text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-indigo-600">
                        Interactive Features
                    </h2>
                    <p class="text-lg mb-4 text-center">
                        Experience the power of Rusti! with real-time updates and dynamic content.
                    </p>

                    <div class="flex flex-col items-center gap-4">
                        <button
                            class="px-6 py-3 bg-indigo-600 hover:bg-indigo-700 rounded-lg text-white font-bold transition duration-300"
                            hx-get="/api/refresh-data"
                            hx-swap="outerHTML"
                            hx-target="#dynamic-section"
                        >
                            Refresh Data
                        </button>
                        <div id="dynamic-section" class="mt-4 text-gray-400">
                            <p>Click button to load new content...</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    })
}

