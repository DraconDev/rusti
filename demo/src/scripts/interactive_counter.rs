use rusti::rusti;

/// A more complex interactive example demonstrating:
/// - Multiple @let declarations with different types
/// - Event handlers with JavaScript
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
                    font-family: sans-serif;
                    max-width: 800px;
                    margin: 50px auto;
                    padding: 20px;
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    min-height: 100vh;
                }
                
                .container {
                    background: white;
                    border-radius: 12px;
                    padding: 30px;
                    box-shadow: 0 20px 60px rgba(0,0,0,0.3);
                }
                
                h1 {
                    color: #667eea;
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
                    color: #667eea;
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
                    border-left: 4px solid #667eea;
                    border-radius: 4px;
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Interactive Counter</h1>
                
                <div class="counter-display">
                    <div id="counter" class="counter-value">0</div>
                    <div id="status" class="status">Click a button to start!</div>
                </div>
                
                <div class="controls">
                    <button class="btn-decrement" onclick="decrement()">Decrement</button>
                    <button class="btn-reset" onclick="reset()">Reset</button>
                    <button class="btn-increment" onclick="increment()">Increment</button>
                </div>
                
                <div class="history">
                    <h3>Action History</h3>
                    <ul id="history-list" class="history-list"></ul>
                </div>
            </div>
            
            <script>
                @let initial_value = 0;
                @let step_size = 1;
                @let max_history = 10;
                @let positive_threshold = 10;
                @let negative_threshold = -10;
                
                // State
                let counter = @{ initial_value };
                let history = [];
                
                // Add history entry
                function addHistory(action, value) {
                    const time = new Date().toLocaleTimeString();
                    history.unshift({ action, value, time });
                    
                    if (history.length > @{ max_history }) {
                        history = history.slice(0, @{ max_history });
                    }
                    
                    updateHistoryDisplay();
                }
                
                // Update history display
                function updateHistoryDisplay() {
                    const list = document.getElementById("history-list");
                    
                    if (history.length === 0) {
                        list.innerHTML = "<li style='color: #999; text-align: center;'>No actions yet</li>";
                        return;
                    }
                    
                    list.innerHTML = history.map(e => 
                        "<li class='history-item'><strong>" + e.time + "</strong> - " + e.action + ": " + e.value + "</li>"
                    ).join("");
                }
                
                // Update counter display
                function updateDisplay() {
                    document.getElementById("counter").textContent = counter;
                    
                    let status = "";
                    if (counter > @{ positive_threshold }) {
                        status = "On fire!";
                    } else if (counter > 0) {
                        status = "Positive vibes!";
                    } else if (counter === 0) {
                        status = "Perfectly balanced";
                    } else if (counter < @{ negative_threshold }) {
                        status = "Deep freeze!";
                    } else {
                        status = "Below zero";
                    }
                    
                    document.getElementById("status").textContent = status;
                }
                
                // Button handlers
                function increment() {
                    counter += @{ step_size };
                    updateDisplay();
                    addHistory("Increment", counter);
                }
                
                function decrement() {
                    counter -= @{ step_size };
                    updateDisplay();
                    addHistory("Decrement", counter);
                }
                
                function reset() {
                    counter = @{ initial_value };
                    updateDisplay();
                    addHistory("Reset", counter);
                }
                
                // Initialize
                updateDisplay();
                updateHistoryDisplay();
                console.log("Counter initialized! Initial:", @{ initial_value }, "Step:", @{ step_size });
            </script>
        </body>
        </html>
    }
}
