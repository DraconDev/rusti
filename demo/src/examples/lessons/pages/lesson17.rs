//! Lesson 17: htmx_integration.rs
//!
//! Building interactive UIs with HTMX (no JavaScript frameworks)
use azumi::html;

/// Todo list with HTMX
pub fn todo_with_htmx() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson17.css" />
        <div class="todo-demo">
            <h1>"HTMX Interactive Features"</h1>
            <p>"Build dynamic UIs without JavaScript frameworks"</p>
            
            <section class="demo-section">
                <h2>"Todo List"</h2>
                
                <form hx-post="/api/todos" hx-swap="beforeend" hx-target="#todo-list" class="todo-form">
                    <input 
                        type="text" 
                        name="text" 
                        placeholder="Add a todo..."
                        class="todo-input"
                        required />
                    <button type="submit" class="btn btn-primary">"Add"</button>
                </form>

                <div id="todo-list" class="todo-list">
                    <div class="todo-item">
                        <span>"Learn HTMX"</span>
                        <button hx-delete="/api/todos/1" hx-swap="remove" class="btn-delete">"×"</button>
                    </div>
                    <div class="todo-item">
                        <span>"Build with Azumi"</span>
                        <button hx-delete="/api/todos/2" hx-swap="remove" class="btn-delete">"×"</button>
                    </div>
                </div>
            </section>

            <section class="demo-section">
                <h2>"Search with Live Results"</h2>
                
                <input 
                    type="text" 
                    name="search"
                    hx-get="/api/search"
                    hx-trigger="keyup changed delay:300ms"
                    hx-target="#search-results"
                    placeholder="Type to search..."
                    class="search-input" />

                <div id="search-results" class="search-results">
                    <p>"Start typing to see search results..."</p>
                </div>
            </section>
        </div>
    }
}

/// Interactive form with HTMX
pub fn htmx_form_demo() -> impl azumi::Component {
    html! {
        <div class="form-demo">
            <h2>"Dynamic Form Submission"</h2>
            
            <form hx-post="/api/form" hx-swap="outerHTML" class="demo-form">
                <div class="form-group">
                    <label>"Username"</label>
                    <input type="text" name="username" required />
                </div>
                
                <div class="form-group">
                    <label>"Email"</label>
                    <input type="email" name="email" required />
                </div>
                
                <div class="form-group">
                    <label>"Message"</label>
                    <textarea name="message" rows="4" placeholder="Your message..."></textarea>
                </div>
                
                <button type="submit" class="btn btn-primary">"Submit with HTMX"</button>
            </form>
            
            <div class="info-box">
                <h3>"How it works:"</h3>
                <ul>
                    <li>"Form submits via AJAX (no page reload)"</li>
                    <li>"Server returns HTML fragment"</li>
                    <li>"HTMX swaps the form with the response"</li>
                    <li>"Can show loading states and progress"</li>
                </ul>
            </div>
        </div>
    }
}

/// Table with inline editing
pub fn editable_table() -> impl azumi::Component {
    html! {
        <div class="table-demo">
            <h2>"Inline Table Editing"</h2>
            
            <table class="editable-table">
                <thead>
                    <tr>
                        <th>"Name"</th>
                        <th>"Email"</th>
                        <th>"Role"</th>
                        <th>"Actions"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"Alice Johnson"</td>
                        <td>"alice@example.com"</td>
                        <td>"Admin"</td>
                        <td>
                            <button 
                                hx-get="/api/edit-user/1"
                                hx-target="closest tr"
                                hx-swap="outerHTML"
                                class="btn btn-small">"Edit"</button>
                        </td>
                    </tr>
                    <tr>
                        <td>"Bob Smith"</td>
                        <td>"bob@example.com"</td>
                        <td>"User"</td>
                        <td>
                            <button 
                                hx-get="/api/edit-user/2"
                                hx-target="closest tr"
                                hx-swap="outerHTML"
                                class="btn btn-small">"Edit"</button>
                        </td>
                    </tr>
                </tbody>
            </table>
            
            <div class="table-info">
                <h3>"Features:"</h3>
                <ul>
                    <li>"Click 'Edit' to inline edit rows"</li>
                    <li>"Server returns edit form"</li>
                    <li>"Submit to save changes"</li>
                    <li>"Cancel to revert"</li>
                </ul>
            </div>
        </div>
    }
}

