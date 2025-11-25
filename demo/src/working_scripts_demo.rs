use rusti::rusti;

pub fn working_scripts_demo() -> impl rusti::Component {
    // Test data for various scenarios
    let test_string = "Hello, Rusti!";
    let test_number = 42;
    let test_float = 3.14159;
    let test_bool = true;
    let items = vec!["Apple", "Banana", "Cherry", "Date", "Elderberry"];
    let numbers = vec![1, 2, 3, 4, 5];
    let user_data = vec![("Alice", 25), ("Bob", 30), ("Charlie", 35)];
    let status = "success";
    let debug_mode = true;
    let nested_array = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    rusti! {
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <title>Working Scripts Test Demo</title>
            <style>
                body {
                    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
                    background: linear-gradient(135deg, "#667eea" 0%, "#764ba2" 100%);
                    padding: "2em";
                    min-height: 100vh;
                }

                .container {
                    max-width: 1200px;
                    margin: 0 auto;
                }

                h1 {
                    color: white;
                    text-align: center;
                    font-size: "3em";
                    margin-bottom: "1em";
                    text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
                }

                .test-section {
                    background: rgba(255, 255, 255, 0.95);
                    border-radius: 16px;
                    padding: "2em";
                    margin-bottom: "2em";
                    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
                }

                h2 {
                    color: "#2aa";
                    margin-top: 0;
                    border-bottom: 2px solid "#667eea";
                    padding-bottom: "0.5em";
                }

                .test-result {
                    background: "#f0f9ff";
                    border-left: 4px solid "#3b82f6";
                    padding: "1em";
                    margin: "1em" 0;
                    font-family: "Courier New", monospace;
                }

                .success {
                    border-left-color: "#10b981";
                    background: "#d1fae5";
                }

                .error {
                    border-left-color: "#ef4444";
                    background: "#fee2e2";
                }

                .code-block {
                    background: "#1e293b";
                    color: "#e2e8f0";
                    padding: "1em";
                    border-radius: 8px;
                    overflow-x: auto;
                    margin: "1em" 0;
                }

                .nav-button {
                    display: inline-block;
                    background: white;
                    color: "#667eea";
                    padding: 12px 24px;
                    border-radius: 8px;
                    text-decoration: none;
                    font-weight: bold;
                    box-shadow: 0 4px 6px rgba(0,0,0,0.1);
                    transition: transform 0.2s;
                }

                .nav-button:hover {
                    transform: translateY(-2px);
                    box-shadow: 0 6px 12px rgba(0,0,0,0.15);
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Working Scripts Test Suite</h1>
                <p style="text-align: center; color: white; font-size: 1.2em; margin-bottom: 2em;">
                    Comprehensive JavaScript integration tests to validate script injection and control flow
                </p>

                <a href="/" class="nav-button">Back to Home</a>

                <!-- Test Results Container -->
                <div id="test-1" class="test-section">
                    <h2>Test 1: Basic Variable Injection</h2>
                    <div id="result-1">Loading...</div>
                </div>

                <div id="test-2" class="test-section">
                    <h2>Test 2: Number Operations</h2>
                    <div id="result-2">Loading...</div>
                </div>

                <div id="test-3" class="test-section">
                    <h2>Test 3: Array Iteration with "@for"</h2>
                    <div id="result-3">Loading...</div>
                </div>

                <div id="test-4" class="test-section">
                    <h2>Test 4: Conditional Logic with "@if"</h2>
                    <div id="result-4">Loading...</div>
                </div>

                <div id="test-5" class="test-section">
                    <h2>Test 5: Match Patterns in Scripts</h2>
                    <div id="result-5">Loading...</div>
                </div>

                <div id="test-6" class="test-section">
                    <h2>Test 6: Complex Data Structures</h2>
                    <div id="result-6">Loading...</div>
                </div>

                <div id="test-7" class="test-section">
                    <h2>Test 7: Nested Loops</h2>
                    <div id="result-7">Loading...</div>
                </div>

                <div id="test-8" class="test-section">
                    <h2>Test 8: Combined Control Flow</h2>
                    <div id="result-8">Loading...</div>
                </div>

                <div id="test-summary" class="test-section">
                    <h2>Test Summary</h2>
                    <div id="summary-content">
                        <p>Running tests...</p>
                    </div>
                </div>
            </div>

            <script>
                // Test tracking
                const testResults = {
                    passed: 0,
                    failed: 0,
                    total: 8
                };

                function markSuccess(testId, message) {
                    const element = document.getElementById("result-" + testId);
                    element.className = "test-result success";
                    element.innerHTML = "<strong>✅ PASSED:</strong> " + message;
                    testResults.passed++;
                }

                function markFailure(testId, message, error) {
                    const element = document.getElementById("result-" + testId);
                    element.className = "test-result error";
                    element.innerHTML = "<strong>❌ FAILED:</strong> " + message + "<br><code>" + error + "</code>";
                    testResults.failed++;
                }

                function updateSummary() {
                    const summaryElement = document.getElementById("summary-content");
                    const percentage = ((testResults.passed / testResults.total) * 100).toFixed(1);
                    summaryElement.innerHTML =
                        "<div style=\"display: grid; grid-template-columns: repeat(3, 1fr); gap: 1em; margin-top: 1em;\">" +
                            "<div style=\"text-align: center; padding: 1em; background: #d1fae5; border-radius: 8px;\">" +
                                "<div style=\"font-size: 2em; color: #10b981; font-weight: bold;\">" + testResults.passed + "</div>" +
                                "<div style=\"color: #065f46;\">Passed</div>" +
                            "</div>" +
                            "<div style=\"text-align: center; padding: 1em; background: #fee2e2; border-radius: 8px;\">" +
                                "<div style=\"font-size: 2em; color: #ef4444; font-weight: bold;\">" + testResults.failed + "</div>" +
                                "<div style=\"color: #991b1b;\">Failed</div>" +
                            "</div>" +
                            "<div style=\"text-align: center; padding: 1em; background: #dbeafe; border-radius: 8px;\">" +
                                "<div style=\"font-size: 2em; color: #3b82f6; font-weight: bold;\">" + percentage + "%</div>" +
                                "<div style=\"color: #1e40af;\">Success Rate</div>" +
                            "</div>" +
                        "</div>";
                }

                // console.log("Total Tests:", testResults.total);
                // console.log("Passed:", testResults.passed);
                // console.log("Failed:", testResults.failed);
                // console.log("Success Rate:", ((testResults.passed / testResults.total) * 100).toFixed(1) + "%");
            </script>
        </body>
        </html>
    }
}
