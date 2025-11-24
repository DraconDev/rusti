use rusti::rusti;

pub fn let_demo() -> impl rusti::Component {
    rusti! {
        <html>
            <head>
                <title>Let Demo</title>
                <style>
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
