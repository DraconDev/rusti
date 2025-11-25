use rusti::rusti;

pub fn todo_app() -> impl rusti::Component {
    rusti! {
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <title>Todo App - Rusti Demo</title>
            <style>
                body {
                    font-family: sans-serif;
                    max-width: 600px;
                    margin: 40px auto;
                    padding: 20px;
                    background: linear-gradient(135deg, "#667eea" 0%, "#764ba2" 100%);
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
                    color: "#667eea";
                    margin-bottom: 30px;
                }
                .input-section {
                    display: flex;
                    gap: 10px;
                    margin-bottom: 20px;
                }
                input[type="text"] {
                    flex: 1;
                    padding: 12px;
                    border: 2px solid "#e0e0e0";
                    border-radius: 6px;
                    font-size: 16px;
                }
                input[type="text"]:focus {
                    outline: none;
                    border-color: "#667eea";
                }
                .btn {
                    padding: 12px 24px;
                    border: none;
                    border-radius: 6px;
                    font-size: 16px;
                    font-weight: 600;
                    cursor: pointer;
                    transition: all 0.3s ease;
                }
                .btn-primary {
                    background: "#667eea";
                    color: white;
                }
                .btn-primary:hover {
                    background: "#5568d3";
                    transform: translateY(-2px);
                }
                .btn-secondary {
                    background: "#e0e0e0";
                    color: "#333";
                }
                .filters {
                    display: flex;
                    gap: 10px;
                    margin-bottom: 20px;
                    justify-content: center;
                }
                .filter-btn {
                    padding: 8px 16px;
                    border: 2px solid "#e0e0e0";
                    background: white;
                    border-radius: 6px;
                    cursor: pointer;
                    transition: all 0.2s;
                }
                .filter-btn.active {
                    border-color: "#667eea";
                    background: "#667eea";
                    color: white;
                }
                .todo-list {
                    list-style: none;
                    padding: 0;
                }
                .todo-item {
                    display: flex;
                    align-items: center;
                    padding: 12px;
                    margin: 8px 0;
                    background: "#f7f7f7";
                    border-radius: 6px;
                    transition: all 0.2s;
                }
                .todo-item:hover {
                    background: "#e8e8e8";
                }
                .todo-item.completed {
                    opacity: 0.6;
                }
                .todo-item.completed .todo-text {
                    text-decoration: line-through;
                }
                .todo-checkbox {
                    width: 20px;
                    height: 20px;
                    margin-right: 12px;
                    cursor: pointer;
                }
                .todo-text {
                    flex: 1;
                    font-size: 16px;
                }
                .delete-btn {
                    background: "#ff6b6b";
                    color: white;
                    border: none;
                    padding: 6px 12px;
                    border-radius: 4px;
                    cursor: pointer;
                    font-size: 14px;
                }
                .delete-btn:hover {
                    background: "#ff5252";
                }
                .stats {
                    margin-top: 20px;
                    padding: 15px;
                    background: "#f0f0f0";
                    border-radius: 6px;
                    text-align: center;
                    color: "#666";
                }
                .empty-state {
                    text-align: center;
                    padding: 40px;
                    color: "#999";
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Todo List App</h1>

                <div class="input-section">
                    <input type="text" id="todo-input" placeholder="What needs to be done?" />
                    <button class="btn btn-primary" onclick="addTodo()">Add</button>
                </div>

                <div class="filters">
                    <button class="filter-btn active" onclick="filterTodos(\"all\")" data-filter="all">All</button>
                    <button class="filter-btn" onclick="filterTodos(\"active\")" data-filter="active">Active</button>
                    <button class="filter-btn" onclick="filterTodos(\"completed\")" data-filter="completed">Completed</button>
                </div>

                <ul id="todo-list" class="todo-list"></ul>

                <div class="stats" id="stats"></div>

                <div style="margin-top: 20px; text-align: center;">
                    <button class="btn btn-secondary" onclick="clearCompleted()">Clear Completed</button>
                </div>
            </div>

            <script>
                const STORAGE_KEY = "rusti_todos";

                let todos = [];
                let currentFilter = "all";

                function loadTodos() {
                    const stored = localStorage.getItem(STORAGE_KEY);
                    if (stored) {
                        todos = JSON.parse(stored);
                    }
                }

                function saveTodos() {
                    localStorage.setItem(STORAGE_KEY, JSON.stringify(todos));
                }

                function addTodo() {
                    const input = document.getElementById("todo-input");
                    const text = input.value.trim();

                    if (text === "") {
                        alert("Please enter a todo!");
                        return;
                    }

                    const todo = {
                        id: Date.now(),
                        text: text,
                        completed: false,
                        createdAt: new Date().toISOString()
                    };

                    todos.push(todo);
                    input.value = "";

                    saveTodos();
                    renderTodos();
                }

                function toggleTodo(id) {
                    const todo = todos.find(function(t) { return t.id === id; });
                    if (todo) {
                        todo.completed = !todo.completed;
                        saveTodos();
                        renderTodos();
                    }
                }

                function deleteTodo(id) {
                    todos = todos.filter(function(t) { return t.id !== id; });
                    saveTodos();
                    renderTodos();
                }

                function clearCompleted() {
                    todos = todos.filter(function(t) { return !t.completed; });
                    saveTodos();
                    renderTodos();
                }

                function filterTodos(filter) {
                    currentFilter = filter;

                    const buttons = document.querySelectorAll(".filter-btn");
                    buttons.forEach(function(btn) {
                        btn.classList.remove("active");
                    });

                    const activeBtn = document.querySelector("[data-filter=\"" + filter + "\"]");
                    if (activeBtn) {
                        activeBtn.classList.add("active");
                    }

                    renderTodos();
                }

                function getFilteredTodos(filter) {
                    if (filter === "active") {
                        return todos.filter(function(t) { return !t.completed; });
                    } else if (filter === "completed") {
                        return todos.filter(function(t) { return t.completed; });
                    }
                    return [todos];
                }

                function renderTodos() {
                    const list = document.getElementById("todo-list");
                    const filtered = getFilteredTodos();

                    if (filtered.length === 0) {
                        list.innerHTML = "<div class=\"empty-state\">No todos to show</div>";
                    } else {
                        list.innerHTML = filtered.map(function(todo) {
                            const completedClass = todo.completed ? " completed" : "";
                            const checked = todo.completed ? " checked" : "";

                            return "<li class=\"todo-item" + completedClass + "\">" +
                                "<input type=\"checkbox\" class=\"todo-checkbox\"" + checked + " onchange=\"toggleTodo(" + todo.id + ")\" />" +
                                "<span class=\"todo-text\">" + todo.text + "</span>" +
                                "<button class=\"delete-btn\" onclick=\"deleteTodo(" + todo.id + ")\">Delete</button>" +
                                "</li>";
                        }).join("");
                    }

                    updateStats();
                }

                function updateStats() {
                    const total = todos.length;
                    const completed = todos.filter(function(t) { return t.completed; }).length;
                    const active = total - completed;

                    const stats = document.getElementById("stats");
                    stats.innerHTML = "<strong>" + total + "</strong> total tasks | " +
                        "<strong>" + active + "</strong> active | " +
                        "<strong>" + completed + "</strong> completed";
                }

                document.getElementById("todo-input").addEventListener("keypress", function(e) {
                    if (e.key === "Enter") {
                        addTodo();
                    }
                });

                loadTodos();
                renderTodos();
                console.log("Todo App initialized with", todos.length, "todos");
            </script>
        </body>
        </html>
    }
}
