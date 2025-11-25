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
                    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
                    max-width: 600px;
                    margin: 40px auto;
                    padding: 20px;
                    background: linear-gradient(135deg, skyblue 0%, purple 100%);
                    min-height: 100vh;
                }
                .container {
                    background: white;
                    border-radius: 16px;
                    padding: 40px;
                    box-shadow: 0 20px 60px rgba(0,0,0,0.3);
                }
                h1 {
                    text-align: center;
                    color: purple;
                    margin: 0 0 30px 0;
                    font-size: 32px;
                }
                .input-section {
                    display: flex;
                    gap: 10px;
                    margin-bottom: 25px;
                }
                input[type="text"] {
                    flex: 1;
                    padding: 14px 16px;
                    border: 2px solid lightgray;
                    border-radius: 8px;
                    font-size: 16px;
                    transition: border-color 0.2s;
                }
                input[type="text"]:focus {
                    outline: none;
                    border-color: purple;
                }
                .btn {
                    padding: 14px 28px;
                    border: none;
                    border-radius: 8px;
                    font-size: 16px;
                    font-weight: 600;
                    cursor: pointer;
                    transition: all 0.3s ease;
                }
                .btn-primary {
                    background: purple;
                    color: white;
                }
                .btn-primary:hover {
                    background: darkviolet;
                    transform: translateY(-2px);
                    box-shadow: 0 4px 12px rgba(128,0,128,0.3);
                }
                .btn-secondary {
                    background: lightgray;
                    color: black;
                }
                .btn-secondary:hover {
                    background: gray;
                    color: white;
                }
                .filters {
                    display: flex;
                    gap: 10px;
                    margin-bottom: 25px;
                    justify-content: center;
                }
                .filter-btn {
                    padding: 10px 20px;
                    border: 2px solid lightgray;
                    background: white;
                    border-radius: 8px;
                    cursor: pointer;
                    transition: all 0.2s;
                    font-weight: 500;
                }
                .filter-btn.active {
                    border-color: purple;
                    background: purple;
                    color: white;
                }
                .filter-btn:hover:not(.active) {
                    background: lavender;
                }
                .todo-list {
                    list-style: none;
                    padding: 0;
                    margin: 0;
                    min-height: 100px;
                }
                .todo-item {
                    display: flex;
                    align-items: center;
                    padding: 14px 16px;
                    margin: 10px 0;
                    background: whitesmoke;
                    border-radius: 8px;
                    transition: all 0.2s;
                    border-left: 4px solid purple;
                }
                .todo-item:hover {
                    background: lavender;
                    transform: translateX(4px);
                }
                .todo-item.completed {
                    opacity: 0.6;
                    border-left-color: gray;
                }
                .todo-item.completed .todo-text {
                    text-decoration: line-through;
                    color: gray;
                }
                .todo-checkbox {
                    width: 22px;
                    height: 22px;
                    margin-right: 14px;
                    cursor: pointer;
                    accent-color: purple;
                }
                .todo-text {
                    flex: 1;
                    font-size: 16px;
                    color: black;
                }
                .delete-btn {
                    background: crimson;
                    color: white;
                    border: none;
                    padding: 8px 16px;
                    border-radius: 6px;
                    cursor: pointer;
                    font-size: 14px;
                    font-weight: 600;
                    transition: all 0.2s;
                }
                .delete-btn:hover {
                    background: darkred;
                    transform: scale(1.05);
                }
                .stats {
                    margin-top: 25px;
                    padding: 16px;
                    background: lavender;
                    border-radius: 8px;
                    text-align: center;
                    color: darkslateblue;
                    font-weight: 500;
                }
                .empty-state {
                    text-align: center;
                    padding: 60px 20px;
                    color: gray;
                    font-size: 18px;
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
                    <button class="filter-btn active" onclick="filterTodos('all')" data-filter="all">All</button>
                    <button class="filter-btn" onclick="filterTodos('active')" data-filter="active">Active</button>
                    <button class="filter-btn" onclick="filterTodos('completed')" data-filter="completed">Completed</button>
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

                    const activeBtn = document.querySelector("[data-filter='" + filter + "']");
                    if (activeBtn) {
                        activeBtn.classList.add("active");
                    }

                    renderTodos();
                }

                function getFilteredTodos() {
                    console.log("Current filter:", currentFilter);
                    console.log("Todos:", todos);

                    if (!todos || !Array.isArray(todos)) {
                        return [];
                    }

                    if (currentFilter === "all") {
                        return todos;
                    }

                    if (currentFilter === "active") {
                        return todos.filter(function(t) { return !t.completed; });
                    }

                    if (currentFilter === "completed") {
                        return todos.filter(function(t) { return t.completed; });
                    }

                    // Default to all todos
                    return todos;
                }

                function renderTodos() {
                    const list = document.getElementById("todo-list");
                    const filtered = getFilteredTodos();

                    console.log("Filtered todos:", filtered);

                    if (!filtered || filtered.length === 0) {
                        console.log("filtered", filtered);
                        console.log("filtered.length", filtered.length);
                        list.innerHTML = "<div class='empty-state'>No todos to show</div>";
                    } else {
                        const html = filtered.map(function(todo) {
                            console.log("Mapping todo:", todo);
                            const completedClass = todo.completed ? " completed" : "";
                            const checked = todo.completed ? " checked" : "";
                            console.log("completedClass:", completedClass, "checked:", checked);

                            const result = "<li class='todo-item" + completedClass + "'>" +
                                "<input type='checkbox' class='todo-checkbox'" + checked + " onchange='toggleTodo(" + todo.id + ")' />" +
                                "<span class='todo-text'>" + escapeHtml(todo.text) + "</span>" +
                                "<button class='delete-btn' onclick='deleteTodo(" + todo.id + ")'>Delete</button>" +
                                "</li>";

                            console.log("Generated li HTML:", result);
                            return result;
                        }).join("");

                        console.log("Generated HTML length:", html.length);
                        console.log("First 200 chars:", html.substring(0, 200));
                        list.innerHTML = html;
                        console.log("After setting innerHTML, list.children.length:", list.children.length);
                    }

                    updateStats();
                }

                function escapeHtml(text) {
                    console.log("escapeHtml called with:", text);
                    const div = document.createElement("div");
                    div.textContent = text;
                    const result = div.innerHTML;
                    console.log("escapeHtml returning:", result);
                    return result;
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
