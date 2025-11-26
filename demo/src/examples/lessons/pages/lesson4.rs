//! Lesson 4: loops_and_iteration.rs
//!
//! Using @for to render lists
use azumi::html;

/// Todo list renderer using @for loop
pub fn todo_list(todos: &[&str]) -> impl azumi::Component  {
    html! {
        <style src="/static/pages/lesson4.css" />
        <div class="lesson4-container">
            <h1 class="lesson4-title">"Todo List"</h1>
            <ul class="lesson4-todo-list">
                @for todo in todos {
                    <li class="lesson4-todo-item">{todo}</li>
                }
            </ul>
        </div>
    }
}

/// Example todo list with sample data
pub fn example_todo_list() -> impl azumi::Component {
    todo_list(&["Buy groceries", "Clean the house", "Call mom"])
}

/// Empty list handling
pub fn empty_todo_list() -> impl azumi::Component {
    todo_list(&[])
}