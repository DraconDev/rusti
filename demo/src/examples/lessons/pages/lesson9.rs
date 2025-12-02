use azumi::html;

/// Lesson 9: Lists and Iteration
///
/// Demonstrates @for loops and list rendering
#[azumi::component]
pub fn simple_list() -> impl azumi::Component {
    html! {
        <style>
            .list_container { padding: "20px"; }
            .item { padding: "8px"; margin: "4px 0"; background: "#f0f0f0"; }
        </style>
        <div class={list_container}>
            <h3>"Simple List"</h3>
            <ul>
                <li class={item}>"Apple"</li>
                <li class={item}>"Banana"</li>
                <li class={item}>"Cherry"</li>
                <li class={item}>"Date"</li>
            </ul>
        </div>
    }
}

/// Example: List with index
#[azumi::component]
pub fn indexed_list() -> impl azumi::Component {
    html! {
        <style>
            .indexed_container { padding: "20px"; }
            .fruit_item { padding: "8px"; margin: "4px 0"; }
            .index { font-weight: "bold"; color: "blue"; }
        </style>
        <div class={indexed_container}>
            <h3>"Indexed List"</h3>
            <ol>
                <li class={fruit_item}>
                    <span class={index}>"1. "</span>
                    "Apple"
                </li>
                <li class={fruit_item}>
                    <span class={index}>"2. "</span>
                    "Banana"
                </li>
                <li class={fruit_item}>
                    <span class={index}>"3. "</span>
                    "Cherry"
                </li>
            </ol>
        </div>
    }
}

/// Example: Complex data iteration
#[azumi::component]
pub fn complex_data_list() -> impl azumi::Component {
    html! {
        <style>
            .users_container { padding: "20px"; }
            .user_card { padding: "15px"; margin: "10px 0"; border: "1px solid #ddd"; }
            .active { background: "#e8f5e9"; }
            .inactive { background: "#ffebee"; }
        </style>
        <div class={users_container}>
            <h3>"User List"</h3>
            <div class="user_card active">
                <p><strong>"Alice"</strong> " - Age: 25 - Status: active"</p>
            </div>
            <div class="user_card inactive">
                <p><strong>"Bob"</strong> " - Age: 30 - Status: inactive"</p>
            </div>
            <div class="user_card active">
                <p><strong>"Charlie"</strong> " - Age: 22 - Status: active"</p>
            </div>
        </div>
    }
}

/// Main lesson component
#[azumi::component]
pub fn lesson9() -> impl azumi::Component {
    html! {
        <style>
            .lesson_container { padding: "20px"; }
            .lesson_title { font-size: "24px"; color: "#333"; }
            .examples { display: "grid"; gap: "20px"; margin-top: "20px"; }
        </style>
        <div class={lesson_container}>
            <h1 class={lesson_title}>"Lesson 9: Lists and Iteration"</h1>
            <p>"Learn how to render lists and iterate over data"</p>

            <div class={examples}>
                @simple_list()
                @indexed_list()
                @complex_data_list()
            </div>
        </div>
    }
}

// Handler for Axum
pub async fn lesson9_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson9()))
}
