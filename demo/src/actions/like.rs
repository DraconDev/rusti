use azumi::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct LikeState {
    pub liked: bool,
    pub count: i32,
}

/// Server-side action that toggles the like state
#[azumi::action]
pub async fn toggle_like(state: LikeState) -> impl Component {
    let new_state = LikeState {
        liked: !state.liked,
        count: if !state.liked {
            state.count + 1
        } else {
            state.count - 1
        },
    };

    // Serialize to JSON for az-scope attribute
    let scope_json = serde_json::to_string(&new_state).unwrap_or_default();

    // Return the updated HTML fragment
    html! {
        <style>
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
            #like_section {}
        </style>
        <div id={like_section} data-test={scope_json.as_str()}>
            <h2>"Server-Side Action"</h2>
            <p>
                "Likes: " <span az-bind:text="count">"10"</span>
            </p>

            <button
                class={btn}
                az-bind:class.liked="liked"
                az-on={click call toggle_like -> #like_section}
            >
                <span az-bind:text="if liked { 'Unlike' } else { 'Like' }">"Like"</span>
            </button>
        </div>
    }
}
