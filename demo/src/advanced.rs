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

// Advanced Match Demo: Enums with Data
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn advanced_match_page() -> impl rusti::Component {
    let messages = vec![
        Message::Write(String::from("Hello, Rusti!")),
        Message::Move { x: 10, y: 20 },
        Message::ChangeColor(255, 0, 128),
        Message::Quit,
    ];

    rusti! {
        <div class="bg-slate-800 rounded-2xl p-8 shadow-lg border border-slate-700">
            <h2 class="text-2xl font-bold text-slate-100 mb-4">Advanced Pattern Matching</h2>
            <div class="space-y-4">
                @for msg in &messages {
                    @match msg {
                        Message::Quit => {
                            <div class="p-4 bg-red-900/30 border-l-4 border-red-600 text-red-200">
                                <span class="font-bold">Quit</span>: System shutting down...
                            </div>
                        }
                        Message::Move { x, y } => {
                            <div class="p-4 bg-blue-900/30 border-l-4 border-blue-600 text-blue-200">
                                <span class="font-bold">Move</span>: Moving to coordinates ({ x }, { y })
                            </div>
                        }
                        Message::Write(text) => {
                            <div class="p-4 bg-green-900/30 border-l-4 border-green-600 text-green-200">
                                <span class="font-bold">Write</span>: "{ text }"
                            </div>
                        }
                        Message::ChangeColor(r, g, b) => {
                            <div class="p-4 bg-purple-900/30 border-l-4 border-purple-600 text-purple-200 flex items-center gap-4">
                                <span class="font-bold">Color Change</span>:
                                <div class="w-8 h-8 rounded-full border border-slate-500" style={format!("background-color: rgb({}, {}, {})", r, g, b)}></div>
                                <span>RGB({ r }, { g }, { b })</span>
                            </div>
                        }
                    }
                }
            </div>
            <pre class="bg-slate-950 text-green-400 p-4 rounded mt-4 text-sm border border-slate-800"><code>{"@match msg {\n    Message::Move { x, y } => { ... }\n    Message::Write(text) => { ... }\n    ... \n}"}</code></pre>
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
                    <input type="text" placeholder="Enter your name" class="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-lg text-slate-200 focus:outline-none focus:border-blue-500 transition-colors"></input>
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
                    <input type="checkbox" id="check1" class="w-5 h-5 bg-slate-900 border-slate-700 rounded text-blue-600 focus:ring-blue-500"></input>
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
