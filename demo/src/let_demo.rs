use rusti::rusti;

pub fn let_demo() -> impl rusti::Component {
    rusti! {
        <html>
            <head>
                <title>Let Demo</title>
                <style>
                    body { font-family: Arial, sans-serif; padding: 20px; }
                    .box { padding: 20px; margin: 10px 0; border-radius: 8px; }
                    .primary { background: #3b82f6; color: white; }
                    .secondary { background: #6b7280; color: white; }
                </style>
            </head>
            <body>
                <h1>@let Syntax Demo</h1>

                <div class="box primary">
                    @let greeting = "Hello";
                    @let name = "Rust Developer";
                    <p>{greeting}, {name}!</p>
                </div>

                <div class="box secondary">
                    @let items = vec!["Apple", "Banana", "Cherry"];
                    @let count = items.len();
                    <p>You have {count} items:</p>
                    <ul>
                        @for item in &items {
                            <li>{item}</li>
                        }
                    </ul>
                </div>

                <div class="box primary">
                    @let x = 10;
                    @let y = 20;
                    @let sum = x + y;
                    <p>{x} + {y} = {sum}</p>
                </div>

                <p><em>Variables declared with @let are scoped within the rusti! macro block</em></p>
            </body>
        </html>
    }
}
