use crate::actions::{toggle_like, LikeState};
use azumi::prelude::*;

pub fn azumi_plus_demo() -> impl Component {
    html! {
    <style>.demo_container {
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
    .btn.liked {
        background: "#ff4081";
        color: "white";
    }
    #like_section {
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
                <button class={btn} az-on="click set count = count + 1">
                    "Increment"
                </button>
            </div>

            <hr />

            // Server Action
            <div id={like_section} az-scope="{ \"liked\": false, \"count\": 10 }">
                <h2>"Server-Side Action"</h2>
                <p>
                    "Likes: " <span az-bind:text="count">"10"</span>
                </p>

                // az-on with unified syntax
                // click call toggle_like -> #like-section
                // This calls the server action `toggle_like` with the current scope state,
                // and replaces the #like-section with the result.
                // Wait, the result of the action is HTML.
                // But our action returns `LikeState`.
                // We need the action to return HTML (Component) to replace the target.
                // Or we need the client to handle JSON response and update scope.

                // In the plan: "Server sends only necessary HTML fragments, client uses morphing"
                // So the action should return HTML.

                <button
                    class={btn}
                    az-bind:class.liked="liked"
                    az-on={click call toggle_like -> #like-section}
                >
                    <span az-bind:text="if liked { 'Unlike' } else { 'Like' }">"Like"</span>
                </button>
            </div>
        </div>
    }
}

pub async fn azumi_plus_demo_handler() -> impl axum::response::IntoResponse {
    azumi::render_to_string(&azumi_plus_demo())
}
