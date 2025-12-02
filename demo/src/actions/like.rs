use azumi::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct LikeState {
    pub liked: bool,
    pub count: i32,
}

/// Outer shell - rendered once on page load, contains styles
pub fn like_section(state: LikeState) -> impl Component {
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

        // Render the dynamic core
        {like_button(state.clone())}
    }
}

/// Inner fragment - this is what gets swapped by actions
pub fn like_button(state: LikeState) -> impl Component {
    // Serialize state for az-scope
    let scope_json = serde_json::to_string(&state).unwrap_or_default();

    html! {
        <style>
            .btn {}
            #like_section {}
        </style>
        <div id={like_section} az-scope={scope_json}>
            <h2>"Server-Side Action"</h2>
            <p>
                "Likes: " <span az-bind:text="count">{state.count}</span>
            </p>

            <button
                class={btn}
                az-bind:class.liked="liked"
                az-on={click call toggle_like -> #like_section}
            >
                <span az-bind:text="if liked { 'Unlike' } else { 'Like' }">
                    {if state.liked { "Unlike" } else { "Like" }}
                </span>
            </button>
        </div>
    }
}

/// Server-side action - returns ONLY the inner fragment, not the outer shell
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

    // Return ONLY the dynamic fragment, styles are already on the page!
    like_button(new_state)
}
