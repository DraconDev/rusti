use rusti::rusti;

pub struct TodoItem {
    pub id: u32,
    pub text: String,
    pub completed: bool,
}

pub fn todo_item(item: &TodoItem) -> impl rusti::Component + '_ {
    let base_classes = "flex items-center justify-between p-4 bg-white rounded-lg shadow-sm border transition-all hover:shadow-md";
    let status_classes = if item.completed {
        "border-green-200 bg-green-50"
    } else {
        "border-gray-200"
    };
    let text_classes = if item.completed {
        "text-gray-400 line-through"
    } else {
        "text-gray-800 font-medium"
    };

    rusti! {
        <div class={format!("{} {}", base_classes, status_classes)}>
            <span class={text_classes}>{ &item.text }</span>
            @if item.completed {
                <span class="px-3 py-1 text-xs font-bold text-green-700 bg-green-200 rounded-full">Done</span>
            } else {
                <span class="px-3 py-1 text-xs font-bold text-yellow-700 bg-yellow-200 rounded-full">Pending</span>
            }
        </div>
    }
}

pub fn todo_list(items: &[TodoItem]) -> impl rusti::Component + '_ {
    rusti! {
        <div class="space-y-3">
            @for item in items {
                @todo_item(item)
            }
        </div>
    }
}

pub fn todo_page(items: &[TodoItem]) -> impl rusti::Component + '_ {
    let completed_count = items.iter().filter(|i| i.completed).count();
    let total_count = items.len();
    let progress = if total_count > 0 {
        (completed_count as f32 / total_count as f32) * 100.0
    } else {
        0.0
    };

    rusti! {
        <html>
            <head>
                <title>Todo List Demo</title>
                <script src="https://cdn.tailwindcss.com"></script>
            </head>
            <body class="bg-gray-100 min-h-screen p-8">
                <div class="max-w-2xl mx-auto">
                    <header class="mb-8 text-center">
                        <h1 class="text-4xl font-black text-gray-900 mb-2">My Tasks</h1>
                        <p class="text-gray-600">Demonstrating iteration and conditionals</p>
                    </header>

                    <div class="bg-white rounded-2xl p-8 shadow-xl mb-8">
                        <div class="flex justify-between items-end mb-6">
                            <div>
                                <h2 class="text-2xl font-bold text-gray-800">Progress</h2>
                                <p class="text-gray-500">{ completed_count } of { total_count } tasks completed</p>
                            </div>
                            <div class="text-3xl font-black text-blue-600">{ format!("{:.0}%", progress) }</div>
                        </div>
                        <div class="w-full bg-gray-200 rounded-full h-2.5">
                            <div class="bg-blue-600 h-2.5 rounded-full transition-all duration-500" style={format!("width: {}%", progress)}></div>
                        </div>
                    </div>

                    @todo_list(items)

                    <footer class="text-center mt-12">
                        <a href="/" class="text-blue-600 hover:underline">Back to Home</a>
                    </footer>
                </div>
            </body>
        </html>
    }
}
