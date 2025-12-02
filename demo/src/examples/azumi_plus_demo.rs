use crate::actions::{like_section, LikeState};
use azumi::prelude::*;

pub fn azumi_plus_demo() -> impl Component {
    html! {
        <style>
        .demo_container {
            padding: "2rem";
            border: "1px solid #ccc";
            border-radius: "8px";
            max-width: "600px";
            margin: "0 auto";
        }
        .btn {
            padding: "0.5rem 1rem";
            border: "none";
            border-radius: "4px";
            cursor: "pointer";
            background: "#eee";
        }
        </style>
        <div class={demo_container}>
            <h1>"Azumi+ Demo"</h1>


            // Local State (Client-side only)
            <div az-scope="{ \"count\": 0 }">
                <h2>"Client-Side Counter"</h2>
                <p>
                    "Count: " <span az-bind:text="count">"0"</span>
                </p>
                <button class="btn" az-on="click set count = count + 1">
                    "Increment"
                </button>
            </div>

            <hr />

            // Server Action - using the refactored component
            {like_section(LikeState { liked: false, count: 10 })}
        </div>
    }
}

pub async fn azumi_plus_demo_handler() -> impl axum::response::IntoResponse {
    azumi::render_to_string(&azumi_plus_demo())
}
