use rusti::rusti;

/// A more complex interactive example demonstrating:
/// - Multiple @let declarations with different types
/// - Event handlers with Rust code
/// - DOM manipulation
/// - Conditional rendering
/// - Array operations
pub fn interactive_counter() -> impl rusti::Component {
    rusti! {
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <title>Interactive Counter - Rusti Demo</title>
            <style>
                body {
                    font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
                    max-width: 800px;
                    margin: 50px auto;
                    padding: 20px;
                    background: linear-gradient(135deg, "#667eea" 0%, #764ba2 100%);
                    min-height: 100vh;
                }
                
                .container {
                    background: white;
                    border-radius: 12px;
                    padding: 30px;
                    box-shadow: 0 20px 60px rgba(0,0,0,0.3);
                }
                
                h1 {
                    color: "#667eea";
                    text-align: center;
                    margin-bottom: 30px;
                }
                
                .counter-display {
                    text-align: center;
                    margin: 30px 0;
                }
                
                .counter-value {
                    font-size: 72px;
                    font-weight: bold;
                    color: "#667eea";
                    text-shadow: 2px 2px 4px rgba(0,0,0,0.1);
                }
                
                .status {
                    font-size: 20px;
                    color: #666;
                    margin-top: 10px;
                }
                
                .controls {
                    display: flex;
                    gap: 10px;
                    justify-content: center;
                    margin: 20px 0;
                }
                
                button {
                    padding: 12px 24px;
                    font-size: 16px;
                    border: none;
                    border-radius: 6px;
                    cursor: pointer;
                    transition: all 0.3s ease;
                    font-weight: 600;
                }
                
                button:hover {
                    transform: translateY(-2px);
                    box-shadow: 0 4px 12px rgba(0,0,0,0.2);
                }
                
                .btn-increment {
                    background: #48bb78;
                    color: white;
                }
                
                .btn-decrement {
                    background: #f56565;
                    color: white;
                }
                
                .btn-reset {
                    background: #4299e1;
                    color: white;
                }
                
                .history {
                    margin-top: 30px;
                    padding: 20px;
                    background: #f7fafc;
                    border-radius: 8px;
                }
                
                .history h3 {
                    margin-top: 0;
                    color: #2d3748;
                }
                
                .history-list {
                    list-style: none;
                    padding: 0;
                }
                
                .history-item {
                    padding: 8px 12px;
                    margin: 5px 0;
                    background: white;
                    border-left: 4px solid "#667eea";
                    border-radius: 4px;
                    animation: slideIn 0.3s ease;
                }
                
                @keyframes slideIn {
                    from {
                        opacity: 0;
                        transform: translateX(-20px);
                    }
                    to {
                        opacity: 1;
                        transform: translateX(0);
                    }
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>"🎯" Interactive Counter</h1>
                
                <div class="counter-display">
                    <div id="counter" class="counter-value">0</div>
                    <div id="status" class="status">Click a button to start!</div>
                </div>
                
                <div class="controls">
                    <button class="btn-decrement" onclick="decrement()">"➖" Decrement</button>
                    <button class="btn-reset" onclick="reset()">"🔄" Reset</button>
                    <button class="btn-increment" onclick="increment()">"➕" Increment</button>
                </div>
                
                <div class="history">
                    <h3>Action History</h3>
                    <ul id="history-list" class="history-list"></ul>
                </div>
            </div>
            
            
        </body>
        </html>
    }
}
