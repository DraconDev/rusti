use rusti::rusti;

pub fn interactive_counter() -> impl rusti::Component {
    rusti! {
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <title>Interactive Counter</title>
            <style>
                body {
                    font-family: sans-serif;
                        max-width: 800px;
                    margin: 50px auto;
                    padding: 20px;
                    background: linear-gradient(135deg, skyblue, purple);
                    min-height: 100vh;
                }
                .container {
                    background: white;
                    border-radius: 12px;
                    padding: 30px;
                    box-shadow: 0 20px 60px rgba(0,0,0,0.3);
                }
                h1 {
                    text-align: center;
                    margin-bottom: 30px;
                    color: darkblue;
                }
                .counter-display {
                    text-align: center;
                    margin: 30px 0;
                }
                .counter-value {
                    font-size: 72px;
                    font-weight: bold;
                    color: darkblue;
                }
                .status {
                    font-size: 20px;
                    margin-top: 10px;
                    color: #666;
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
                    font-weight: 600;
                    color: white;
                }
                button:hover {
                    transform: translateY(-2px);
                    box-shadow: 0 4px 12px rgba(0,0,0,0.2);
                }
                .btn-increment {
                    background: green;
                }
                .btn-decrement {
                    background: red;
                }
                .btn-reset {
                    background: blue;
                }
                .history {
                    margin-top: 30px;
                    padding: 20px;
                    background: whitesmoke;
                    border-radius: 8px;
                }
                .history h3 {
                    margin-top: 0;
                }
                .history-list {
                    list-style: none;
                    padding: 0;
                }
                .history-item {
                    padding: 8px 12px;
                    margin: 5px 0;
                    background: white;
                    border-left: 4px solid darkblue;
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
                const INITIAL_VALUE = 0;
                const STEP_SIZE = 1;
                const MAX_HISTORY = 10;
                const POSITIVE_THRESHOLD = 10;
                const NEGATIVE_THRESHOLD = -10;

                let counter = INITIAL_VALUE;
                let history = [];

                console.log("History:", history);
                console.log("History length:", history.length);
                console.log("Max history:", MAX_HISTORY);

                function addHistory(action, value) {
                    const time = new Date().toLocaleTimeString();
                    const entry = { action: action, value: value, time: time };
                    history.unshift(entry);



                    if (history.length > MAX_HISTORY) {
                        history = history.slice(0, MAX_HISTORY);
                    }

                    updateDisplayHistory();
                }

                function updateDisplayHistory() {
                    const list = document.getElementById("history-list");

                    if (history.length === 0) {
                        list.innerHTML = "<li>No actions yet</li>";
                        return;
                    }

                    list.innerHTML = history.map(function(e) {
                        return "<li class=\"history-item\"><strong>" + e.time + "</strong> - " + e.action + ": " + e.value + "</li>";
                    }).join("");
                }

                function updateDisplay() {
                    document.getElementById("counter").textContent = counter;

                    let status = "";
                    if (counter > POSITIVE_THRESHOLD) {
                        status = "On fire!";
                    } else if (counter > 0) {
                        status = "Positive vibes!";
                    } else if (counter === 0) {
                        status = "Perfectly balanced";
                    } else if (counter < NEGATIVE_THRESHOLD) {
                        status = "Deep freeze!";
                    } else {
                        status = "Below zero";
                    }

                    document.getElementById("status").textContent = status;
                }

                function increment() {
                    counter += STEP_SIZE;
                    updateDisplay();
                    addHistory("Increment", counter);
                }

                function decrement() {
                    counter -= STEP_SIZE;
                    updateDisplay();
                    addHistory("Decrement", counter);
                }

                function reset() {
                    counter = INITIAL_VALUE;
                    updateDisplay();
                    addHistory("Reset", counter);
                }

                updateDisplay();
                updateDisplayHistory();
                console.log("Counter initialized! Initial:", INITIAL_VALUE, "Step:", STEP_SIZE);
            </script>
        </body>
        </html>
    }
}