/// Modal dialog with HTMX
pub fn htmx_modal() -> impl azumi::Component {
    html! {
        <div class="modal-demo">
            <h2>"Modal Dialogs"</h2>
            
            <button 
                hx-get="/api/modal-content"
                hx-target="#modal-container"
                hx-swap="innerHTML"
                class="btn btn-primary">"Open Modal"</button>
            
            <div id="modal-container" class="modal-container">
                <!-- Modal will be inserted here -->
            </div>
            
            <div class="modal-info">
                <h3>"Modal Features:"</h3>
                <ul>
                    <li>"Content loaded from server"</li>
                    <li>"Can include forms and interactions"</li>
                    <li>"Easy to implement and maintain"</li>
                    <li>"Works without JavaScript frameworks"</li>
                </ul>
            </div>
        </div>
    }
}

/// Progress indicator demo
pub fn progress_demo() -> impl azumi::Component {
    html! {
        <div class="progress-demo">
            <h2>"Progress Indicators"</h2>
            
            <button 
                hx-post="/api/long-task"
                hx-target="#task-result"
                hx-swap="innerHTML"
                hx-indicator=".htmx-indicator"
                class="btn btn-primary">"Start Long Task"</button>
            
            <div class="htmx-indicator">"Loading..."</div>
            <div id="task-result" class="task-result"></div>
            
            <div class="progress-info">
                <h3>"HTMX Indicators:"</h3>
                <ul>
                    <li>"Show loading states automatically"</li>
                    <li>"Can customize the indicator element"</li>
                    <li>"Multiple indicators supported"</li>
                    <li>"Disappears when request completes"</li>
                </ul>
            </div>
        </div>
    }
}

/// Complete HTMX integration demo
pub fn htmx_integration_demo() -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson17.css" />
        <div class="htmx-integration">
            <header class="demo-header">
                <h1>"HTMX Integration Demo"</h1>
                <p>"Interactive UIs without JavaScript frameworks"</p>
            </header>

            <nav class="demo-nav">
                <button id="todoDemo" class="nav-btn active">"Todo List"</button>
                <button id="formDemo" class="nav-btn">"Dynamic Forms"</button>
                <button id="tableDemo" class="nav-btn">"Editable Table"</button>
                <button id="modalDemo" class="nav-btn">"Modals"</button>
                <button id="progressDemo" class="nav-btn">"Progress"</button>
            </nav>

            <main class="demo-content">
                <div id="todoDemoContent" class="demo-section">
                    @todo_with_htmx()
                </div>
                
                <div id="formDemoContent" class="demo-section" style="display: none;">
                    @htmx_form_demo()
                </div>
                
                <div id="tableDemoContent" class="demo-section" style="display: none;">
                    @editable_table()
                </div>
                
                <div id="modalDemoContent" class="demo-section" style="display: none;">
                    @htmx_modal()
                </div>
                
                <div id="progressDemoContent" class="demo-section" style="display: none;">
                    @progress_demo()
                </div>
            </main>

            <aside class="demo-sidebar">
                <h3>"HTMX Benefits"</h3>
                <ul>
                    <li>"No JavaScript frameworks needed"</li>
                    <li>"Progressive enhancement"</li>
                    <li>"Server-side rendering"</li>
                    <li>"Simple and maintainable"</li>
                    <li>"Works with any backend"</li>
                </ul>
                
                <h3>"Common Attributes"</h3>
                <ul>
                    <li>"hx-get/post/put/delete"</li>
                    <li>"hx-target"</li>
                    <li>"hx-swap"</li>
                    <li>"hx-trigger"</li>
                    <li>"hx-indicator"</li>
                </ul>
            </aside>
        </div>
    }
}

/// Handler for Axum
pub async fn lesson17_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&htmx_integration_demo()))
}