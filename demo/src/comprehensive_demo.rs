use rusti::rusti;

pub fn comprehensive_demo() -> impl rusti::Component {
    rusti! {
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <title>Comprehensive Rusti Demo</title>
            <style>
                * {
                    margin: 0;
                    padding: 0;
                    box-sizing: border-box;
                }

                body {
                    font-family: "Inter", system-ui, sans-serif;
                    background: linear-gradient(135deg, "#667eea" 0%, #764ba2 100%);
                    min-height: 100vh;
                    padding: 2rem;
                }

                .container {
                    max-width: 800px;
                    margin: 0 auto;
                    background: white;
                    border-radius: 16px;
                    padding: "2em";
                    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
                }

                h1 {
                    color: "#667eea";
                    font-size: 2.5rem;
                    margin-bottom: 1rem;
                    text-align: center;
                }

                .feature-box {
                    background: #f8f9fa;
                    border-left: 4px solid "#667eea";
                    padding: 1.5rem;
                    margin: 1.5rem 0;
                    border-radius: 8px;
                }

                .feature-box h2 {
                    color: #495057;
                    margin-bottom: 0.75rem;
                    font-size: 1.5rem;
                }

                .counter {
                    display: flex;
                    align-items: center;
                    gap: 1rem;
                    margin-top: 1rem;
                }

                button {
                    background: "#667eea";
                    color: white;
                    border: none;
                    padding: 0.75rem 1.5rem;
                    border-radius: 8px;
                    font-size: 1rem;
                    cursor: pointer;
                    transition: all 0.3s ease;
                }

                button:hover {
                    background: #5a67d8;
                    transform: translateY(-2px);
                    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
                }

                .count-display {
                    font-size: 2rem;
                    font-weight: bold;
                    color: "#667eea";
                }

                .item-list {
                    list-style: none;
                    padding: 0;
                }

                .item-list li {
                    background: white;
                    padding: 1rem;
                    margin: 0.5rem 0;
                    border-radius: 8px;
                    border: 2px solid "#e9ecef";
                    transition: all 0.2s ease;
                }

                .item-list li:hover {
                    border-color: "#667eea";
                    transform: translateX(5px);
                }

                code {
                    background: #f1f3f5;
                    padding: 0.25rem 0.5rem;
                    border-radius: 4px;
                    font-family: "Monaco", monospace;
                    color: "#e83e8c";
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Comprehensive Rusti Demo</h1>

                <div class="feature-box">
                    <h2>"Feature 1: @let Variables"</h2>
                    @let greeting = "Hello";
                    @let name = "Rustacean";
                    @let full_greeting = format!("{}, {}!", greeting, name);

                    <p><code>"@let"</code> allows scoped variable declarations!</p>
                    <p><strong>{full_greeting}</strong></p>

                    @let x = 10;
                    @let y = 25;
                    @let sum = x + y;

                    <p>Math: {x} + {y} = <strong>{sum}</strong></p>
                </div>

                <div class="feature-box">
                    <h2>Feature 2: Inline CSS</h2>
                    <p>Standard CSS works perfectly!</p>
                    <ul>
                        <li>Most units work fine: 3rem, 16px</li>
                        <li>Decimal units: decimal values work fine</li>
                        <li>Rare lexer issues: quote problematic values</li>
                    </ul>
                </div>

                <div class="feature-box">
                    <h2>Feature 3: Inline JavaScript</h2>
                    <p>Standard JavaScript works!</p>
                    <p><strong>Rule:</strong> Always use double quotes, never single quotes!</p>

                    <div class="counter">
                        <button onclick="decrementCounter()">-</button>
                        <span class="count-display" id="counter">0</span>
                        <button onclick="incrementCounter()">+</button>
                        <button onclick="resetCounter()">Reset</button>
                    </div>
                </div>

                <div class="feature-box">
                    <h2>Feature 4: @let + @for Combo</h2>
                    @let items = vec!["React".to_string(), "Vue".to_string(), "Angular".to_string(), "Svelte".to_string()];
                    @let item_count = items.len();

                    <p>You have <strong>{item_count}</strong> frameworks:</p>
                    <ul class="item-list">
                        @for (index, item) in items.iter().enumerate() {
                            <li>#{index + 1}: {item}</li>
                        }
                    </ul>
                </div>

                <div class="feature-box">
                    <h2>"Feature 5: @let + @if Logic"</h2>
                    @let score = 95;
                    @let grade = if score >= 90 { "A" } else if score >= 80 { "B" } else { "C" };
                    @let is_passing = score >= 60;

                    <p>Score: <strong>{score}</strong></p>
                    <p>Grade: <strong>{grade}</strong></p>

                    @if is_passing {
                        <p style="color: green;">Status: <strong>Passing!</strong></p>
                    } else {
                        <p style="color: red;">Status: <strong>Needs Improvement</strong></p>
                    }
                </div>
            </div>

           <script>
                const app = {
                    count: 0,
                    maxCount: 10,

                    increment() {
                        if (this.count < this.maxCount) {
                            this.count++;
                            this.updateDisplay();
                        }
                    },

                    decrement() {
                        if (this.count > 0) {
                            this.count--;
                            this.updateDisplay();
                        }
                    },

                    reset() {
                        this.count = 0;
                        this.updateDisplay();
                    },

                    updateDisplay() {
                        const display = document.getElementById("counter");
                        if (display) {
                            display.textContent = this.count;

                            if (this.count === 0) {
                                display.style.color = "#6c757d";
                            } else if (this.count >= this.maxCount) {
                                display.style.color = "#dc3545";
                            } else {
                                display.style.color = "#667eea";
                            }
                        }
                    }
                };

                function incrementCounter() {
                    app.increment();
                }

                function decrementCounter() {
                    app.decrement();
                }

                function resetCounter() {
                    app.reset();
                }

                document.addEventListener("DOMContentLoaded", function() {
                    console.log("Rusti Comprehensive Demo Loaded!");
                    app.updateDisplay();
                });
            </script>
        </body>
        </html>
    }
}
