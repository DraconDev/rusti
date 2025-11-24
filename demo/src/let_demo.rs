use rusti::rusti;

pub fn let_demo() -> impl rusti::Component {
    rusti! {
        <html>
            <head>
                <title>Let Demo</title>
                <style>
                    .nav { margin-top: 20px; }
                    .btn { display: inline-block; margin: 5px; padding: 8px 12px; background: #4f46e5; color: white; border-radius: 4px; text-decoration: none; font-family: Inter, sans-serif; }
                    .btn:hover { background: #4338ca; }
                    body {
                        font-family: Arial, sans-serif;
                        padding: 20px;
                    }
                    .box {
                        padding: 20px;
                        margin: 10px 0;
                        border-radius: 8px;
                        background: #3b82f6;
                        color: white;
                    }
                </style>
            </head>
            <body>
                <h1>Variable Declarations Demo</h1>

                @let greeting = "Hello";
                @let name = "Rust Developer";

                <div class="box">
                    <p>{greeting}, {name}!</p>
                </div>
                <div class="nav">
                    <h2>Demo Routes</h2>
                    <a class="btn" href="/">Home</a>
                    <a class="btn" href="/htmx">HTMX Demo</a>
                    <a class="btn" href="/conditionals">Conditionals</a>
                    <a class="btn" href="/lists">Lists &amp; Iteration</a>
                    <a class="btn" href="/match">Pattern Matching</a>
                    <a class="btn" href="/xss">XSS Protection</a>
                    <a class="btn" href="/attributes">Dynamic Attributes</a>
                    <a class="btn" href="/nested-loops">Nested Loops</a>
                    <a class="btn" href="/advanced-match">Advanced Match</a>
                    <a class="btn" href="/forms">Forms</a>
                    <a class="btn" href="/script-style">Script &amp; Style Demo</a>
                    <a class="btn" href="/let-demo">@let Syntax Demo</a>
                    <a class="btn" href="/component-macro">Component Macro</a>
                </div>

                @let x = 10;
                @let y = 20;
                @let sum = x + y;

                <div class="box">
                    <p>{x} + {y} = {sum}</p>
                </div>
            </body>
        </html>
    }
}
