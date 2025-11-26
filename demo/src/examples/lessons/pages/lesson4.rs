//! Lesson 4: loops_and_iteration.rs
//!
//! Using @for to render lists
use azumi::html;

/// Todo list renderer using @for loop
pub fn todo_list(todos: &[&str]) -> impl azumi::Component {
    html! {
        <ul>
            @for todo in todos {
                <li>{todo}</li>
            }
        </ul>
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