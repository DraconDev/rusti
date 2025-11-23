use rusti::rusti;

// Nested Loops Demo: Multiplication Table
pub fn nested_loops_page() -> impl rusti::Component {
    let size = 10;
    let rows: Vec<i32> = (1..=size).collect();
    let cols: Vec<i32> = (1..=size).collect();

    rusti! {
        <div class="bg-slate-800 rounded-2xl p-8 shadow-lg border border-slate-700">
            <h2 class="text-2xl font-bold text-slate-100 mb-4">Nested Loops: Multiplication Table</h2>
            <div class="overflow-x-auto">
                <table class="w-full text-center border-collapse">
                    <thead>
                        <tr>
                            <th class="p-2 text-slate-400 border border-slate-700 bg-slate-900">x</th>
                            @for col in &cols {
                                <th class="p-2 text-slate-200 border border-slate-700 bg-slate-900">{ col }</th>
                            }
                        </tr>
                    </thead>
                    <tbody>
                        @for row in &rows {
                            <tr>
                                <th class="p-2 text-slate-200 border border-slate-700 bg-slate-900">{ row }</th>
                                @for col in &cols {
                                    <td class="p-2 text-slate-300 border border-slate-700 hover:bg-slate-700 transition-colors">
                                        { row * col }
                                    </td>
                                }
                            </tr>
                        }
                    </tbody>
                </table>
            </div>
            <pre class="bg-slate-950 text-green-400 p-4 rounded mt-4 text-sm border border-slate-800"><code>{"@for row in &rows {\n    <tr>\n        @for col in &cols {\n            <td>{ row * col }</td>\n        }\n    </tr>\n}"}</code></pre>
        </div>
    }
}

// Advanced Pattern Matching with Options
pub fn advanced_match_page() -> impl rusti::Component {
    let statuses = vec!["success", "warning", "error", "info"];
    let priorities = vec![1, 2, 3, 4];

    rusti! {
        <div class="space-y-8">
            <div class="bg-slate-800 rounded-2xl p-8 shadow-lg border border-slate-700">
                <h2 class="text-2xl font-bold text-slate-100 mb-4">Status Matching</h2>
                <div class="space-y-4">
                    @for status in &statuses {
                        @match *status {
                            "success" => {
                                <div class="p-4 bg-green-900/30 border-l-4 border-green-600 text-green-200">
                                    <span class="text-2xl">{ "✓" }</span>
                                    <span class="ml-3 font-bold">Success</span>: Operation completed
                                </div>
                            }
                            "warning" => {
                                <div class="p-4 bg-yellow-900/30 border-l-4 border-yellow-600 text-yellow-200">
                                    <span class="text-2xl">{ "⚠" }</span>
                                    <span class="ml-3 font-bold">Warning</span>: Proceed with caution
                                </div>
                            }
                            "error" => {
                                <div class="p-4 bg-red-900/30 border-l-4 border-red-600 text-red-200">
                                    <span class="text-2xl">{ "✗" }</span>
                                    <span class="ml-3 font-bold">Error</span>: Operation failed
                                </div>
                            }
                            _ => {
                                <div class="p-4 bg-blue-900/30 border-l-4 border-blue-600 text-blue-200">
                                    <span class="text-2xl">{ "ℹ" }</span>
                                    <span class="ml-3 font-bold">Info</span>: General information
                                </div>
                            }
                        }
                    }
                </div>
                <pre class="bg-slate-950 text-green-400 p-4 rounded mt-4 text-sm border border-slate-800"><code>{"@match status {\n    \"success\" => { ... }\n    \"warning\" => { ... }\n    _ => { ... }\n}"}</code></pre>
            </div>

            <div class="bg-slate-800 rounded-2xl p-8 shadow-lg border border-slate-700">
                <h2 class="text-2xl font-bold text-slate-100 mb-4">Numeric Priority Matching</h2>
                <div class="grid md:grid-cols-2 gap-4">
                    @for priority in &priorities {
                        @match *priority {
                            1 => {
                                <div class="p-6 bg-red-900/30 border border-red-700 rounded-lg text-red-200">
                                    <div class="text-5xl mb-2">{ "🔥" }</div>
                                    <div class="text-xl font-bold">Critical</div>
                                    <div class="text-sm">Immediate action required</div>
                                </div>
                            }
                            2 => {
                                <div class="p-6 bg-orange-900/30 border border-orange-700 rounded-lg text-orange-200">
                                    <div class="text-5xl mb-2">{ "⚡" }</div>
                                    <div class="text-xl font-bold">High</div>
                                    <div class="text-sm">Address soon</div>
                                </div>
                            }
                            3 => {
                                <div class="p-6 bg-yellow-900/30 border border-yellow-700 rounded-lg text-yellow-200">
                                    <div class="text-5xl mb-2">{ "📋" }</div>
                                    <div class="text-xl font-bold">Medium</div>
                                    <div class="text-sm">Plan accordingly</div>
                                </div>
                            }
                            _ => {
                                <div class="p-6 bg-blue-900/30 border border-blue-700 rounded-lg text-blue-200">
                                    <div class="text-5xl mb-2">{ "📝" }</div>
                                    <div class="text-xl font-bold">Low</div>
                                    <div class="text-sm">When time permits</div>
                                </div>
                            }
                        }
                    }
                </div>
                <pre class="bg-slate-950 text-green-400 p-4 rounded mt-4 text-sm border border-slate-800"><code>{"@match priority {\n    1 => { <div>Critical</div> }\n    2 => { <div>High</div> }\n    _ => { <div>Low</div> }\n}"}</code></pre>
            </div>
        </div>
    }
}

// Forms Demo
pub fn forms_page() -> impl rusti::Component {
    rusti! {
        <div class="bg-slate-800 rounded-2xl p-8 shadow-lg border border-slate-700">
            <h2 class="text-2xl font-bold text-slate-100 mb-4">Forms & Inputs</h2>
            <form class="space-y-6">
                <div>
                    <label class="block text-sm font-medium text-slate-400 mb-2">Text Input</label>
                    <input type="text" placeholder="Enter your name" class="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-lg text-slate-200 focus:outline-none focus:border-blue-500 transition-colors">
                </div>

                <div>
                    <label class="block text-sm font-medium text-slate-400 mb-2">Select Option</label>
                    <select class="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-lg text-slate-200 focus:outline-none focus:border-blue-500 transition-colors">
                        <option>Option 1</option>
                        <option>Option 2</option>
                        <option>Option 3</option>
                    </select>
                </div>

                <div class="flex items-center gap-3">
                    <input type="checkbox" id="check1" class="w-5 h-5 bg-slate-900 border-slate-700 rounded text-blue-600 focus:ring-blue-500">
                    <label for="check1" class="text-slate-300">I agree to the terms</label>
                </div>

                <div>
                    <label class="block text-sm font-medium text-slate-400 mb-2">Message</label>
                    <textarea rows="4" class="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-lg text-slate-200 focus:outline-none focus:border-blue-500 transition-colors" placeholder="Type your message..."></textarea>
                </div>

                <button type="button" class="px-6 py-3 bg-blue-600 text-white rounded-lg font-semibold hover:bg-blue-700 transition-colors w-full">
                    Submit Form
                </button>
            </form>
        </div>
    }
}
